use data_base::DataBase;
use serde::{Deserialize, Serialize};
//use time::macros::format_description;
//use time::PrimitiveDateTime as DateTime;

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
