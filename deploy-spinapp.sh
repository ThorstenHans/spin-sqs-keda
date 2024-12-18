#! /bin/bash

set -euo pipefail

echo "🚀 Deploying SpinApp to your Kubernetes Cluster"

read -p "Please provide your AWS region: " AWS_REGION

if [[ -z "$AWS_REGION" ]]; then
    echo "AWS region cannot be empty"
    exit 1
fi

pushd manifests
kubectl create configmap aws --from-literal region=$AWS_REGION
kubectl apply -f deployment.yaml
popd

echo "✅ SpinApp deployed to your Kubernetes Cluster"