use actix_web::{HttpServer, App};

mod routes;


macro_rules! include_routes {
    ($($x:literal,*)*) => {
        $(
            .service($x)
        )*
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::hello)
    })
    .bind(("127.0.0.1", 8761))?
    .run()
    .await
}
