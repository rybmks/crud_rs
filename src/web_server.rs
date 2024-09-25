use crate::{handle_adding_to_db, handle_users_getting, User};
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
pub async fn start_web_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .max_age(3600),
            )
            .service(
                web::scope("/api")
                    .route("/get", web::get().to(getting_string))
                    .route("/post", web::post().to(add_to_db)),
            )
    })
    .bind(("192.168.0.7", 8080))?
    .run()
    .await
}
async fn add_to_db(data: web::Json<User>) -> impl Responder {
    let res = handle_adding_to_db(data).await;

    match res {
        Ok(()) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}
async fn getting_string() -> impl Responder {
    let users = handle_users_getting().await;

    HttpResponse::Ok().json(users)
}
