use actix_web::{web, HttpResponse};
use sqlx::PgConnection;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"INSERT INTO subscriptions (id, email ,name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await;
    HttpResponse::Ok().finish()
}
