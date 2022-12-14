#![doc = include_str!("../README.md")]

use serde::{de::DeserializeOwned, Deserialize};

const API: &str = "https://hypixel.cactive.network/api/v3";

pub struct Client {
    key: String,
    cache: bool,
}

#[derive(Deserialize)]
pub struct NicknameHistory {
    pub uuid: String,
    pub nickname: String,
    pub active: bool,
    pub created_at: String,
    pub voided_at: String,
}

#[derive(Deserialize)]
pub struct PunishmentData {
    pub id: String,
    pub punishment_type: String,
    pub uuid: String,
    pub executor: Option<String>,
    pub reason: String,
    pub length: Option<u32>,
}

#[derive(Deserialize)]
pub struct PlayerDataNicknameHistory {
    pub nickname: String,
    pub active: Option<bool>,
    pub created_at: String,
    pub voided_at: Option<String>,
}

#[derive(Deserialize)]
pub struct PlayerDataInfractions {
    pub id: String,
    pub punishment_type: String,
    pub executor: Option<String>,
    pub reason: String,
    pub length: Option<u32>,
}

#[derive(Deserialize)]
pub struct PlayerDataTracker {
    pub server: Option<String>,
    pub map: Option<String>,
    pub proxy: Option<String>,
    pub last_login: Option<String>,
}

#[derive(Deserialize)]
pub struct PlayerDataIPHistory {
    pub ip: String,
    pub login_at: String,
    pub logout_at: Option<String>,
    pub connection_proxy: Option<String>,
}

#[derive(Deserialize)]
pub struct PlayerData {
    pub uuid: String,
    pub nickname_history: Vec<PlayerDataNicknameHistory>,
    pub infractions: Vec<PlayerDataInfractions>,
    pub tracker: PlayerDataTracker,
    pub ip_history: Option<Vec<PlayerDataIPHistory>>,
}

#[derive(Deserialize)]
pub struct KeyEndpoints {
    pub id: String,
    pub version: i8,
    pub status: bool,
}

#[derive(Deserialize)]
pub struct KeyData {
    pub key: String,
    pub valid: bool,
    pub active: bool,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub owner_cactiveconnections_id: Option<String>,
    pub endpoints: Vec<KeyEndpoints>,
}

#[derive(Deserialize)]
pub struct StaffTracker {
    pub uuid: String,
    pub rank: String,
    pub online: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct InternalError {
    pub r#type: String,
    pub code: u16,
    pub message: String,
    pub internal: bool,
}

#[derive(Deserialize, Clone)]
pub struct APIError {
    pub r#type: String,
    pub code: u16,
    pub message: String,
}

#[derive(Deserialize)]
pub struct APIData<T> {
    pub success: bool,
    pub id: String,
    pub data: Option<T>,
    pub errors: Option<Vec<APIError>>,
}

impl From<APIError> for InternalError {
    fn from(error: APIError) -> Self {
        InternalError {
            r#type: error.r#type,
            code: error.code,
            message: error.message,
            internal: false,
        }
    }
}

impl From<reqwest::Error> for InternalError {
    fn from(error: reqwest::Error) -> Self {
        InternalError {
            r#type: "failed-api-request".to_owned(),
            code: 500,
            message: error.to_string(),
            internal: true,
        }
    }
}

impl Client {
    /// Create a new client, providing a key string and a cache boolean.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use cactive_hypixel_api::Client;
    ///
    /// let key = "my_api_key".to_owned();
    /// let client = Client::new(key, false);
    /// ```
    pub fn new(key: String, cache: bool) -> Self {
        Self { key, cache }
    }

