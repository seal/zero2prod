use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

use crate::domain::{NewSubscriber, SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        //request_id = %Uuid::new_v4(), different from the actix one 
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_subscriber = NewSubscriber {
        email: form.0.email,
        name: SubscriberName::parse(form.0.name),
    };

    //let subscriber_name = crate::domain::SubscriberName(form.name.clone());
    //if !is_valid_name(&form.name) {
    //return HttpResponse::BadRequest().finish();
    //}
    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("req id Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
pub fn is_valid_name(s: &str) -> bool {
    let is_empty_or_whitespace = s.trim().is_empty();
    //  length count where unicode characters are two
    let is_too_long = s.graphemes(true).count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden = s.chars().any(|g| forbidden_characters.contains(&g));
    !(is_empty_or_whitespace || is_too_long || contains_forbidden)
}
#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
            "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.inner_ref(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("failed to execute query {:?}", e);
        e
    })?;
    Ok(())
}
