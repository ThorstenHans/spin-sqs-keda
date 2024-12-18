#! /bin/bash

set -euo pipefail

echo "🚀 Deploying KEDA AWS SQS Scaler to your Kubernetes Cluster"

read -p "Please provide the SQS queue URL: " SQS_QUEUE_URL

if [[ -z "$SQS_QUEUE_URL" ]]; then
    echo "AWS SQS queue URL cannot be empty"
    exit 1
fi

if [[ -z "${AWS_ACCESS_KEY_ID:-}" ]]; then
    read -p "Please provide the AWS Access Key ID: " AWS_ACCESS_KEY_ID
    if [[ -z "$AWS_ACCESS_KEY_ID" ]]; then
        echo "AWS Access Key ID cannot be empty"
        exit 1
    fi
fi

if [[ -z "${AWS_SECRET_ACCESS_KEY:-}" ]]; then
    read -p "Please provide the AWS Secret Access Key: " AWS_SECRET_ACCESS_KEY
    if [[ -z "$AWS_SECRET_ACCESS_KEY" ]]; then
        echo "AWS Secret Access Key cannot be empty"
        exit 1
    fi
fi

if [[ -z "${AWS_SESSION_TOKEN:-}" ]]; then
    read -p "Please provide the AWS Session Token: " AWS_SESSION_TOKEN
    if [[ -z "$AWS_SESSION_TOKEN" ]]; then
        echo "AWS Session Token cannot be empty"
        exit 1
    fi
fi

read -p "Please provide your AWS region: " AWS_REGION

if [[ -z "$AWS_REGION" ]]; then
    echo "AWS region cannot be empty"
    exit 1
fi

kubectl create secret generic aws-credentials \
  --from-literal=AWS_ACCESS_KEY_ID="${AWS_ACCESS_KEY_ID}" \
  --from-literal=AWS_SECRET_ACCESS_KEY="${AWS_SECRET_ACCESS_KEY}" \
  --from-literal=AWS_SESSION_TOKEN="${AWS_SESSION_TOKEN}"

pushd manifests
sed -e "s|SQS_QUEUE_URL|${SQS_QUEUE_URL}|g" \
  -e "s|AWS_REGION|${AWS_REGION} ./scaler.tmpl.yaml > scaler.yaml
kubectl apply -f scaler.yaml
popd
echo "✅ KEDA AWS SQS Scaler deployed to your Kubernetes Cluster"