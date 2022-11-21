use regex::Regex;
use teloxide::{
    requests::Requester,
    types::{ChatId, Recipient, UserId},
    Bot,
};

#[derive(Debug, Clone)]
pub struct TelegramBot {
    bot: Bot,
    recipients: Receivers,
}

const USER_GROUPS_REGEX: &str = r"users:(?P<users>\S*);groups:(?P<groups>\S*)";

fn split_by_commas(string: &str) -> Vec<u64> {
    let ids: Vec<&str> = string.split(",").skip_while(|&x| x.is_empty()).collect();
    ids.into_iter().map(|x| x.parse::<u64>().unwrap()).collect()
}

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
        println!("{:?}", recipients);
        return TelegramBot { recipients, bot };
    }

    pub async fn send_message(&mut self, message: String) {
        let recipients = self.recipients.users.append(&mut self.recipients.groups);
        for user in self.recipients.users.iter() {
            match self.bot.send_message(*user, &message).await {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }

        for chat in self.recipients.groups.iter() {
            match self.bot.send_message(*chat, &message).await {
                Err(e) => println!("{:?}", e),
                _ => (),
            }
        }
    }
}
