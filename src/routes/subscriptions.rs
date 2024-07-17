use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let req_id = Uuid::new_v4();
    log::info!("req '{}' : Adding '{}' '{}' to the list of subscribers.", req_id, form.email, form.name);
    log::info!("Saving new subscriber to the DB");
    match sqlx::query!(
        r#"INSERT INTO subscriptions (id, email ,name, subscribed_at) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log:info!("req '{}' : New subscriber details save", req_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("req '{}' : Failed to execute query: {:?}", req_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
