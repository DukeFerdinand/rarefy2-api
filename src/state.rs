use mysql_async::Pool;

pub struct AppState {
    pub app_name: String,
    pub conn_pool: Pool,
}
