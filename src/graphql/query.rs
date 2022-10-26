use std::sync::Arc;

use crate::persistence::models::PreferenceEntity;
use crate::persistence::repository::Repository;
use async_graphql::*;
use chrono::{DateTime, Utc};
use scylla::{IntoTypedRows, Session};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub struct Query;

#[Object]
impl Query {
    async fn get_preferences(&self, ctx: &Context<'_>) -> Vec<Preference> {
        let session = ctx.data::<Arc<Session>>().expect("Error");
        if let Some(rows) = session
            .query(
                "SELECT comment_id, description, post_id FROM comments.comments",
                &[],
            )
            .await
            .expect("Can get rows")
            .rows
        {
            for row in rows.into_typed::<(Uuid, String, Uuid)>() {
                let (a, b, c) = row.unwrap();
                println!("a, b, c: {}, {}, {}", a, b, c);
            }
        }
        let pool = ctx.data::<PgPool>().expect("Error");
        Repository::get_preferences(&pool)
            .await
            .expect("Can't obtain preferences")
            .into_iter()
            .map(Preference::from)
            .collect()
    }

    async fn get_preference(&self, ctx: &Context<'_>, id: ID) -> Option<Preference> {
        find_preference_by_id_internal(ctx, id).await
    }

    #[graphql(entity)]
    async fn find_preference_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<Preference> {
        find_preference_by_id_internal(ctx, id).await
    }

    // get all preferences in a locale ej: es: Deportes, Baile, etc... en: Sport, Dance, etc

    // get by preference_id with their locales Ej: uuid: x , Deportes, Sports, Etc {Preference, locales: Locales}
}

async fn find_preference_by_id_internal(ctx: &Context<'_>, id: ID) -> Option<Preference> {
    let pool = ctx.data::<PgPool>().expect("Error");
    if let Ok(pref) = Repository::get_preference(Uuid::parse_str(id.to_string().as_str()).unwrap(), &pool).await {
        return Some(Preference::from(pref));
    }
    None
}

#[derive(Serialize, Deserialize)]
struct Preference {
    id: ID,
    created_at: DateTime<Utc>,
    status: bool,
}

#[Object]
impl Preference {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    async fn status(&self) -> &bool {
        &self.status
    }
}

impl From<PreferenceEntity> for Preference {
    fn from(entity: PreferenceEntity) -> Self {
        Preference {
            id: entity.preference_id.into(),
            created_at: entity.created_at,
            status: entity.status,
        }
    }
}
