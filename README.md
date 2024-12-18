# Spin SQS + KEDA Sample Application

This repository contains a sample illustrating how to build an ETL application with [Spin](https://github.com/fermyon/spin), Amazon SQS and KEDA.

## Prerequisites

To run the sample, you must have the following:

- `AWS_*` environment variables set for authentication
- An AWS SQS queue provisioned

### Local Execution

To run the app locally, you additionally need:

- A `valkey` or `redis` container running locally, as the Spin app sends transformed data to a Redis Channel. (The Spin app requires certain information about Valkey/Redis)

## Deploying cluster-wide Services

### SpinKube

You can find two different scripts for deploying [SpinKube](https://spinkube.dev) to your Kubernetes cluster.

#### SpinKube on k3d

If you have `k3d` installed on your system, you could leverage the SpinKube images which come with `containerd-shim-spin` already pre-provisioned:

```bash
k3d cluster create wasm-cluster \
  --image ghcr.io/spinkube/containerd-shim-spin/k3d:v0.17.0 \
  --port "8081:80@loadbalancer" \
  --agents 2
```

In that situation, you must use the `./deploy-spinkube-k3d.sh` script for deploying [cert-manager](https://cert-manager.io) and the Spin Operator.

### SpinKube on different Kubernetes distros

For all other Kubernetes distributions, you can use the `./deploy-spinkube.sh` script to deploy SpinKube and all its dependencies.

### KEDA

Use the `./deploy-keda.sh` script to deploy KEDA to your Kubernetes cluster.

### Valkey (Redis fork)

Use the `./deploy-valkey.sh` script to deploy Valkey to your Kubernetes cluster.


## Application Distribution and Deployment

First, you need to build and package the application, use the `./build-and-push-spinapp.sh` script to do so.

Next, you must deploy the app itself, this is done by executing `./deploy-spinapp.sh`.

Finally, you deploy the KEDA related Kubernetes manifests (`ScaledObject`, ...). Use the `./deploy-scaler.sh` script to do so.

## Load Generating

In [`./src/loader`](./src/loader/), you can find a simple application for generating load (sending messages to your SQS queue).

Run the application as shown here:

```bash
pushd src/loader
export QUEUE_URL="<YOUR_QUEUE_URL>"

cargo run --message-count 1000 --queue-url $QUEUE_URL

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/loader --message-count 1000 --queue-url ***`
AWS SQS Loader
--------------

Generating 1000 messages for ***
AWS SQS allows sending batches with max 10 items.

💡 Will create and send 100 batches
   Batch sent!
   
   ...

   Batch sent!
✅ Sent a total of 1000 messages

popd
```