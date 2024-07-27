use crate::import::*;

#[post("/submit-post")]
async fn submit_post(
    session: Session,
    pool: web::Data<PgPool>,
    mut post_info: Multipart,
) -> Result<HttpResponse, Error> {
    println!("submit post handler");
    let mut post_request = PostRequst {
        title: None,
        content: String::new(),
        images: None,
    };

    while let Some(item) = post_info.next().await {
        let mut field = item?;
        let name = field.content_disposition().get_name().unwrap();
        match name {
            "title" => {
                println!("title");
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    println!("{:#?}", String::from_utf8(data.to_vec()));
                    post_request.title = Some(String::from_utf8(data.to_vec()).unwrap());
                }
            }
            "content" => {
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    println!("{:#?}", String::from_utf8(data.to_vec()));
                    post_request.content = String::from_utf8(data.to_vec()).unwrap();
                }
            }
            "images" => {
                let file_name = field.content_disposition().get_filename().unwrap();
                let file_extension = file_name.split('.').last().unwrap();
                let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
                let file_path = format!("./uploads/post_images/{}", unique_filename);

                match &mut post_request.images {
                    None => {
                        post_request.images = Some(vec![file_path.clone()]);
                    }
                    Some(ref mut i) => {
                        i.push(file_path.clone());
                    }
                }

                let fp_clone = file_path.clone();
                web::block(|| std::fs::File::create(fp_clone)).await??;

                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    let filepath_clone = file_path.clone();
                    web::block(move || {
                        let mut f = std::fs::OpenOptions::new()
                            .append(true)
                            .open(filepath_clone)
                            .unwrap();
                        f.write_all(&data)
                    })
                    .await??;
                }
            }
            _ => {}
        }
    }

    // println!("{:#?}", session.get::<bool>("auth"));
    let user_id: i32 = session.get("user_id")?.unwrap();
    sqlx::query!(
        "INSERT INTO posts (user_id, title, content, images) VALUES ($1, $2, $3, $4)",
        user_id,
        post_request.title,
        post_request.content,
        post_request.images.as_deref()
    )
    .execute(pool.get_ref())
    .await
    .unwrap();
    println!("ok");

    Ok(HttpResponse::Ok().json(json!({"success":true})))
}

#[get("/posts")]
async fn get_posts(
    pool: web::Data<PgPool>,
    query: web::Query<PaginationParams>,
) -> Result<HttpResponse, Error> {
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let rows = sqlx::query!(
        "SELECT posts.id, posts.user_id, users.nickname,users.avatar, 
        posts.title, posts.content, posts.images, posts.created_at
        FROM posts JOIN users ON posts.user_id=users.id ORDER BY posts.created_at DESC LIMIT $1 OFFSET $2",
        limit,
        offset
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let posts: Vec<PostResponse> = rows
        .into_iter()
        .map(|row| PostResponse {
            id: row.id,
            user_id: row.user_id,
            avatar: row.avatar.unwrap(),
            nickname: row.nickname,
            title: row.title,
            content: row.content,
            images: if let Some(img) = &row.images {
                for i in img.iter() {
                    println!("{:#?}", i);
                }
                row.images
            } else {
                None
            },
            created_at: row.created_at,
        })
        .collect();
    println!("{:#?}", posts.len());
    Ok(HttpResponse::Ok().json(json!({ "posts": posts })))
}
