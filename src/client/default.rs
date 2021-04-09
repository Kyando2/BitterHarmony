use super::BitterHandler;

pub struct ClientConfig;

pub struct BitterClient {
    config: ClientConfig,
}

impl BitterClient {
    pub fn new() -> BitterClient {
        BitterClient {
            config: ClientConfig {}
        }
    }
}