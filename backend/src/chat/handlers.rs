use std::time::Instant;

use actix::prelude::*;
use actix_session::Session;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
pub use sqlx::PgPool;

use super::chat_server::ChatServer;
use super::chat_session::WsChatSession;

#[get("/ws")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    session: Session,
    pool: web::Data<PgPool>,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    let user_id = match session.get::<i32>("user_id")? {
        Some(id) => id,
        _ => return Err(actix_web::error::ErrorInternalServerError("No Auth")),
    };

    let result = sqlx::query!("SELECT username FROM users WHERE id=$1", user_id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;

    let username = result.username;
    println!("chat route connect! ");
    let to_name = req
        .query_string()
        .split('=')
        .collect::<Vec<&str>>()
        .last()
        .unwrap_or(&"None")
        .to_string();
    println!("connecting to {:#?} !", to_name);

    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: Some(username),
            to_name: Some(to_name),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
