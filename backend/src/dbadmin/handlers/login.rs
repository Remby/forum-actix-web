use crate::{dbadmin::model::LoginInfo, import::*};

#[post("/login")]
async fn login_user(
    session: Session,
    pool: web::Data<PgPool>,
    login_info: web::Json<LoginInfo>,
) -> Result<HttpResponse, Error> {
    println!("login handler");
    let user = sqlx::query_as!(
        SessionResponse,
        "SELECT id, username FROM users WHERE username = $1 AND password = $2",
        login_info.username,
        login_info.password
    )
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if let Some(user) = user {
        session.insert("user_id", user.id)?;
        session.renew();
        // let cookie = Cookie::build("auth", "true")
        //     .path("/")
        //     .same_site(actix_web::cookie::SameSite::Lax) // or None if you are testing locally
        //     .finish();
        let response = HttpResponse::Ok().json(user);
        // response.add_cookie(&cookie)?;
        println!("get login");
        Ok(response)
    } else {
        Ok(HttpResponse::Unauthorized().json("Invalid username or password"))
    }
}

#[get("/auth")]
async fn reply_auth(
    session: Session,
)->Result<HttpResponse,Error>{
    let result:Option<i32> = 
    session.get("user_id").map_err(
        |err|actix_web::error::ErrorInternalServerError(err))?;
    println!("{:#?}", result);
    match result {
        Some(_)=>Ok(HttpResponse::Ok().json(json!({"auth":true}))) ,
        _=>Ok(HttpResponse::Ok().json(json!({"auth":false}))) ,
    }
}