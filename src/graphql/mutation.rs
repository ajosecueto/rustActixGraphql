use async_graphql::*;
use rdkafka::producer::FutureProducer;
use sqlx::PgPool;
use crate::events;
use crate::events::aggregation::AggregationEvent;
use crate::persistence::models::NewPreferenceLocaleEntity;
use crate::persistence::repository::Repository;

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_preference(&self, ctx: &Context<'_>, locales: Vec<PreferenceLocalesInput>) -> bool {
        // let pool = ctx.data::<PgPool>().expect("Error");
        let producer = ctx.data::<FutureProducer>().expect("Error");
        let locales: Vec<NewPreferenceLocaleEntity> = locales.into_iter().map(NewPreferenceLocaleEntity::from).collect();
        AggregationEvent::preference_event(locales, &producer).await;
        return true;
    }
    // Edit video url by locale_id
}


#[derive(InputObject, Debug)]
struct PreferenceLocalesInput {
    locale: String,
    description: String,
    video_url: Option<String>,
}

impl From<PreferenceLocalesInput> for NewPreferenceLocaleEntity {
    fn from(entity: PreferenceLocalesInput) -> Self {
        NewPreferenceLocaleEntity {
            description: entity.description,
            locale: entity.locale,
            video_url: entity.video_url,
        }
    }
}


