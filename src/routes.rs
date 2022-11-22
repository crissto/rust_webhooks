use rocket::{request::FromParam, response::content::RawJson, State};

use crate::{telegram::TelegramBot, util::title_case};

#[get("/health")]
pub fn health() -> RawJson<&'static str> {
    info!("Health is ok");
    RawJson("{ \"ok\": true }")
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub enum Status {
    Ok,
    Error,
    Unknown,
}

impl<'r> FromParam<'r> for Status {
    type Error = &'r str;

    fn from_param<'a>(param: &'a str) -> Result<Self, <Status as FromParam>::Error> {
        match param.to_ascii_lowercase().as_str() {
            "ok" => Ok(Status::Ok),
            "error" => Ok(Status::Error),
            _ => Ok(Status::Unknown),
        }
    }
}

#[get("/<service>/<status>?<message>&<rest..>")]
pub async fn webhook(
    data: &State<TelegramBot>,
    service: String,
    status: Status,
    message: String,
    rest: Option<String>,
) -> RawJson<&'static str> {
    let emoji = match status {
        Status::Ok => "✅",
        Status::Error => "❌",
        Status::Unknown => "❓",
    };
    let mut message = format!(
        "{} {} said {}",
        emoji,
        title_case(service.as_str()),
        message,
    );

    let rest_inner = match rest {
        Some(rest) => rest,
        None => "".to_string(),
    };
    if rest_inner.len() > 1 {
        message.push_str(&format!("\n\nExtra:\t{}", rest_inner).as_str());
    }

    data.send_message(message).await;

    RawJson("{ \"ok\": true }")
}
