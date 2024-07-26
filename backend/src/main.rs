use actix::*;
use actix_cors::Cors;
use actix_files as fs;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::get;
use actix_web::{cookie::Key, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Instant,
};

use backend::chat::chat_server::ChatServer;
use backend::chat::chat_session::WsChatSession;
use backend::dbadmin::handlers::*;


#[get("/ws")]
async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    session: Session,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    match session.get::<i32>("user_id")? {
        Some(id)=>println!("{}",id),
        _=>{}
        
    }
    println!("chat route connect! ");
    let username = req.query_string().to_string();
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_owned(),
            name: Some(username),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");
    let key = Key::generate();

    let visitor_count = Arc::new(AtomicUsize::new(0));
    let chat_server = ChatServer::new(visitor_count.clone()).start();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_, _| true)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
            ])
            .allowed_header(actix_web::http::header::CONTENT_TYPE)
            .supports_credentials() // 允许携带 cookies
            .max_age(3600);

        App::new()
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), key.clone())
                    .cookie_secure(false) // 生产环境应设置为true
                    .cookie_http_only(true)
                    .cookie_same_site(actix_web::cookie::SameSite::Lax) // 确保跨域时使用
                    .build(),
            )
            .wrap(cors)
            .app_data(web::Data::new(chat_server.clone())) // 添加 ChatServer 的 Data
            .app_data(web::Data::new(pool.clone()))
            .service(fs::Files::new("/uploads", "./uploads").show_files_listing())
            .service(login_user)
            .service(check_name)
            .service(upload_avatar)
            .service(register_user)
            .service(user_profile)
            .service(submit_post)
            .service(get_posts)
            .service(get_comments)
            .service(create_comment)
            .service(reply_comment)
            .service(submit_md_imgs)
            .service(submit_md)
            .service(reply_auth)
            .service(get_user_name_avater)
            .service(get_my_articles)
            .service(chat_route)
            .service(get_search_users)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
