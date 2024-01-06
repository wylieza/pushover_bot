extern crate pushover_rs;

use std::env;
use std::net::TcpStream;
use std::time::Duration;
use pushover_rs::{Message, MessageBuilder, PushoverSound, send_pushover_request};
use std::time::{SystemTime, UNIX_EPOCH};

struct PushoverCredentials {
    user: String,
    api_key: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    if !simple_wait_internet() {
        println!("unable to connect to internet!");
        return;
    }
    if let Some(creds) = pushover_credentials_from_env() {
        reboot_notification(&creds).await;
    }
}

fn pushover_credentials_from_env() -> Option::<PushoverCredentials> {
    let pushover_user_key_var = "PUSHOVER_USER_KEY";
    let pushover_bot_key_var = "PUSHOVER_GENERAL_PURPOSE_BOT_KEY";

    let user_key = env::var(pushover_user_key_var);
    let bot_key = env::var(pushover_bot_key_var);

    if user_key.is_err() || bot_key.is_err() {
        return None;
    }

    Some(PushoverCredentials {
        user: user_key.unwrap(),
        api_key: bot_key.unwrap(),
    })
}

async fn reboot_notification(credentials: &PushoverCredentials) {
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

fn simple_wait_internet() -> bool {
    let retry_interval = Duration::from_secs(5);
    const MAX_RETRIES: u32 = 12;

    let mut retry_count = 0;

    // Loop until a valid internet connection is established
    loop {
        match TcpStream::connect("example.com:80") {
            Ok(_) => {
                return true;
            }
            Err(_) => {
                retry_count += 1;
            }
        }

        if retry_count > MAX_RETRIES {
            return false;
        }

        // Sleep for a while before the next retry
        std::thread::sleep(retry_interval);
    }
}
