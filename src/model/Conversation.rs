use serde::Deserializer;
use serde::Serializer;

#[derive(Serializer,Deserializer,Clone,Debug)]
pub struct Conversation{
    pub  messages: Vec<message>,
}
impl Conversation{
    pub fn new()->Conversation{
        Conversation{
            messages:Vec::new()
        }
    }
}


#[derive(Serializer,Deserializer,Clone,Debug)]
pub struct Message{
    pub user:bool,
    pub user: String,
}

