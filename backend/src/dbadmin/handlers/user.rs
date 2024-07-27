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
    if query.username.is_empty() {
        return Ok(HttpResponse::Ok().finish());
    }
    let result = sqlx::query_as!(
        UserNameAvatarResponse,
        "SELECT username, nickname, avatar FROM users WHERE (username LIKE $1 OR nickname LIKE $1)",
        format!("{}%", query.username)
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|err| actix_web::error::ErrorServiceUnavailable(err));

    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record)),
        _ => Err(actix_web::error::ErrorBadRequest("err")),
    }
}

#[post("/add-users")]
async fn add_users(
    session: Session,
    pool: web::Data<PgPool>,
    username: String,
) -> Result<HttpResponse, Error> {
    println!("add users handler");

    let sender_id: i32;
    match session.get("user_id")? {
        Some(id) => sender_id = id,
        _ => return Err(actix_web::error::ErrorUnauthorized("No Auth")),
    }
    println!("sender id{}", sender_id);
    println!("receive username:{}", username);
    let result = sqlx::query!(
        "INSERT INTO friends_requests 
        (sender_id, receiver_id)
        SELECT $1,id FROM users WHERE username=$2
        RETURNING id",
        sender_id,
        username
    )
    .fetch_one(pool.get_ref())
    .await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().json(json!({"success":true}))),
        Err(err) => {
            println!("{:#?}", err);
            Ok(HttpResponse::Ok().json(json!({"success":false})))
        }
    }
}

#[get("/get-friends-requests")]
async fn get_friends_requests(
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    println!("get request handler");

    let receiver_id: i32;
    match session.get("user_id")? {
        Some(id) => receiver_id = id,
        _ => return Err(actix_web::error::ErrorUnauthorized("No Auth")),
    }

    let result = sqlx::query_as!(
        UserNameAvatarResponse,
        "
        SELECT u.username, u.nickname, u.avatar FROM friends_requests r
        JOIN users u ON r.sender_id = u.id
        WHERE r.receiver_id = $1
        ",
        receiver_id
    )
    .fetch_all(pool.get_ref())
    .await;
    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record)),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}

#[post("/accept-friends-requests")]
async fn accept_friends_requests(
    session: Session,
    pool: web::Data<PgPool>,
    username: String,
) -> Result<HttpResponse, Error> {
    println!("accept request handler");

    let receiver_id: i32;
    match session.get("user_id")? {
        Some(id) => receiver_id = id,
        _ => return Err(actix_web::error::ErrorUnauthorized("No Auth")),
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
    let result1 = sqlx::query!(
        "
        INSERT INTO friends (user_id, friend_id)
        SELECT $1, id FROM users WHERE username = $2;
        ",
        receiver_id,
        username
    )
    .execute(&mut transaction)
    .await;
    let result2 = sqlx::query!(
        "
        INSERT INTO friends (user_id, friend_id)
        SELECT id, $1 FROM users WHERE username = $2;
        ",
        receiver_id,
        username
    )
    .execute(&mut transaction)
    .await;

    let result3 = sqlx::query!(
        "
        DELETE FROM friends_requests WHERE sender_id= 
        (SELECT id FROM users WHERE username=$1) AND
        receiver_id=$2;
        ",
        username,
        receiver_id
    )
    .execute(&mut transaction)
    .await;

    match (result1, result2, result3) {
        (Ok(_), Ok(_), Ok(_)) => {
            transaction
                .commit()
                .await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
            Ok(HttpResponse::Ok().json(json!({"success":true})))
        }

        _ => {
            transaction
                .rollback()
                .await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err))?;
            Ok(HttpResponse::Ok().json(json!({"success":false})))
        }
    }
}

#[get("/get-friends-list")]
async fn get_friends_list(
    session: Session,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    println!("get friends list handler");

    let receiver_id: i32;
    match session.get("user_id")? {
        Some(id) => receiver_id = id,
        _ => return Err(actix_web::error::ErrorUnauthorized("No Auth")),
    }

    let result = sqlx::query_as!(
        UserNameAvatarResponse,
        "
        SELECT u.username, u.nickname, u.avatar FROM friends f
        JOIN users u ON f.friend_id = u.id
        WHERE f.user_id = $1
        ",
        receiver_id
    )
    .fetch_all(pool.get_ref())
    .await;
    match result {
        Ok(record) => Ok(HttpResponse::Ok().json(record)),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}
