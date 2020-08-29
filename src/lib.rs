use reqwest::Client;

const BASE_URL: &'static str = "https://api.shodan.io";

pub struct ShodanClient {
    api_key: &'static str,
    client: Client,
}

pub fn new(api_key: &'static str) -> ShodanClient {
    ShodanClient { api_key, client: Client::new()}
}
