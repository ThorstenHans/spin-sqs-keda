#! /bin/bash

set -euo pipefail

echo "🚀 Deploying KEDA AWS SQS Scaler to your Kubernetes Cluster"

read -p "Please provide the SQS queue URL: " SQS_QUEUE_URL

if [[ -z "$SQS_QUEUE_URL" ]]; then
    echo "AWS SQS queue URL cannot be empty"
    exit 1
fi


read -p "Please provide your AWS region: " AWS_REGION

if [[ -z "$AWS_REGION" ]]; then
    echo "AWS region cannot be empty"
    exit 1
fi


pushd manifests
sed -e "s|SQS_QUEUE_URL|${SQS_QUEUE_URL}|g" \
  -e "s|AWS_REGION|${AWS_REGION}|g" ./scaler.tmpl.yaml > scaler.yaml
kubectl apply -f scaler.yaml
popd
echo "✅ KEDA AWS SQS Scaler deployed to your Kubernetes Cluster"