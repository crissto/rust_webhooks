use std::sync::{Arc, Mutex};

use actix_web::{
    get,
    middleware::Logger,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder, Result,
};

use dotenv::dotenv;
use log::info;
use serde::{Deserialize, Serialize};
use telegram::TelegramBot;
use teloxide::prelude::*;

mod telegram;

struct AppState {
    telegram_bot: Arc<Mutex<TelegramBot>>,
}

#[derive(Deserialize, Debug)]
struct EnvCofig {
    telegram_bot_key: String,
    telegram_users: String,
}

const PORT: i32 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    pretty_env_logger::init();
    info!("Starting bot");
    let envs = envy::from_env::<EnvCofig>()
        .expect("You have to provide both TELEGRAM_BOT_KEY and TELEGRAM_USER");

    let telegram_bot = TelegramBot::new(&envs.telegram_bot_key, envs.telegram_users);
    let data = Data::new(AppState {
        telegram_bot: Arc::new(Mutex::new(telegram_bot)),
    });

    info!("Server running at http://localhost:{}", PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
            .service(health)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

#[get("/")]
async fn hello(data: Data<AppState>) -> impl Responder {
    let mut bot = data.telegram_bot.lock().unwrap();
    bot.send_message(String::from("TEST!")).await;
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
}

#[get("/health")]
async fn health() -> Result<impl Responder> {
    info!("Health is ok");
    Ok(web::Json(HealthResponse { ok: true }))
}
