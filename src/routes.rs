use actix_web::{ Responder, get, post, web::Path };
use futures_util::StreamExt;
use serde::{ Serialize, Deserialize };



#[get("/hello")]
pub async fn hello() -> impl Responder {
    "Hello world!"
}


struct AddRequest {
    location: String,
    course: String
}

#[post("/add/{username}")]
pub async fn add_session(username: Path<String>, mut payload: actix_web::web::Payload) -> impl Responder {
    let mut body = actix_web::web::BytesMut::new();
    while let Some(s) = payload.next().await {
        body.extend_from_slice(&s.unwrap());
    }
    
    dbg!(username);
    dbg!(body);
    "success"
}