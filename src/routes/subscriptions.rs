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
    log::info!(
        "Adding '{}' '{}' as a new subscriber,",
        payload.email,
        payload.name
    );
    log::info!("Saving new subscriber details in the db");
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
        log::error!("Failed to execute query: {:?}", e);
        HttpResponse::InternalServerError().finish()
    })?;
    log::info!("New subscriber details have been saved");

    Ok(HttpResponse::Ok().finish())
}
