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


impl Clone for ConfigSet {
    fn clone(&self) -> Self {
        ConfigSet {
            bearer_token: self.bearer_token.clone(),
            group_id: self.group_id.clone(),
            user_id: self.user_id.clone(),
            discord_webhook: self.discord_webhook.clone(),
        }
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            config: self.config.clone()
        }
    }
}