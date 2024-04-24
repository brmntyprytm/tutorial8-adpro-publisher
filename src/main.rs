use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, MessageHandler, HandleError};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }
    
    fn get_handler_action(&self) -> String {
        todo!()
    }
}

fn main() {
    let mut p = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
    let _ = p.publish_event("user_created".to_owned(), 
        UserCreatedEventMessage { user_id: "1".to_owned(), user_name:
        "2206821563-Amir".to_owned() });
    let _ = p.publish_event("user_created".to_owned(),
        UserCreatedEventMessage { user_id: "2".to_owned(), user_name:
        "2206821563-Budi".to_owned() });
    let _ = p.publish_event("user_created".to_owned(),
        UserCreatedEventMessage { user_id: "3".to_owned(), user_name:
        "2206821563-Cica".to_owned() });
    let _ = p.publish_event("user_created".to_owned(),
        UserCreatedEventMessage { user_id: "4".to_owned(), user_name:
        "2206821563-Dira".to_owned() });
    let _ = p.publish_event("user_created".to_owned(),
        UserCreatedEventMessage { user_id: "5".to_owned(), user_name:
        "2206821563-Emir".to_owned() });
}
