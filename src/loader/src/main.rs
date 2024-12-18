use anyhow::{bail, Result};
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_sqs::{
    operation::send_message_batch::SendMessageBatchOutput, types::SendMessageBatchRequestEntry,
};
use clap::Parser;
use cli::Cli;
use models::Customer;
use paginate::Pages;
use uuid::Uuid;

mod cli;
mod models;

const PAGE_SIZE: usize = 10;

#[::tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let message_count = args.message_count;
    let queue_url = args.queue_url;
    println!("AWS SQS Loader");
    println!("--------------\n");
    println!("Generating {} messages for {}", message_count, queue_url);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    println!(
        "AWS SQS allows sending batches with max {} items.\n",
        PAGE_SIZE
    );
    let pages = Pages::new(message_count.into(), PAGE_SIZE);
    println!("💡 Will create and send {} batches", pages.page_count());
    for page in pages.into_iter() {
        match send_batch(page.length, queue_url.as_str(), &config).await {
            Ok(_) => println!("   Batch sent!"),
            Err(e) => bail!(e),
        }
    }
    println!("✅ Sent a total of {}", message_count);
    Ok(())
}

async fn send_batch(
    count: usize,
    queue_url: &str,
    config: &SdkConfig,
) -> Result<SendMessageBatchOutput> {
    let client = aws_sdk_sqs::Client::new(&config);
    let entries = Customer::new_random(count)
        .into_iter()
        .map(|c| {
            SendMessageBatchRequestEntry::builder()
                .id(Uuid::new_v4().to_string())
                .message_body(serde_json::to_string(&c).unwrap())
                .build()
                .unwrap()
        })
        .collect::<Vec<SendMessageBatchRequestEntry>>();
    client
        .send_message_batch()
        .set_queue_url(Some(queue_url.to_string()))
        .set_entries(Some(entries))
        .send()
        .await
        .map_err(anyhow::Error::from)
}
