use crate::infrastructure::kafka::send_message;
use crate::persistence::models::NewPreferenceLocaleEntity;
use capnp_futures::serialize;
use rdkafka::producer::FutureProducer;
use crate::events::schema::preference_capnp::{preference, locale};
use crate::events::schema::SchemaBuilder;


pub struct AggregationEvent;




impl AggregationEvent {

    pub async fn preference_event(locales: Vec<NewPreferenceLocaleEntity>, producer: &FutureProducer) {
        // create a message
        let preference = SchemaBuilder::build_preference(locales).await;
        send_message(&producer, &preference).await;
    }
}