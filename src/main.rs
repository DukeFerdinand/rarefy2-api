use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, middleware::{Logger, self}};
use mysql_async::{prelude::*, Pool};

struct AppState {
    app_name: String,
    conn_pool: Pool
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
                Ok(_) =>HttpResponse::Ok().body(format!("{} is healthy!", data.app_name)),
                Err(_) => HttpResponse::ServiceUnavailable().body("Database test query failed.")
            }
        },
        Err(_) => HttpResponse::ServiceUnavailable().body("Could not get connection from pool")
    }
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().unwrap();

    // Init using RUST_LOG, or all info level events if the env var is not set
    // The string given to default_filter_or is the same format as the env var
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found in env!");
        let conn_pool = Pool::from_url(database_url).expect("Could not parse DATABASE_URL into MySQL options");

        App::new()
            .wrap(middleware::Logger::new("%r %s"))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Rarefy API"),
                conn_pool
            }))
            .service(hello)
            .service(echo)
            .service(get_health)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}