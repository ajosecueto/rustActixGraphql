use async_graphql::*;
use sqlx::PgPool;
use crate::persistence::models::NewPreferenceLocaleEntity;
use crate::persistence::repository::Repository;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_preference(&self, ctx: &Context<'_>, locales: Vec<PreferenceLocalesInput>) -> bool {
        let pool = ctx.data::<PgPool>().expect("Error");
        let preference = Repository::create_preference(&pool).await.expect("Cant save preference");
        Repository::create_locales(
            preference.preference_id,
            locales.iter().map(NewPreferenceLocaleEntity::from).collect(),
            &pool,
        ).await.expect("Cant save locales");
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

impl From<&PreferenceLocalesInput> for NewPreferenceLocaleEntity {
    fn from(entity: &PreferenceLocalesInput) -> Self {
        NewPreferenceLocaleEntity {
            description: entity.description.clone(),
            locale: entity.locale.clone(),
            video_url: entity.video_url.clone(),
        }
    }
}


