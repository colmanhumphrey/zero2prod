use std::convert::TryInto;
use crate::domain::{NewSubscriber, SubscriberEmail, SubscriberName};
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use web::Form;

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    email: String,
    name: String,
}

impl TryInto<NewSubscriber>  for SubscribeRequest {
    type Error = String;

    fn try_into(self) -> Result<NewSubscriber, Self::Error> {
        let name = SubscriberName::parse(self.name)?;
        let email = SubscriberEmail::parse(self.email)?;
        Ok(NewSubscriber { email, name })
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(payload, pool),
    fields(
        email = %payload.email,
        name = %payload.name
    )
)]
pub async fn subscribe(
    payload: Form<SubscribeRequest>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let new_subscriber = payload.0.try_into()
        .map_err(|_| HttpResponse::BadRequest().finish())?;

    insert_subscriber(&pool, &new_subscriber)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
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
        new_subscriber.email(),
        new_subscriber.name(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
