use crate::{dbadmin::model::CreateUser, import::*};

#[post("/register")]
async fn register_user(pool: web::Data<PgPool>, user: web::Json<CreateUser>) -> impl Responder {
    println!("register hander");
    let result = sqlx::query!(
        "INSERT INTO users (nickname, gender, birthdate, age, avatar, bio, username, password)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
        user.nickname,
        user.gender,
        user.birthdate,
        user.age,
        user.avatar,
        user.bio,
        user.username,
        user.password,
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"success": true})),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/check-name")]
async fn check_name(
    pool: web::Data<PgPool>,
    user_name: web::Json<CheckUserNameRequest>,
) -> impl Responder {
    let username = user_name.0.username.as_str();
    println!("here");
    let result = sqlx::query!("SELECT COUNT(*) FROM users WHERE username = $1", username)
        .fetch_one(pool.get_ref())
        .await;
    match result {
        Ok(record) => {
            if record.count.unwrap_or(0) > 0 {
                HttpResponse::Ok().json(json!({"available": false}))
            } else {
                HttpResponse::Ok().json(json!({"available": true}))
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/upload-avatar")]
async fn upload_avatar(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let filepath;
    println!("here");
    while let Ok(Some(mut field)) = payload.try_next().await {
        println!("hello");
        let content_disposition = field.content_disposition();
        println!("{:#?}", content_disposition);
        if let Some(filename) = content_disposition.get_filename() {
            let file_extension = filename.split('.').last().unwrap();
            let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
            filepath = format!("./uploads/avatar/{}", unique_filename);
            println!("{}", filepath);
            let current_dir = std::env::current_dir().unwrap();
            println!("Current directory: {:?}", current_dir);
            let filepath_clone = filepath.clone();
            let mut f = web::block(|| std::fs::File::create(filepath_clone)).await??;
            println!("finish");
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                f = web::block(move || f.write_all(&data).map(|_| f))
                    .await?
                    .expect("error");
            }
            println!("{:#?}", filepath);
            // println!("aaa");
            return Ok(HttpResponse::Ok().json(json!({ "url": filepath })));
        } else {
            return Err(actix_web::error::InternalError::from_response(
                "Missing filename",
                HttpResponse::BadRequest().body("Missing filename"),
            )
            .into());
        }
    }

    Err(actix_web::error::InternalError::from_response(
        "Error reading multipart field",
        HttpResponse::BadRequest().body("Error reading multipart field"),
    )
    .into())
}
