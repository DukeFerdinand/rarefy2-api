mod query_impl;
mod state;

use actix_web::{
    get, http::header::ContentType, middleware, post, web, App, HttpResponse, HttpServer, Responder,
};
use log::error;
use mysql_async::{prelude::*, Pool};
use serde::Serialize;

use state::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
struct Account {
    pub id: String,
    pub username: String,
    pub password: Option<String>,
    pub joined: String,
    pub updated: String,
}

fn to_json(data: impl Serialize) -> Result<String, serde_json::Error> {
    serde_json::to_string(&data)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/health")]
async fn get_health(data: web::Data<AppState>) -> impl Responder {
    let conn = data.conn_pool.get_conn().await;

    match conn {
        Ok(c) => {
            let res = r"SELECT 1;".ignore(c).await;

            match res {
                Ok(_) => HttpResponse::Ok().body(format!("{} is healthy!", data.app_name)),
                Err(_) => HttpResponse::ServiceUnavailable().body("Database test query failed."),
            }
        }
        Err(_) => HttpResponse::ServiceUnavailable().body("Could not get connection from pool"),
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/accounts")]
async fn get_accounts(state: web::Data<AppState>) -> impl Responder {
    match query_impl::query_accounts(state).await {
        Ok(acc) => {
            let json = to_json(acc).unwrap();
            HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(json)
        }
        Err(e) => {
            error!("Error querying accounts: {}", e);
            HttpResponse::InternalServerError().body("Something went wrong while querying accounts")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init using RUST_LOG, or all info level events if the env var is not set
    // The string given to default_filter_or is the same format as the env var
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    match dotenvy::dotenv() {
        Ok(_) => log::info!("Loaded .env"),
        Err(_) => {
            log::warn!("Could not load .env! If this is expected please ignore this warning :)")
        }
    }

    HttpServer::new(|| {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env!");
        let conn_pool =
            Pool::from_url(database_url).expect("Could not parse DATABASE_URL into MySQL options");

        App::new()
            .wrap(middleware::Logger::new("%r %s"))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Rarefy API"),
                conn_pool,
            }))
            .service(hello)
            .service(echo)
            .service(get_health)
            .service(get_accounts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
