extern crate capnp;

use std::error::Error;
use capnp::message::{Builder, HeapAllocator, TypedReader};
use crate::persistence::models::NewPreferenceLocaleEntity;


pub mod preference_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/preference_capnp.rs"));
}


pub struct SchemaBuilder;

impl SchemaBuilder {
    /// Create a message
    pub async fn build_preference(locales: Vec<NewPreferenceLocaleEntity>) -> Vec<u8> {
        let mut message = ::capnp::message::Builder::new_default();
        let preference_message = message.init_root::<preference_capnp::preference::Builder>();
        let mut new_locales = preference_message.init_locale(locales.len().try_into().unwrap());
        for (index, data) in locales.into_iter().enumerate() {
            let mut locale = new_locales.reborrow().get(index.try_into().unwrap());
            locale.set_locale(&data.locale);
            locale.set_description(&data.description);
            locale.set_video_url(&data.video_url.unwrap_or_default());
        }
        let mut buf_slice = vec![];
        capnp::serialize::write_message(&mut buf_slice, &message).unwrap();
        return buf_slice;
    }


    pub async fn read_preference(payload: &[u8]) ->  Result<Vec<NewPreferenceLocaleEntity>, Box<dyn Error>> {
        let mut request: Vec<NewPreferenceLocaleEntity> = Vec::new();
        let message_reader = capnp::serialize::read_message(payload, ::capnp::message::ReaderOptions::new())?;
        let preference_message = message_reader.get_root::<preference_capnp::preference::Reader>()?;
        for locale in preference_message.get_locale()?.iter() {
            request.push(
                NewPreferenceLocaleEntity {
                    locale: locale.get_locale()?.to_string(),
                    description: locale.get_description()?.to_string(),
                    video_url: Some(locale.get_video_url()?.to_string()),
                }
            );
        }

        Ok(
            request
        )

    }
}