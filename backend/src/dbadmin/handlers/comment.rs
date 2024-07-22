use crate::{
    dbadmin::model::{Comment, CommentWithChildren},
    import::*,
};

async fn get_comments_by_post_id(
    pool: &PgPool,
    post_id: i32,
) -> Result<Vec<CommentWithChildren>, sqlx::Error> {
    let comments = sqlx::query_as!(
        Comment,
        "
        WITH RECURSIVE comment_tree AS (
        SELECT c.id, c.post_id, c.user_id, c.content, c.parent_comment_id, c.created_at, u.nickname
        FROM comments c 
        JOIN users u ON c.user_id = u.id
        WHERE c.post_id =$1 AND c.parent_comment_id IS NULL
        UNION ALL
        SELECT c.id, c.post_id, c.user_id, c.content, c.parent_comment_id, c.created_at, u.nickname
        FROM comments c 
        JOIN users u ON c.user_id = u.id
        INNER JOIN comment_tree ct ON ct.id = c.parent_comment_id
        )
        SELECT id, post_id ,user_id ,content, parent_comment_id, created_at, nickname 
        FROM comment_tree
        ORDER BY created_at
        ",
        post_id
    )
    .fetch_all(pool)
    .await?;
    let comments_with_children: Vec<_> = comments
        .into_iter()
        .map(|comment| CommentWithChildren {
            comment,
            children: Some(Vec::new()),
        })
        .collect();

    fn build_comment_tree(
        parent: &mut CommentWithChildren,
        comments: &mut Vec<CommentWithChildren>,
    ) {
        let mut i = 0;
        while i < comments.len() {
            if comments[i].comment.parent_comment_id == parent.comment.id {
                let mut child = comments.remove(i);
                build_comment_tree(&mut child, comments);
                parent.children.as_mut().unwrap().push(child);
            } else {
                i += 1;
            }
        }
    }

    let mut root_comments: Vec<CommentWithChildren> = Vec::new();
    let mut remaining_comments: Vec<CommentWithChildren> = Vec::new();

    for comment in comments_with_children.into_iter() {
        if comment.comment.parent_comment_id.is_none() {
            root_comments.push(comment);
        } else {
            remaining_comments.push(comment);
        }
    }

    for root_comment in &mut root_comments {
        println!("{:#?}", root_comment.comment.created_at);
        build_comment_tree(root_comment, &mut remaining_comments);
    }

    Ok(root_comments)
}

#[get("/posts/{post_id}/comments")]
async fn get_comments(
    pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    println!("get comment");
    match get_comments_by_post_id(&pool, post_id.into_inner()).await {
        Ok(comments) => Ok(HttpResponse::Ok().json(comments)),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
    }
}

#[post("/submit-comment")]
async fn create_comment(
    session: Session,
    pool:web::Data<PgPool>,
    new_comment: web::Json<CreateCommentRequest>
)->Result<HttpResponse,Error>{
    let raw_user_id:Option<i32> = session.get("user_id")?;
    let user_id;
    if let Some(id) = raw_user_id{
        user_id = id; 
        println!("{user_id}");
    }
    else{
        
        return Err(actix_web::error::ErrorInternalServerError("error"));
    }
    println!("{:#?}", new_comment.0.parent_comment_id);
    let _comment = sqlx::query!(
        "
        INSERT INTO comments (user_id,post_id, content, parent_comment_id)
        VALUES ($1, $2, $3, $4)
        RETURNING id, post_id, user_id, content, parent_comment_id, created_at
        ",
        user_id,
        new_comment.post_id,
        new_comment.content,
        new_comment.parent_comment_id
         
    ).fetch_one(pool.get_ref())
    .await
    .map_err(|e|{
        eprint!("{:#?}",e);
        actix_web::error::ErrorInternalServerError(e)
    })?;
    println!("Ok created comment");
    Ok(HttpResponse::Ok().finish())
}


#[post("/reply-comment")]
async fn reply_comment(
    session:Session,
    pool:web::Data<PgPool>,
    reply:web::Json<ReplyCommentRequest>
)->Result<HttpResponse,Error>{
    println!("reply comment");
    let user_id:i32;
    match session.get("user_id")? {
        Some(id)=>user_id=id,
        _=>return  Err(actix_web::error::ErrorInternalServerError("error"))

    }
    println!("user id: {}", user_id);

    let result = sqlx::query!(
        "
        INSERT INTO comments (user_id,post_id,content,parent_comment_id)
        VALUES ($1,$2,$3,$4)
        RETURNING id, content, created_at
        ",
        user_id, 
        reply.post_id,
        reply.content,
        reply.parent_comment_id
    ).fetch_one(pool.get_ref()).
    await.map_err(|err|actix_web::error::ErrorInternalServerError(err))?;

    print!("reply ok");

    let user = sqlx::query!(
        "SELECT nickname FROM   users WHERE id=$1"
        ,user_id
    ).fetch_one(pool.get_ref()).
    await.map_err(|err|actix_web::error::ErrorInternalServerError(err))?;

    let response = ReplyCommentResponse{
        id:result.id,
        user_id:user_id,
        nickname:user.nickname,
        content:result.content,
        parent_comment_id:reply.parent_comment_id,
        created_at: result.created_at,
    };

    Ok(HttpResponse::Ok().json(response))
}