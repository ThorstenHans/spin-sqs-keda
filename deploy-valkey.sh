#! /bin/bash

set -euo pipefail

echo "🚀 Deploying Valkey to your Kubernetes Cluster"

# Deploy Valkey
helm install valkey --namespace valkey --create-namespace oci://registry-1.docker.io/bitnamicharts/valkey
# Grab the Valkey Password
export VALKEY_PASSWORD=$(kubectl get secret --namespace valkey valkey -o jsonpath="{.data.valkey-password}" | base64 -d)
# Create a Secret in the default namespace with the valkey url
kubectl create secret generic valkey --from-literal=valkey-url="redis://:${VALKEY_PASSWORD}@valkey-primary.valkey.svc.cluster.local:6379"
# Create a ConfigMap in the default namespace with non-sensitive config data for valkey
kubectl create configmap valkey --from-literal host=valkey-primary.valkey.svc.cluster.local --from-literal port=6379

echo "✅ Valkey deployed to your Kubernetes Cluster"