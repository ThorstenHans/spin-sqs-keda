#! /bin/bash
OCI_REF=ttl.sh/customer-transformer:24h
set -euo pipefail

echo "🚀 Building and Pushing your Spin App"

read -p "Please provide the SQS queue URL: " SQS_QUEUE_URL

if [[ -z "$SQS_QUEUE_URL" ]]; then
    echo "AWS SQS queue URL cannot be empty"
    exit 1
fi
pushd src
pushd spin-app
sed "s|SQS_QUEUE_URL|${SQS_QUEUE_URL}|g" ./spin.tmpl.toml > ./spin.toml
spin registry push --build $OCI_REF
popd
popd


echo "✅ SpinApp build and pushed to $OCI_REF"