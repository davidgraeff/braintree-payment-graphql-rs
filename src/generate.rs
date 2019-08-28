use failure::*;
use graphql_client_codegen::{generate_module_token_stream, CodegenMode, GraphQLClientCodegenOptions};
use quote::*;
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::Token;

#[cfg(feature = "rustfmt")]
fn format(codes: &str, filename: &str) -> String {
    use rustfmt_nightly::{Config, Input, Session};

    let mut config = Config::default();

    config.set().emit_mode(rustfmt_nightly::EmitMode::Stdout);
    config.set().verbose(rustfmt_nightly::Verbosity::Quiet);

    let mut out = Vec::with_capacity(codes.len() * 2);

    let r = Session::new(config, Some(&mut out)).format(Input::Text(codes.to_owned()));
    match r {
        Err(_) => {
            format_err!("rustfmt error on {}", filename);
            codes.to_owned()
        }
        _ => match out.len() {
            0 => codes.to_owned(),
            _ => String::from_utf8(out).unwrap(),
        },
    }
}

#[cfg(not(feature = "rustfmt"))]
fn format(codes: &str, _: &str) -> String {
    return codes.to_owned();
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> Result<(), failure::Error>) -> Result<(), failure::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}

fn create_options() -> GraphQLClientCodegenOptions {
    use graphql_client_codegen::deprecation::DeprecationStrategy;

    let mut options = GraphQLClientCodegenOptions::new(CodegenMode::Cli);
    options.set_module_visibility(
        syn::VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into(),
    );

    options.set_input_derives("Serialize,PartialEq".to_owned());
    options.set_response_derives("Serialize,PartialEq,Debug".to_owned());
    options.set_deprecation_strategy(DeprecationStrategy::Warn);

    options
}

pub fn generate(query_path: PathBuf, output_directory: PathBuf) -> Result<(), failure::Error> {
    let schema_data = include_str!("../schema.graphql");
    let mut file = tempfile::Builder::new()
        .prefix("schema")
        .suffix(".graphql")
        .rand_bytes(5)
        .tempfile()?;
    file.write_all(schema_data.as_bytes())?;
    file.flush()?;

    visit_dirs(&query_path, &move |query_file: &std::fs::DirEntry| {
        let gen = generate_module_token_stream(query_file.path().clone(), file.path(), create_options())?;

        let gen = match env!("CARGO_PKG_NAME") == "braintreepayment_graphql" {
            true => quote!(
                #![allow(non_camel_case_types)]
                #[allow(unused_imports)]
                use crate::queries::{Amount,CurrencyCodeAlpha,CountryCodeAlpha3,Timestamp,CustomFieldName};
                #gen
            ),
            false => quote!(
                #![allow(non_camel_case_types)]
                #[allow(unused_imports)]
                use braintreepayment_graphql::queries::{Amount,CurrencyCodeAlpha,CountryCodeAlpha3,Timestamp,CustomFieldName};
                #gen
            ),
        };

        let generated_code = gen.to_string();
        let generated_code = format(&generated_code, query_file.path().to_str().unwrap());

        let query_file_name: std::ffi::OsString = query_file.file_name().to_owned();

        let dest_file_path: PathBuf = output_directory.join(query_file_name).with_extension("rs");
        println!("Generated file {}", dest_file_path.to_str().as_ref().unwrap());

        let mut file = File::create(dest_file_path)?;
        write!(file, "{}", generated_code)?;
        Ok(())
    })?;
    Ok(())
}
