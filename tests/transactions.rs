use braintreepayment_graphql::{braintree_error, Braintree, Credentials, mutation_id, queries::transaction_helpers::*};
use failure::*;

#[test]
fn payment_charge_fail() -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::transactions::charge_payment_method::*;
    use rust_decimal_macros::*;

    let bt = Braintree::new(Credentials::from_file("credentials.json")?);

    let payment_id = "the_id";

    let response = bt.perform(ChargePaymentMethod {
        payment_method_id: payment_id.to_owned(),
        transaction: TransactionInput {
            order_id: Some("demo_id".to_owned()),
            purchase_order_number: Some("demo_id".to_owned()),
            ..TransactionInput::new(dec!(12.12))
        },
        client_mutation_id: None,
    });

    let error = braintree_error(response.err().as_ref());
    if !error.is_some() {
        bail!("Expected error");
    }
    let error = error.unwrap();
    assert_eq!(error.error_class, "VALIDATION".to_owned());
    assert_eq!(
        error.path,
        vec![
            "chargePaymentMethod".to_owned(),
            "input".to_owned(),
            "paymentMethodId".to_owned()
        ]
    );
    assert_eq!(
        error.message,
        "Unknown or expired single-use payment method.".to_owned()
    );
    Ok(())
}

#[test]
fn transactions() -> Result<(), failure::Error> {
    use rust_decimal_macros::*;
    let bt = Braintree::new(Credentials::from_file("credentials.json")?);

    let pm_paypal = "fake-paypal-billing-agreement-nonce";
    let pm_paypay_onetime = "fake-paypal-one-time-nonce";

    let pm_creditcard = "fake-valid-nonce";
    let pm_creditcard_no_billing_addr = "fake-valid-no-billing-address-nonce";

    let pm_creditcard_processor_declined = "fake-processor-declined-visa-nonce";

    let pm_method_id_already_used = "fake-consumed-nonce";

    let pm_android = "fake-android-pay-nonce";

    let pm_local = "fake-local-payment-nonce";

    let _creditcard_rejected_fraud = "4000111111111511";
    let _creditcard_unsuccessful = "4000111111111115";
    let _creditcard_valid = "4111111111111111";

    let amount_settled = dec!(0.01);
    let amount_declined = dec!(2000.51);
    let _amount_pending_paypal = dec!(4002.00);
    let _amount_gateway_reject_incomplete = dec!(5001.00);

    // Charge payment tests
    payment(&bt, pm_paypal, amount_settled, false, Some("pm_paypal".to_owned()))?;
    payment(&bt, pm_paypay_onetime, amount_settled, false, Some("pm_paypay_onetime".to_owned()))?;
    payment(&bt, pm_creditcard, amount_settled, false, Some("pm_creditcard".to_owned()))?;
    payment(&bt, pm_local, amount_settled, false, Some("pm_local".to_owned()))?;
    //payment(&bt, pm_android, amount_settled, false)?;

    let unique_order_id = mutation_id();
    let created_transaction = payment(&bt, pm_creditcard_no_billing_addr, amount_settled, false, Some(unique_order_id.clone()))?;

    assert_eq!(created_transaction.order_id, Some(unique_order_id));

    let r = get_transaction(&bt, &created_transaction.id)?;
    assert_eq!(r.unwrap().amount.unwrap().value.unwrap(), amount_settled);

    let r = search_transaction(&bt, &created_transaction.order_id.unwrap())?;
    assert!(r.len()>=1);
    assert_eq!(r.get(0).unwrap().amount.as_ref().unwrap().value.unwrap(), amount_settled);

    // Erroneous charging: Declined
    let r = payment(&bt, pm_local, amount_declined, false,Some("pm_local_amount_declined".to_owned()))?;

    use braintreepayment_graphql::queries::transactions::get_transaction::TransactionStatus;
    let r = get_transaction(&bt, &r.id)?;
    assert_eq!(r.unwrap().status.unwrap(), TransactionStatus::AUTHORIZING);

    // Erroneous charging: Declined
    let r = payment(&bt, pm_creditcard_processor_declined, amount_settled, false,Some("pm_creditcard_processor_declined".to_owned()))?;
    let r = get_transaction(&bt, &r.id)?;
    assert_eq!(r.unwrap().status.unwrap(), TransactionStatus::SUBMITTED_FOR_SETTLEMENT);


    // Erroneous charging: Payment method already used
    let error = payment(&bt, pm_method_id_already_used, amount_settled, false,Some("demo_id".to_owned())).expect_err("");
    let error = braintree_error(Some(&error)).ok_or(err_msg(err_msg("Expected pm_method_id_already_used")))?;
    assert_eq!(error.path,vec!["chargePaymentMethod", "input", "transaction", "paymentMethodNonce"]);
    assert_eq!(error.error_class, "VALIDATION".to_owned());

    // Vault payment method test
    let vaulted_transaction= vault(&bt, pm_creditcard)?;

    // The sandbox environment does not support verification
    let r = verify(&bt, &vaulted_transaction.id)?;
    use braintreepayment_graphql::queries::transactions::verify_payment_method::VerificationStatus;
    //assert_eq!(r.amount.unwrap().value.unwrap(), amount_settled);
    //assert_eq!(r.processor_response.unwrap(),"");
    assert_eq!(r.status.unwrap(), VerificationStatus::GATEWAY_REJECTED);

    delete_transaction(&bt, &vaulted_transaction.id)?;

    let vaulted_transaction=vault(&bt, pm_paypal)?;
    payment(&bt, &vaulted_transaction.id, amount_settled, true,Some("recurring".to_owned()))?;
    payment(&bt, &vaulted_transaction.id, amount_settled, true,Some("recurring".to_owned()))?;
    delete_transaction(&bt, &vaulted_transaction.id)?;

    // Erroneous vaulting
    let error = vault(&bt, pm_paypay_onetime).expect_err("");
    let error = braintree_error(Some(&error)).ok_or(err_msg("Expected vaulting error"))?;
    assert_eq!(error.error_class, "VALIDATION".to_owned());
    assert_eq!(error.path,vec!["vaultPaymentMethod", "input", "paymentMethodId"]);

    Ok(())
}
