use std::error::Error;
use std::rc::Rc;
use capnp::serialize;
use sqlx::PgPool;
use rdkafka::{Message};
use crate::persistence::models::NewPreferenceLocaleEntity;
use crate::persistence::repository::Repository;
use crate::events::schema::preference_capnp::{preference, locale};
use futures::{Stream, StreamExt, TryStreamExt};
use rdkafka::message::{BorrowedMessage, OwnedMessage};
use log::info;
use crate::events::schema::SchemaBuilder;


pub struct Reducer {
    pub db: Rc<PgPool>,
}

impl Reducer {
    async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
        // Simulate some work that must be done in the same order as messages are
        // received; i.e., before truly parallel processing can begin.
        println!("Message received: {:?}", msg.offset());
    }

    async fn record_owned_message_receipt(&self, _msg: &OwnedMessage) {
        // Like `record_borrowed_message_receipt`, but takes an `OwnedMessage`
        // instead, as in a real-world use case  an `OwnedMessage` might be more
        // convenient than a `BorrowedMessage`.
        match _msg.topic() {
            "kafka" => {
                let payload = _msg.payload().expect("Kafka message should contain payload");
                let _ = self.create_preference(payload).await;
            }
            _ => {}
        }
    }

    pub async fn start_consumer(&self) -> () {
        let consumer = crate::infrastructure::kafka::create_consumer("kafka".to_string());

        let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
            println!("Message received: {}", borrowed_message.offset());
            async move {
                // Process each message
                Reducer::record_borrowed_message_receipt(&borrowed_message).await;
                // Borrowed messages can't outlive the consumer they are received from, so they need to
                // be owned in order to be sent to a separate thread.
                let owned_message = borrowed_message.detach();
                self.record_owned_message_receipt(&owned_message).await;
                Ok(())
            }
        });

        println!("Starting event loop");
        stream_processor.await.expect("stream processing failed");
        println!("Stream processing terminated");
    }


    pub async fn create_preference(&self, payload: &[u8]) -> Result<(), Box<dyn Error>> {
        let request = SchemaBuilder::read_preference(&payload).await?;
        let preference = Repository::create_preference(&self.db).await.expect("Cant save preference");
        Repository::create_locales(
            preference.preference_id,
            request,
            &self.db,
        ).await.expect("Cant save locales");
        Ok(())
    }
}