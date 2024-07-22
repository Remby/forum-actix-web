use actix_cors::Cors;
use actix_files as fs;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use backend::dbadmin::handlers::*;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

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

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_,_|true)
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
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
