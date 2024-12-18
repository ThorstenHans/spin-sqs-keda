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

helm repo add kwasm http://kwasm.sh/kwasm-operator/ --force-update

helm install \
  kwasm-operator kwasm/kwasm-operator \
  --namespace kwasm \
  --create-namespace \
  --set kwasmOperator.installerImage=ghcr.io/spinkube/containerd-shim-spin/node-installer:v0.17.0

kubectl annotate node --all kwasm.sh/kwasm-node=true

kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.4.0/spin-operator.crds.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.4.0/spin-operator.runtime-class.yaml
kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.4.0/spin-operator.shim-executor.yaml

helm install spin-operator \
  --namespace spin-operator \
  --create-namespace \
  --version 0.4.0 \
  --wait \
  oci://ghcr.io/spinkube/charts/spin-operator

echo "✅ SpinKube deployed to your Kubernetes Cluster"