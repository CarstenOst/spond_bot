use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ConfigSet {
    pub(crate) bearer_token: String,
    pub(crate) group_id: String,
    pub(crate) user_id: String,
    pub(crate) discord_webhook: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) config: Vec<ConfigSet>,
}

