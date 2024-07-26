use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize,Debug)]
pub struct CheckUserNameRequest{
    pub username: String,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize)]
pub struct SessionResponse{
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct PostRequst{
    pub title: Option<String>,
    pub content: String,
    pub images: Option<Vec<String>> ,
}

#[derive(Deserialize)]
pub struct PaginationParams{
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Serialize)]
pub struct PostResponse {
    pub id: i32,
    pub user_id:i32,
    pub nickname: String,
    pub avatar: String,
    pub title: String,
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct CreateCommentRequest{
    pub post_id: i32,
    pub content: String,
    pub parent_comment_id: Option<i32>,

}

#[derive(Deserialize)]
pub struct ReplyCommentRequest{
    pub post_id: i32,
    pub content:String,
    pub parent_comment_id: i32,
}

#[derive(Serialize)]
pub struct ReplyCommentResponse{
    pub id: i32,
    pub user_id:i32,
    pub nickname:String,
    pub content: String,
    pub parent_comment_id: i32,
    pub created_at:Option<DateTime<Utc>>
}

#[derive(Serialize)]
pub struct MdImgsResponse{
    pub urls: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct MdSubmitRequest{
    pub title:String,
    pub content:String,
    pub images: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct UserNameAvatarResponse{
    pub username: String,
    pub nickname: String,
    pub avatar: Option<String>,
}

#[derive(Serialize)]
pub struct ArticlesResponse{
    pub id: i32,
    pub title:String,
    pub content:String,
    pub created_at: Option<DateTime<Utc>>
}
#[derive(Deserialize)]
pub struct SearchUserRequest{
    pub username:String
}