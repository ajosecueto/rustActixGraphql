use async_graphql::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::persistence::models::PreferenceEntity;
use crate::persistence::repository::Repository;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_preferences(&self, ctx: &Context<'_>) -> Vec<Preference> {
        let pool = ctx.data::<PgPool>().expect("Error");
        Repository::get_preferences(&pool).await.expect("Can't obtain preferences")
            .iter()
            .map(Preference::from)
            .collect()
    }


    // get all preferences in a locale ej: es: Deportes, Baile, etc... en: Sport, Dance, etc

    // get by preference_id with their locales Ej: uuid: x , Deportes, Sports, Etc {Preference, locales: Locales}
}


#[derive(Serialize, Deserialize)]
struct Preference {
    preference_id: ID,
    created_at: DateTime<Utc>,
    status: bool,
}

#[Object]
impl Preference {
    async fn preference_id(&self) -> &ID {
        &self.preference_id
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn status(&self) -> &bool {
        &self.status
    }
}


impl From<&PreferenceEntity> for Preference {
    fn from(entity: &PreferenceEntity) -> Self {
        Preference {
            preference_id: entity.preference_id.into(),
            created_at: entity.created_at,
            status: entity.status,
        }
    }
}
