use actix_web::{Responder, get};

#[get("/hello")]
pub async fn hello() -> impl Responder {
    "Hello world!"
}