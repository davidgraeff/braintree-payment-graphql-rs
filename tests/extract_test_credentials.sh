#!/bin/bash -e
# Github Actions has a secret variable for this repository called "SERVICE_ACCOUNT_JSON"
# that contains the base64 encoded credentials json
if [ "$(uname)" == "Darwin" ]; then
    echo "$SERVICE_ACCOUNT_JSON" | base64 -D > credentials.json
else
    echo "$SERVICE_ACCOUNT_JSON" | base64 -d > credentials.json
fi
# Print the first few characters
head -n 3 credentials.json
# Sanity check
[[ $(jq -r ".environment" credentials.json) == "Sandbox" ]] || { echo >&2 'Failed to extract credentials.json'; exit 1; }