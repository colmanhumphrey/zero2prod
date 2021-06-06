use crate::routes::error_chain_fmt;
use actix_web::{web, HttpResponse, ResponseError, http::StatusCode};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(
    name = "Confirming a pending subscriber",
    skip(pool, parameters)
)]
pub async fn multi_confirm(
    pool: web::Data<PgPool>,
    parameters: web::Query<Parameters>,
) -> Result<HttpResponse, ConfirmError> {
    let id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .context("Unable do get token result from database")?;

    match id {
        None => Err(ConfirmError::UnknownToken),
        Some(subscriber_id) => {
            confirm_subscriber(&pool, subscriber_id)
                .await
                .context("Couldn't confirm subscriber")?;
            Ok(HttpResponse::Ok().finish())
        }
    }
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(pool, subscriber_id))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), ConfirmSubscriberError> {
    sqlx::query!(
        r#"
        UPDATE subscriptions
            SET status = 'confirmed'
            WHERE id = $1
        "#,
        subscriber_id,
    )
    .execute(pool)
    .await
    .map_err(ConfirmSubscriberError)?;

    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(pool, subscription_token))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, GetIdError> {
    let result = sqlx::query!(
        r#"
        SELECT subscriber_id
          FROM subscription_tokens
         WHERE subscription_token = $1
        "#,
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(GetIdError)?;

    Ok(result.map(|r| r.subscriber_id))
}

pub struct ConfirmSubscriberError(sqlx::Error);
pub struct GetIdError(sqlx::Error);

impl std::fmt::Debug for ConfirmSubscriberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
impl std::fmt::Debug for GetIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl std::fmt::Display for ConfirmSubscriberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
             trying to mark subscriber as confirmed"
        )
    }
}

impl std::fmt::Display for GetIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
             trying get the ID from a token"
        )
    }
}

impl std::error::Error for ConfirmSubscriberError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // cast &sqlx::Error to &dyn::Error
        Some(&self.0)
    }
}
impl std::error::Error for GetIdError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // cast &sqlx::Error to &dyn::Error
        Some(&self.0)
    }
}

#[tracing::instrument(name = "Confirming a pending subscriber", skip(pool, parameters))]
pub async fn confirm(
    pool: web::Data<PgPool>,
    parameters: web::Query<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    let subpool = pool.as_ref();
    let num_affected = sqlx::query!(
        r#"
        UPDATE subscriptions
            SET status = 'confirmed'
            FROM subscription_tokens
            WHERE subscriptions.id = subscription_tokens.subscriber_id
              AND subscription_tokens.subscription_token = $1
        "#,
        parameters.subscription_token
    )
    .execute(subpool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    println!("Num affected: {}", num_affected.rows_affected());

    Ok(HttpResponse::Ok().finish())
}

#[derive(thiserror::Error)]
pub enum ConfirmError {
    #[error("There is no subscriber associated with the provided token")]
    UnknownToken,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for ConfirmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for ConfirmError {
    fn status_code(&self) -> StatusCode {
        match self {
            ConfirmError::UnknownToken => StatusCode::UNAUTHORIZED,
            ConfirmError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
