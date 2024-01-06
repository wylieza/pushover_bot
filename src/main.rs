extern crate pushover_rs;

use std::env;
use pushover_rs::{Message, MessageBuilder, PushoverResponse, PushoverSound, send_pushover_request};
use std::time::{SystemTime, UNIX_EPOCH};

struct Pushover_Credentials {
    user: String,
    api_key: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    if let Some(creds) = pushover_credentials_from_env() {
        reboot_notification(&creds).await;
    }
}

fn pushover_credentials_from_env() -> Option::<Pushover_Credentials> {
    let pushover_user_key_var = "PUSHOVER_USER_KEY";
    let pushover_bot_key_var = "PUSHOVER_GENERAL_PURPOSE_BOT_KEY";

    let user_key = env::var(pushover_user_key_var);
    let bot_key = env::var(pushover_bot_key_var);

    if user_key.is_err() || bot_key.is_err() {
        return None;
    }

    Some(Pushover_Credentials {
        user: user_key.unwrap(),
        api_key: bot_key.unwrap(),
    })
}

async fn reboot_notification(credentials: &Pushover_Credentials) {
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let now: u64 = duration_since_epoch.as_secs();

    let message: Message = MessageBuilder::new(&credentials.user, &credentials.api_key, "general purpose raspberry pi has rebooted!")
        .set_title("Pi is online")
        .set_priority(0)
        .set_sound(PushoverSound::ALIEN)
        .set_timestamp(now)
        .build();
    
    match send_pushover_request(message).await {
        Ok(_) => println!("all okay!"),
        Err(err) => {dbg!(err);},
    };
}
