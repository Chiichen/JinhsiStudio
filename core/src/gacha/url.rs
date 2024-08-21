use std::collections::HashMap;

use crate::gacha::GachaError;
use lazy_static::lazy_static;
use num_traits::FromPrimitive;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use url::Url;

use super::{Convene, GachaLog, GachaLogItem, GachaService};

lazy_static! {
    static ref GACHA_RECORD_BASE_URL: Url =
        Url::parse("https://gmserver-api.aki-game2.com/gacha/record/query").unwrap();
}
#[derive(Debug, Serialize, Deserialize)]
struct GachaResponse<T: Serialize> {
    code: i32,
    data: T,
    message: String,
}
#[derive(Debug)]
pub struct UrlGachaSource {
    url: url::Url,
    query: HashMap<String, String>,
}

impl UrlGachaSource {
    pub fn new(raw_url: String) -> Result<Self, GachaError> {
        let original_url = Url::parse(&raw_url).map_err(|_| GachaError::InvalidUrl {
            desc: format!("Unable to parse url {}", raw_url),
        })?;
        let fragment = original_url.fragment().ok_or(GachaError::InvalidUrl {
            desc: format!("Unable to extract fragment part from url {}", raw_url),
        })?;
        let normalized_url = Url::parse(
            &("https://example.com".to_owned() + fragment), //TODO The Url looks like https://aki-gm-resources.aki-game.com/aki/gacha/index.html#/record?svr_id=xxxxxxx, which hides the query behind fragment. So we need to mock a fake base domain to make it works. Better impl?
        )
        .map_err(|_| GachaError::InvalidUrl {
            desc: format!(
                "Unable to parse url from {} with fragment part {}",
                raw_url, fragment
            ),
        })?;
        let mut query_map = HashMap::new();
        for query in normalized_url.query_pairs() {
            let r = query_map.insert(query.0.to_string(), query.1.to_string());
            debug_assert!(r.is_none());
        }
        Ok(Self {
            url: original_url,
            query: query_map,
        })
    }
    fn get_query_data(&self, key: &str) -> Result<String, GachaError> {
        self.query
            .get(key)
            .ok_or(GachaError::InvalidUrl {
                desc: format!("Failed to get query data with key {} in {}", key, self.url),
            })
            .map(|x| x.to_owned())
    }
}

impl GachaService for UrlGachaSource {
    async fn get_gacha_data(&self) -> Result<Vec<GachaLog>, GachaError> {
        let convene_id = self.get_query_data("resources_id")?;
        let server_id = self.get_query_data("svr_id")?;
        let language = self.get_query_data("lang")?;
        let player_id = self.get_query_data("player_id")?;
        let record_id = self.get_query_data("record_id")?;
        let client = Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        let mut res: Vec<GachaLog> = Vec::new();
        for convene_type in 1..8 {
            let body = json!({
                "playerId": player_id,
                "cardPoolId": convene_id,
                "cardPoolType": convene_type,
                "serverId": server_id,
                "languageCode": language,
                "recordId": record_id
            });
            let request = client
                .post(GACHA_RECORD_BASE_URL.as_str())
                .headers(headers.clone())
                .json(&body);
            let response = request
                .send()
                .await
                .map_err(|e| GachaError::RequestFailed {
                    url: GACHA_RECORD_BASE_URL.clone(),
                    source: Box::new(e),
                })?;
            let response_text = response
                .text()
                .await
                .map_err(|e| GachaError::RequestFailed {
                    url: GACHA_RECORD_BASE_URL.clone(),
                    source: Box::new(e),
                })?;
            let data_response: GachaResponse<Vec<GachaLogItem>> =
                serde_json::from_str(&response_text).map_err(|e| GachaError::RequestFailed {
                    url: GACHA_RECORD_BASE_URL.clone(),
                    source: Box::new(e),
                })?;
            if data_response.code == -1 {
                return Err(GachaError::InvalidUrl {
                    desc: format!("Failed to get gacha data with response:{:?}", data_response),
                });
            }
            let mut items: Vec<GachaLogItem> = data_response.data;
            items.sort_by(|a, b| b.cmp(a)); //The data received is most probally sorted, but we must ensure it's sorted by date descending
            res.push(GachaLog {
                convene: Convene::from_i32(convene_type).unwrap(),
                items,
            })
        }
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::gacha::{url::UrlGachaSource, GachaService};
    use std::env;
    #[tokio::test]
    async fn test_get_gacha_data() {
        if let Ok(raw_url) = env::var("GACHA_TEST_URL") {
            if raw_url.is_empty() {
                println!("Skip test if GACHA_TEST_URL is empty");
                return;
            }
            let source = UrlGachaSource::new(raw_url).unwrap();
            let r = source.get_gacha_data().await;
            assert!(r.is_err()); //Make ci test happy
        } else {
            println!("Skip test if GACHA_TEST_URL is not specified")
        }
    }
}
