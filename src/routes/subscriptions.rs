use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SubscribeRequest {
    email: String,
    name: String,
}

pub async fn subscribe(
    payload: web::Form<SubscribeRequest>,
    connection: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber,",
        request_id,
        payload.email,
        payload.name
    );
    log::info!(
        "request_id {} - Saving new subscriber details in the db",
        request_id
    );
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        payload.email,
        payload.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    .map_err(|e| {
        log::error!(
            "request_id {} - Failed to execute query: {:?}",
            request_id,
            e
        );
        HttpResponse::InternalServerError().finish()
    })?;
    log::info!(
        "request_id {} - New subscriber details have been saved",
        request_id
    );

    Ok(HttpResponse::Ok().finish())
}
