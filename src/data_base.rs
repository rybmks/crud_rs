use sqlx::{PgPool, Pool, Postgres};

pub struct DataBase {
    pool: Option<Pool<Postgres>>,
}
impl DataBase {
    pub async fn get_rows_as_struct<'a, T>(&mut self, query: &str) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        if let Some(conn_pool) = &self.pool {
            let users = sqlx::query_as::<_, T>(query).fetch_all(conn_pool).await?;
            Ok(users)
        } else {
            Err(sqlx::Error::PoolTimedOut)
        }
    }

    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(url).await?;
        Ok(Self { pool: Some(pool) })
    }

    pub async fn open_pool(&mut self, url: &str) -> Result<(), sqlx::Error> {
        let conn_pool = PgPool::connect(url).await?;
        self.pool = Some(conn_pool);
        Ok(())
    }

    pub async fn close_pool(&mut self) {
        if let Some(pool) = self.pool.take() {
            pool.close().await;
        }
    }
    pub async fn execute_queries<'a>(
        &mut self,
        queries: Vec<sqlx::query::Query<'a, Postgres, sqlx::postgres::PgArguments>>,
    ) -> Result<Vec<sqlx::postgres::PgQueryResult>, sqlx::Error> {
        if let Some(conn_pool) = &self.pool {
            let transaction = conn_pool.begin().await?;

            let mut results = Vec::new();

            for query in queries {
                let result = query.execute(conn_pool).await?;
                results.push(result);
            }

            transaction.commit().await?;

            Ok(results)
        } else {
            Err(sqlx::Error::PoolTimedOut)
        }
    }
}
