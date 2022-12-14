use log::info;
use regex::Regex;
use teloxide::{
    requests::Requester,
    types::{ChatId, UserId},
    Bot,
};

use crate::util::split_by_commas;

#[derive(Debug, Clone)]
pub struct TelegramBot {
    bot: Bot,
    recipients: Receivers,
}

const USER_GROUPS_REGEX: &str = r"users:(?P<users>\S*);groups:(?P<groups>\S*)";

#[derive(Debug, Clone)]
struct Receivers {
    users: Vec<UserId>,
    groups: Vec<ChatId>,
}

impl TelegramBot {
    fn get_receivers(env_var: String) -> Receivers {
        let user_groups_regex = Regex::new(USER_GROUPS_REGEX).unwrap();
        let captures = user_groups_regex.captures(&env_var).expect("TELEGRAM_USERS should have this format `users:<array_of_users>;groups:<array_of_groups>`");

        let user_ids = captures.get(1).unwrap().as_str();
        let group_ids = captures.get(2).unwrap().as_str();

        Receivers {
            users: split_by_commas(user_ids)
                .iter()
                .map(|user_id| UserId(*user_id))
                .collect(),
            groups: split_by_commas(group_ids)
                .iter()
                .map(|group_id| ChatId((*group_id).try_into().unwrap()))
                .collect(),
        }
    }

    pub fn new(key: &str, recipients: String) -> Self {
        let bot = Bot::new(key);
        let recipients = TelegramBot::get_receivers(recipients);
        info!("Starting telegram bot");

        TelegramBot { recipients, bot }
    }

    pub async fn send_message(&self, message: String) {
        for user in self.recipients.users.iter() {
            if let Err(e) = self.bot.send_message(*user, &message).await {
                println!("{:?}", e)
            }
        }

        for chat in self.recipients.groups.iter() {
            if let Err(e) = self.bot.send_message(*chat, &message).await {
                println!("{:?}", e)
            }
        }
    }
}