    /// Retrieve an ascending vector of players referenced from the nickname parameter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let nickname = "angry_and_free".to_owned();
    /// let data = match client.nickname_history(nickname).await {
    ///     Ok(data) => data,
    ///     Err(err) => return println!("{}", err[0].message),
    /// };
    /// ```
    pub async fn nickname_history(
        &self,
        nickname: String,
    ) -> Result<Vec<NicknameHistory>, Vec<InternalError>> {
        Self::request_data(format!(
            "{API}/nickname-history?key={}&cache={}&nickname={nickname}",
            self.key, self.cache,
        ))
        .await
    }

    /// Retrieve a structure of player data, providing a uuid parameter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let uuid = "eea2d4fd-a8b8-413b-9439-f06faaf7e109".to_owned();
    /// let data = match client.player_data(uuid).await {
    ///     Ok(data) => data,
    ///     Err(err) => return println!("{}", err[0].message),
    /// };
    /// ```
    pub async fn player_data(&self, uuid: String) -> Result<PlayerData, Vec<InternalError>> {
        Self::request_data(format!(
            "{API}/player-data?key={}&cache={}&uuid={uuid}",
            self.key, self.cache,
        ))
        .await
    }

    /// Retrieve an ascending vector of Hypixel staff providing a filter ("all", "online", "offline") parameter.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let filter = "online".to_owned();
    /// let data = match client.staff_tracker(filter).await {
    ///     Ok(data) => data,
    ///     Err(err) => return println!("{}", err[0].message),
    /// };
    /// ```
    pub async fn staff_tracker(
        &self,
        filter: String,
    ) -> Result<Vec<StaffTracker>, Vec<InternalError>> {
        Self::request_data(format!(
            "{API}/staff-tracker?key={}&cache={}&filter={filter}",
            self.key, self.cache,
        ))
        .await
    }

    /// Retrieve a structure of punishment data, providing an ID parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// let id = "C256D602".to_owned();
    /// let data = match client.punishment_data(id).await {
    ///     Ok(data) => data,
    ///     Err(err) => return println!("{}", err[0].message),
    /// };
    /// ```
    pub async fn punishment_data(&self, id: String) -> Result<PunishmentData, Vec<InternalError>> {
        Self::request_data(format!(
            "{API}/staff-tracker?key={}&cache={}&id={id}",
            self.key, self.cache,
        ))
        .await
    }

    /// Retrieve the key data of the provided an key parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// let key = "my_api_key".to_owned();
    /// let data = match client.key_data(key).await {
    ///     Ok(data) => data,
    ///     Err(err) => return println!("{}", err[0].message),
    /// };
    /// ```
    pub async fn key_data(&self, key: String) -> Result<KeyData, Vec<InternalError>> {
        Self::request_data(format!("{API}/key?key={key}")).await
    }

    async fn request_data<T, S>(url: S) -> Result<T, Vec<InternalError>>
    where
        T: DeserializeOwned,
        S: reqwest::IntoUrl,
    {
        let request = match reqwest::get(url).await {
            Ok(req) => req,
            Err(err) => return Err(vec![err.into()]),
        };
        map_errors(request).await
    }
}

async fn map_errors<T: DeserializeOwned>(
    request: reqwest::Response,
) -> Result<T, Vec<InternalError>> {
    match request.json::<APIData<T>>().await {
        Ok(json) => {
            if json.success {
                Ok(json.data.unwrap())
            } else {
                Err(json
                    .errors
                    .unwrap()
                    .into_iter()
                    .map(|error| error.into())
                    .collect())
            }
        }
        Err(err) => Err(vec![err.into()]),
    }
}

#[tokio::test]
async fn nickname_history_test() {
    let client = Client::new("key".to_owned(), false);
    match client.nickname_history("k".to_owned()).await {
        Ok(_) => println!("Success"),
        Err(error) => println!("Error {}", error[0].message),
    }
}

#[tokio::test]
async fn key_data_test() {
    let client = Client::new("key".to_owned(), false);
    match client.key_data("api".to_owned()).await {
        Ok(data) => println!("Success {}", data.endpoints[0].id),
        Err(error) => println!("Error {}", error[0].message),
    }
}
