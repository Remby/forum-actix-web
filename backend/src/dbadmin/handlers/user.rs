use crate::{dbadmin::model::UserDisPlay, import::*};

#[get("/user")]
async fn user_profile(session: Session, pool: web::Data<PgPool>) -> impl Responder {
    println!("user profile handler");
    let user_id: i32 = match session.get("user_id") {
        Ok(Some(id)) => id,
        _ => return HttpResponse::Unauthorized().finish(),
    };
    println!("{user_id}");
    let result = sqlx::query_as!(UserDisPlay,
        "SELECT username, nickname, gender, age, birthdate, avatar, bio, created_at FROM users WHERE id = $1",
        user_id
    ).fetch_one(pool.get_ref()).await;
    println!("doneA");
    match result {
        Ok(record) => HttpResponse::Ok().json(record),
        _ => HttpResponse::Unauthorized().finish(),
    }
}

#[get("/get-user-name-avatar")]
async fn get_user_name_avater(
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    println!("get username avatar handler");
    let user_id: i32;
    match session.get("user_id")? {
        Some(id) => user_id = id,
        _ => return Err(actix_web::error::ErrorUnauthorized("No Auth")),
    }

    let result = sqlx::query_as!(
        UserNameAvatarResponse,
        "SELECT username, nickname, avatar FROM users WHERE id=$1",
        user_id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(|err| actix_web::error::ErrorServiceUnavailable(err));

    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record)),
        _ => Err(actix_web::error::ErrorBadRequest("err")),
    }
}

#[get("/search/users")]
async fn get_search_users(
    session: Session,
    pool: web::Data<PgPool>,
    query: web::Query<SearchUserRequest>,
) -> Result<HttpResponse, Error> {
    println!("search username  handler");
    let user_id: i32;
    match session.get("user_id")? {
        Some(id) => user_id = id,
        _ => return Err(actix_web::error::ErrorUnauthorized("No Auth")),
    }
    if query.username.is_empty(){
        return Ok(HttpResponse::Ok().finish());
    }
    let result = sqlx::query_as!(
        UserNameAvatarResponse,
        "SELECT username, nickname, avatar FROM users WHERE (username LIKE $1 OR nickname LIKE $1)",
        format!("{}%",query.username)
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|err| actix_web::error::ErrorServiceUnavailable(err));

    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record)),
        _ => Err(actix_web::error::ErrorBadRequest("err")),
    }
}
