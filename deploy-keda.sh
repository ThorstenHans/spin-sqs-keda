#! /bin/bash

set -euo pipefail

echo "🚀 Deploying KEDA to your Kubernetes Cluster"

helm repo add kedacore https://kedacore.github.io/charts --force-update
helm install keda kedacore/keda --namespace keda --create-namespace

echo "✅ KEDA deployed to your Kubernetes Cluster"