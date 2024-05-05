use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Member {
    #[serde(rename = "firstName")]
    pub(crate) first_name: String,
    #[serde(rename = "lastName")]
    pub(crate) last_name: String,
    pub(crate) id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Group {
    pub(crate) members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Recipient {
    pub(crate) group: Group,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Event {
    pub(crate) recipients: Recipient,
    // Include other fields from your JSON structure as needed
}