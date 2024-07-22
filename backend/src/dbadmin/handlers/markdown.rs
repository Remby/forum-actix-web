use crate::import::*;

#[post("/submit-md-imgs")]
async fn submit_md_imgs(mut imgs_info: Multipart) -> Result<HttpResponse, Error> {
    println!("submit md imgs");

    let mut urls: Vec<String> = Vec::new();
    while let Some(item) = imgs_info.next().await {
        let mut field = item?;
        let file_name = field.content_disposition().get_filename().unwrap();
        let file_extension = file_name.split('.').last().unwrap();
        let unique_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
        let file_path = format!("./uploads/md_images/{}", unique_filename);
        println!("{file_path}");
        urls.push(file_path.clone());
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

    Ok(HttpResponse::Ok().json(MdImgsResponse { urls: urls }))
}

#[post("/submit-md")]
async fn submit_md(
    session: Session,
    pools: web::Data<PgPool>,
    md: web::Json<MdSubmitRequest>,
) -> Result<HttpResponse, Error> {
    println!("submit md");
    let user_id: i32;
    match session.get("user_id")? {
        Some(id) => user_id = id,
        _ => return Err(actix_web::error::ErrorInternalServerError("ERR")),
    }
    let _result = sqlx::query!(
        "INSERT INTO markdowns (user_id, title, content, image_urls) VALUES ($1, $2, $3, $4) ",
        user_id,
        md.title,
        md.content,
        md.images.as_deref(),
    )
    .execute(pools.get_ref())
    .await
    .map_err(|x| actix_web::error::ErrorInternalServerError(x));
    println!("ok");
    Ok(HttpResponse::Ok().finish())
}

#[get("/my-articles")]
async fn get_my_articles(
    session: Session, 
    pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let user_id:i32;
    match session.get("user_id")?{
       Some(id) =>user_id = id,
       _=>return Err(actix_web::error::ErrorUnauthorized("err"))
    }

    let result = sqlx::query_as!(
        ArticlesResponse,
        "SELECT id, title,content, created_at FROM markdowns WHERE user_id=$1",
        user_id
    ).fetch_all(pool.get_ref()).await
    .map_err(|err|actix_web::error::ErrorInternalServerError(err))?;

        Ok(HttpResponse::Ok().json(result))
}
