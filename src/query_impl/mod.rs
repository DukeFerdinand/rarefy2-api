use mysql_async::prelude::*;

use std::error::Error;

use crate::state::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub joined: String,
    pub updated: String,
}

pub async fn query_accounts(
    state: actix_web::web::Data<AppState>,
) -> Result<Vec<Account>, Box<dyn Error>> {
    let mut db = state.conn_pool.get_conn().await?;

    let query = r"
        SELECT id, username, joined, updated
        FROM accounts a;
    ";

    let accounts = db
        .query_map(query, |(id, username, joined, updated)| Account {
            id,
            username,
            password: None,
            joined,
            updated,
        })
        .await?;

    Ok(accounts)
}
