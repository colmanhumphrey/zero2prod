use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String
}

#[tracing::instrument(
    name = "Confirming a pending subscriber",
    skip(_parameters)
)]
pub async fn confirm(
    _parameters: web::Query<Parameters>,
) -> Result<HttpResponse, HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}
