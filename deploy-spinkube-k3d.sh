#! /bin/bash

set -euo pipefail

echo "🚀 Deploying SpinKube to your Kubernetes Cluster"

helm repo add jetstack https://charts.jetstack.io --force-update
helm install \
  cert-manager jetstack/cert-manager \
  --namespace cert-manager \
  --create-namespace \
  --version v1.16.2 \
  --set crds.enabled=true

kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.4.0/spin-operator.crds.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.4.0/spin-operator.runtime-class.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.4.0/spin-operator.shim-executor.yaml

helm install spin-operator \
  --namespace spin-operator \
  --create-namespace \
  --version 0.4.0 \
  --wait \
  oci://ghcr.io/spinkube/charts/spin-operator

kubectl apply -f https://raw.githubusercontent.com/spinkube/spin-operator/07a3175c25f47c722fb9510fc6999a9665cff289/config/samples/simple.yaml

echo "✅ SpinKube deployed to your Kubernetes Cluster"