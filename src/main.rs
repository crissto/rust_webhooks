#[macro_use]
extern crate rocket;

use crate::routes::{health, webhook};
use crate::telegram::TelegramBot;
use dotenv::dotenv;
use serde::Deserialize;

mod routes;
mod telegram;
mod util;

#[derive(Deserialize, Debug)]
struct EnvCofig {
    telegram_bot_key: String,
    telegram_users: String,
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    pretty_env_logger::init();
    let envs = envy::from_env::<EnvCofig>()
        .expect("You have to provide both TELEGRAM_BOT_KEY and TELEGRAM_USER");
    let telegram_bot = TelegramBot::new(&envs.telegram_bot_key, envs.telegram_users);

    rocket::build()
        .manage(telegram_bot)
        .mount("/", routes![webhook, health])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::routes::rocket_uri_macro_health;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn hello_world() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get(uri!(health)).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "{ \"ok\": true }");
    }
}
