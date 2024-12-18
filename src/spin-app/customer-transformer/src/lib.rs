use anyhow::Result;
use bindings::{
    fermyon::spin_sqs::sqs_types::{Message, MessageAction},
    Error, Guest,
};
use models::{InboundCustomerModel, OutboundCustomerModel};
use spin_sdk::{redis::Connection, variables};

mod bindings;
mod models;

struct Component;

const REDIS_CONNECTION_STRING_VAR: &str = "redis_connection_string";

impl Guest for Component {
    fn handle_queue_message(message: Message) -> Result<MessageAction, Error> {
        // prepare
        let Ok(redis_cs) = variables::get(REDIS_CONNECTION_STRING_VAR) else {
            println!("Redis Connection String not set.");
            return Ok(MessageAction::Leave);
        };
        // validate
        let Some(id) = message.id else {
            println!("Leaving message in SQS: No Message ID");
            return Ok(MessageAction::Leave);
        };
        let Some(body) = message.body else {
            println!("Leaving message in SQS: No Message Body");
            return Ok(MessageAction::Leave);
        };

        let Ok(customer) = serde_json::from_str::<InboundCustomerModel>(&body) else {
            println!("Leaving message in SQS: Invalid Message Body");
            return Ok(MessageAction::Leave);
        };

        // transform
        let transformed = transform(&customer);
        // load
        match Loader::with_target(redis_cs).load(id, transformed) {
            Ok(r) => Ok(r),
            Err(e) => Err(Error::Other(e.to_string())),
        }
    }
}

pub fn transform(customer: &InboundCustomerModel) -> OutboundCustomerModel {
    OutboundCustomerModel::from(customer)
}

pub struct Loader {
    connection_string: String,
}

impl Loader {
    pub fn with_target(connection_string: String) -> Self {
        Self { connection_string }
    }

    pub fn load(&self, id: String, outbound_model: OutboundCustomerModel) -> Result<MessageAction> {
        let connection = Connection::open(&self.connection_string)?;
        let payload = serde_json::to_vec(&outbound_model)?;
        connection.set(id.as_str(), &payload)?;
        println!(
            "Loaded {} with Id ({}) to Redis Channel",
            outbound_model.full_name, id
        );
        Ok(MessageAction::Delete)
    }
}

bindings::export!(Component with_types_in bindings);
