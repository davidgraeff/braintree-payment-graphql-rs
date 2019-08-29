#!/bin/sh
curl https://raw.githubusercontent.com/braintree/graphql-api/master/schema.graphql -o schema.graphql && \
cargo run --bin braintree-queries