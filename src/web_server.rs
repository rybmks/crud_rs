use crate::{data_base::DataBase, handle_users_getting, User};
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::vec;

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
    let mut db = DataBase::new("postgres://postgres:123456@localhost:5432/rust_postgre_test")
        .await
        .unwrap();

    let query = sqlx::query(
        r#"INSERT INTO "user" (name, age, date)
        VALUES ($1, $2, $3)"#,
    )
    .bind(&data.name)
    .bind(data.age)
    .bind("seichas");

    db.execute_queries(vec![query]).await.unwrap();

    HttpResponse::Ok().json(data)
}
async fn getting_string() -> impl Responder {
    let users = handle_users_getting().await;

    HttpResponse::Ok().json(users)
}
