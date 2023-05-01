use std::error::Error;
use mysql_async::prelude::*;

use crate::state::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub joined: String,
    pub updated: String,
}

pub struct AccountsManager {
    app_state: actix_web::web::Data<AppState>
}

impl AccountsManager {
    pub fn from_app_state(app_state: actix_web::web::Data<AppState>) -> AccountsManager {
        AccountsManager { app_state }
    }

    pub async fn query_accounts(&self) -> Result<Vec<Account>, Box<dyn Error>> {
        let mut db = self.app_state.conn_pool.get_conn().await?;

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
}


