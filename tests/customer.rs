#[macro_use(unwrap_query)]
extern crate braintreepayment_graphql;
use braintreepayment_graphql::{Braintree, mutation_id, Credentials};
use failure::*;
use graphql_client::GraphQLQuery;

#[test]
fn customer_tests() -> Result<(), failure::Error> {
    let bt = Braintree::new(Credentials::from_file("credentials.json")?);

    let customer_id = create_customer(&bt)?;
    println!("Update customer");
    update_customer(&bt, &customer_id)?;
    println!("Query customer");
    query_customer(&bt, &customer_id)?;
    println!("Client Token");
    client_token(&bt, &customer_id)?;
    println!("Delete customer");
    delete_customer(&bt, &customer_id)?;

    Ok(())
}

fn create_customer(bt: &Braintree) -> Result<String, failure::Error> {
    use braintreepayment_graphql::queries::customer::create_customer::*;

    let customer = bt
        .perform(CreateCustomer {
            customer: CustomerInput {
                first_name: Some("first".to_owned()),
                last_name: Some("last".to_owned()),
                email: Some("test@abc.de".to_owned()),
                ..CustomerInput::new()
            },
        })?
        .create_customer
        .and_then(|r| r.customer)
        .ok_or(err_msg("customer"))?;

    println!("Received customer {:?}", customer);
    Ok(customer.id)
}

fn update_customer(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::update_customer::*;

    let _ = bt.perform(UpdateCustomer {
        cust_id: customer_id.to_owned(),
        customer: CustomerInput {
            first_name: Some("new".to_owned()),
            last_name: Some("name".to_owned()),
            email: Some("changed@abc.de".to_owned()),
            ..CustomerInput::new()
        },
    })?;
    Ok(())
}

fn query_customer(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::get_customer::*;

    let customer = unwrap_query!(bt
            .perform(
                GetCustomer {
                    cust_id: customer_id.to_owned(),
                },
            )? => GetCustomerNodeOn::Customer)?;

    assert_eq!(customer.first_name, Some("new".to_owned()));
    assert_eq!(customer.last_name, Some("name".to_owned()));
    assert_eq!(customer.email, Some("changed@abc.de".to_owned()));
    Ok(())
}

fn client_token(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::customer_client_token::*;

    let client_token = bt
        .perform(CustomerClientToken {
            cust_id: customer_id.to_owned(),
        })?
        .create_client_token
        .and_then(|f| f.client_token)
        .ok_or(err_msg("Token"))?;

    println!("{}", client_token);
    Ok(())
}

fn delete_customer(bt: &Braintree, customer_id: &str) -> Result<(), failure::Error> {
    use braintreepayment_graphql::queries::customer::delete_customer::*;

    let delete_mut_id_orig = mutation_id();

    let delete_mut_id = bt
        .perform(DeleteCustomer {
            cust_id: customer_id.to_owned(),
            client_mutation_id: Some(delete_mut_id_orig.to_owned()),
        })?
        .delete_customer
        .and_then(|f| f.client_mutation_id)
        .ok_or(err_msg("Token"))?;

    assert_eq!(delete_mut_id_orig, delete_mut_id);
    Ok(())
}
