use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDate, Utc};
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username:String,
    pub password: String,
    pub nickname: String,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub age: Option<i32>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDisPlay {
    pub username:String,
    pub nickname: String,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub age: Option<i32>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub username:String,
    pub password: String,
    pub nickname: String,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub age: Option<i32>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: Option<String>,
    pub content: String,
    pub images: Option<Vec<String>>,
    pub create_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePost {
    pub user_id: i32,
    pub title: Option<String>,
    pub content: String,
    pub images: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Comment {
    pub id: Option<i32>,
    pub post_id: Option<i32>,
    pub user_id: Option<i32>,
    pub content: Option<String>,
    // pub children: Option<Vec<Comment>>,
    pub parent_comment_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub nickname: Option<String>,

}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentWithChildren{
    pub comment:Comment,
    pub children:Option<Vec<CommentWithChildren>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateComment {
    pub post_id: i32,
    pub user_id: i32,
    pub comment: String,
    pub parent_comment_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Follow {
    pub follower_id: i32,
    pub followee_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginInfo{
    pub username:String,
    pub password:String
}

