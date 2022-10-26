use serde::de::Unexpected::Str;
use sqlx::PgPool;
use uuid::Uuid;
use crate::persistence::models::{NewPreferenceLocaleEntity, PreferenceEntity};

pub struct Repository;

impl Repository {
    pub async fn create_preference(pool: &PgPool) -> Result<PreferenceEntity, sqlx::Error> {
        let preference: PreferenceEntity = sqlx::query_as!(PreferenceEntity, r#"
        INSERT INTO
            preferences (status)
        VALUES
            (TRUE)
        RETURNING *
        "#).fetch_one(pool).await?;
        Ok(preference)
    }

    pub async fn get_preference(preference_id: Uuid, pool: &PgPool) -> Result<PreferenceEntity, sqlx::Error> {
        let values: PreferenceEntity = sqlx::query_as!(PreferenceEntity, r#"
        SELECT
        *
        FROM
        preferences
        "#).fetch_one(pool).await?;

        Ok(
                values
        )
    }

    pub async fn get_preferences(pool: &PgPool) -> Result<Vec<PreferenceEntity>, sqlx::Error> {
        let values: Vec<PreferenceEntity> = sqlx::query_as!(PreferenceEntity, r#"
        SELECT
            *
        FROM
            preferences
        "#).fetch_all(pool).await?;

        Ok(
            values
        )
    }

    pub async fn create_locales(preference_id: Uuid, locales: Vec<NewPreferenceLocaleEntity>, pool: &PgPool) -> Result<(), sqlx::Error> {
        let mut locales_vec: Vec<String> = Vec::new();
        let mut descriptions: Vec<String> = Vec::new();
        let mut video_urls: Vec<Option<String>> = Vec::new();
        let mut preferences_ids: Vec<Uuid> = Vec::new();

        for locale in locales {
            locales_vec.push(locale.locale);
            descriptions.push(locale.description);
            video_urls.push(locale.video_url);
            preferences_ids.push(preference_id);
        }

        sqlx::query!(r#"
        INSERT INTO
            preference_locales (locale, description, video_url, preference_id)
        VALUES
            (UNNEST($1::TEXT[]), UNNEST($2::TEXT[]), UNNEST($3::TEXT[]), UNNEST($4::UUID[]))
        "#,
            &locales_vec,
            &descriptions,
            video_urls as _,
            &preferences_ids
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn get_locales(preference_id: &[Uuid], locale: String, pool: &PgPool) -> Result<Vec<PreferenceEntity>, sqlx::Error> {
        todo!()
    }

    pub async fn update_url(locale_id: Uuid, pool: &PgPool) -> Result<PreferenceEntity, sqlx::Error> {
        todo!()
    }
}
