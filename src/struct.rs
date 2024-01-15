// https://schema.org/Message
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
  // sender can be persion or recipient
    pub sender: Person,
    pub recipient: Person,

    pub  DatePublished: String,
}

pub struct Person {
    pub name: String,
}