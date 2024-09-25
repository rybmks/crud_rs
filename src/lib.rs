use actix_web::web;
use data_base::DataBase;
use serde::{Deserialize, Serialize};

pub mod data_base;
pub mod web_server;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub name: String,
    pub age: i16,
    pub date: String,
}

pub async fn handle_users_getting() -> Vec<User> {
    let mut db = DataBase::new("postgres://postgres:123456@localhost:5432/rust_postgre_test")
        .await
        .unwrap();

    let q = r#"SELECT * FROM "user""#;

    db.get_rows_as_struct(q).await.unwrap()
}

pub async fn handle_adding_to_db(data: web::Json<User>) -> Result<(), sqlx::Error> {
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

    db.execute_queries(vec![query]).await?;

    Ok(())
}
