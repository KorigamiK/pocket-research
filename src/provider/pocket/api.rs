use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::serialize::{
    bool_from_int_string, from_str, option_string_date_unix_timestamp_format,
    optional_vec_from_map, string_date_unix_timestamp_format, to_comma_delimited_string,
    try_url_from_string, vec_from_map,
};

#[derive(Serialize)]
struct PocketOAuthRequest<'a> {
    consumer_key: &'a str,
    redirect_uri: &'a str,
    state: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
struct PocketOAuthResponse {
    code: String,
    #[allow(dead_code)]
    state: Option<String>,
}

#[derive(Serialize)]
struct PocketAuthorizeRequest<'a> {
    consumer_key: &'a str,
    code: &'a str,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct PocketAuthorizeResponse {
    access_token: String,
    username: String,
    state: Option<String>,
}

/// Authenticate with Pocket
/// Returns the access token
pub async fn login(
    client: &reqwest::Client,
    consumer_key: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let body = PocketOAuthRequest {
        consumer_key,
        redirect_uri: "0.0.0.0",
        state: Some("pocket-research"),
    };
    let req = client
        .post("https://getpocket.com/v3/oauth/request")
        .json(&body)
        .header("X-Accept", "application/json")
        .send()
        .await?;
    let resp = req.json::<PocketOAuthResponse>().await?;
    let code = resp.code;

    let authorize_url = {
        let params = vec![
            ("request_token", code.clone()),
            ("redirect_uri", "0.0.0.0".into()),
        ];
        let mut url = Url::parse("https://getpocket.com/auth/authorize").unwrap();
        url.query_pairs_mut().extend_pairs(params.into_iter());
        url
    };

    println!("Follow the url to provide access:\n{}", authorize_url);
    println!("Press enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());

    let body = &PocketAuthorizeRequest {
        consumer_key,
        code: &code,
    };

    let req = client
        .post("https://getpocket.com/v3/oauth/authorize")
        .json(&body)
        .header("X-Accept", "application/json")
        .send()
        .await?;

    let resp: PocketAuthorizeResponse = req.json().await?;
    println!("{:?}", resp);

    Ok(resp.access_token)
}

#[derive(Serialize)]
pub struct PocketRequest<'a, T> {
    consumer_key: &'a str,
    access_token: &'a str,
    #[serde(flatten)]
    request: T,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "lowercase")]
enum PocketGetState {
    Unread,
    Archive,
    All,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "lowercase")]
enum PocketGetSort {
    Newest,
    Oldest,
    Title,
    Site,
}

#[derive(Serialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "lowercase")]
enum PocketGetDetail {
    Simple,
    Complete,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
struct PocketGetParams {
    state: Option<PocketGetState>,
    sort: Option<PocketGetSort>,
    detail_type: Option<PocketGetDetail>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct PocketGetResponse {
    #[serde(deserialize_with = "vec_from_map")]
    list: Vec<PocketItem>,
    status: u16,
    error: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum PocketItemStatus {
    #[serde(rename = "0")]
    Normal,
    #[serde(rename = "1")]
    Archived,
    #[serde(rename = "2")]
    Deleted,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ItemTag {
    #[serde(deserialize_with = "from_str")]
    pub item_id: u64,
    pub tag: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct PocketItem {
    #[serde(deserialize_with = "from_str")]
    pub item_id: u64,
    #[serde(default, deserialize_with = "try_url_from_string")]
    pub given_url: Option<Url>,
    pub given_title: String,

    #[serde(deserialize_with = "string_date_unix_timestamp_format")]
    pub time_added: DateTime<Utc>,
    #[serde(deserialize_with = "option_string_date_unix_timestamp_format")]
    pub time_read: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "string_date_unix_timestamp_format")]
    pub time_updated: DateTime<Utc>,
    #[serde(deserialize_with = "option_string_date_unix_timestamp_format")]
    pub time_favorited: Option<DateTime<Utc>>,

    #[serde(deserialize_with = "bool_from_int_string")]
    pub favorite: bool,

    #[serde(deserialize_with = "from_str")]
    pub resolved_id: u64,
    pub resolved_title: Option<String>,
    #[serde(default, deserialize_with = "try_url_from_string")]
    pub resolved_url: Option<Url>,

    pub sort_id: u64,

    pub status: PocketItemStatus,

    #[serde(default, deserialize_with = "optional_vec_from_map")]
    pub tags: Option<Vec<ItemTag>>,
    pub excerpt: Option<String>,

    pub lang: Option<String>,
    pub time_to_read: Option<u64>,
    pub listen_duration_estimate: Option<u64>,
    #[serde(default, deserialize_with = "try_url_from_string")]
    pub amp_url: Option<Url>,
    #[serde(default, deserialize_with = "try_url_from_string")]
    pub top_image_url: Option<Url>,
}

pub async fn get(
    access_token: &str,
    consumer_key: &str,
    client: &reqwest::Client,
) -> Result<Vec<PocketItem>, Box<dyn std::error::Error>> {
    let body = &PocketRequest {
        access_token,
        consumer_key,
        request: PocketGetParams {
            state: Some(PocketGetState::All),
            sort: Some(PocketGetSort::Oldest),
            detail_type: Some(PocketGetDetail::Complete),
        },
    };

    let req = client
        .post("https://getpocket.com/v3/get")
        .json(&body)
        .header("X-Accept", "application/json")
        .send()
        .await?;

    let resp_json: PocketGetResponse = req.json().await?;
    // let resp_map = req.json::<serde_json::Map<String, Value>>().await?;
    // println!("{:#?}", resp_map);
    Ok(resp_json.list)
}

#[derive(Serialize, Debug, Clone)]
pub struct PocketAddRequest<'a> {
    pub url: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'a str>,
    #[serde(serialize_with = "to_comma_delimited_string")]
    pub tags: Option<&'a [&'a str]>,
}

pub async fn add(
    client: &reqwest::Client,
    access_token: &str,
    consumer_key: &str,
    add_request: PocketAddRequest<'_>,
) -> Result<i64, Box<dyn std::error::Error>> {
    println!("Starting Pocket add request");

    let body = &PocketRequest {
        access_token,
        consumer_key,
        request: add_request,
    };

    let response = client
        .post("https://getpocket.com/v3/add")
        .json(&body)
        .header("X-Accept", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        println!("Successfully added item to Pocket");
        // get the item.item_id from the response
        let item_id = response.json::<serde_json::Value>().await?;
        let item_id = item_id.get("item").ok_or("item object not found in response")?;
        let item_id  = item_id.get("item_id").ok_or("item_id not found in response")?;
        let item_id = item_id.as_str().ok_or("item_id not a string")?;
        Ok(item_id.parse()?)
    } else {
        let error_message = format!(
            "Failed to add item to Pocket. Status: {}",
            response.status()
        );
        println!("{}", error_message);
        Err(error_message.into())
    }
}
