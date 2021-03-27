use actix_web::{HttpResponse, web};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String
}

#[tracing::instrument(
    name = "Confirming a pending subscriber",
    skip(pool, parameters)
)]
pub async fn multi_confirm(
    pool: web::Data<PgPool>,
    parameters: web::Query<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    let id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    match id {
        None => Err(HttpResponse::Unauthorized().finish()),
        Some(subscriber_id) => {
            confirm_subscriber(&pool, subscriber_id)
                .await
                .map_err(|_| HttpResponse::InternalServerError().finish())?;
            Ok(HttpResponse::Ok().finish())
        }
    }
}

#[tracing::instrument(
    name = "Mark subscriber as confirmed",
    skip(pool, subscriber_id)
)]
pub async fn confirm_subscriber(
    pool: &PgPool,
    subscriber_id: Uuid,
) -> Result<(), sqlx::Error> {
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
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;

    Ok(())
}

#[tracing::instrument(
    name = "Get subscriber_id from token",
    skip(pool, subscription_token)
)]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
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
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(result.map(|r| r.subscriber_id))
}

#[tracing::instrument(
    name = "Confirming a pending subscriber",
    skip(pool, parameters)
)]
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
