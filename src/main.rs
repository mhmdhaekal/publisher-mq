use std::env;

use borsh_derive::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use dotenv::dotenv;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct UserCreatedEventMesage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMesage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMesage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
}

fn main() {
    dotenv().ok();
    let rabbitmq_url = env::var("RABBITMQ_URL").unwrap();
    let mut publisher = CrosstownBus::new_publisher(rabbitmq_url).unwrap();
    let users: Vec<&str> = vec!["Amir", "Budi", "Cica", "Dira", "Emir"];

    for (i, user) in users.iter().enumerate() {
        let user_id = (i + 1).to_string();
        let user_name = format!("2206817490-{}", user);
        let message: UserCreatedEventMesage = UserCreatedEventMesage { user_id, user_name };
        _ = publisher.send(String::from("user_created"), message)
    }
}
