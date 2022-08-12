use serde::Deserialize;

const API: &str = "https://hypixel.cactive.network/api/v3";

pub struct Client {
    key: String,
    cache: bool
}

#[derive(Deserialize)]
pub struct NicknameHistory {
    pub uuid: String,
    pub nickname: String,
    pub active: bool,
    pub created_at: String,
    pub voided_at: String
}

#[derive(Deserialize)]
pub struct PunishmentData {
    pub id: String,
    pub punishment_type: String,
    pub uuid: String,
    pub executor: Option<String>,
    pub reason: String,
    pub length: Option<u32>
}

#[derive(Deserialize)]
pub struct PlayerDataNicknameHistory {
    pub nickname: String,
    pub active: Option<bool>,
    pub created_at: String,
    pub voided_at: Option<String>
}

#[derive(Deserialize)]
pub struct PlayerDataInfractions {
    pub id: String,
    pub punishment_type: String,
    pub executor: Option<String>,
    pub reason: String,
    pub length: Option<u32>
}

#[derive(Deserialize)]
pub struct PlayerDataTracker {
    pub server: Option<String>,
    pub map: Option<String>,
    pub proxy: Option<String>,
    pub last_login: Option<String>
}

#[derive(Deserialize)]
pub struct PlayerDataIPHistory {
    pub ip: String,
    pub login_at: String,
    pub logout_at: Option<String>,
    pub connection_proxy: Option<String>
}

#[derive(Deserialize)]
pub struct PlayerData {
    pub uuid: String,
    pub nickname_history: Vec<PlayerDataNicknameHistory>,
    pub infractions: Vec<PlayerDataInfractions>,
    pub tracker: PlayerDataTracker,
    pub ip_history: Option<Vec<PlayerDataIPHistory>>
}

#[derive(Deserialize)]
pub struct KeyEndpoints {
    pub id: String,
    pub version: i8,
    pub status: bool
}

#[derive(Deserialize)]
pub struct KeyData {
    pub key: String,
    pub valid: bool,
    pub active: bool,
    pub created_at: Option<String>,
    pub expires_at: Option<String>,
    pub owner_cactiveconnections_id: Option<String>,
    pub endpoints: Vec<KeyEndpoints>
}

#[derive(Deserialize)]
pub struct StaffTracker {
    pub uuid: String,
    pub rank: String,
    pub online: Option<bool>
}

#[derive(Deserialize)]
pub struct InternalError {
    pub r#type: String,
    pub code: u16,
    pub message: String,
    pub internal: bool
}

#[derive(Deserialize, Clone)]
pub struct APIError {
    pub r#type: String,
    pub code: u16,
    pub message: String
}

#[derive(Deserialize)]
pub struct APINicknameResponse {
    pub success: bool,
    pub id: String,
    pub data: Option<Vec<NicknameHistory>>,
    pub errors: Option<Vec<APIError>>
}

#[derive(Deserialize)]
pub struct APIPlayerData {
    pub success: bool,
    pub id: String,
    pub data: Option<PlayerData>,
    pub errors: Option<Vec<APIError>>
}

#[derive(Deserialize)]
pub struct APIKeyData {
    pub success: bool,
    pub id: String,
    pub data: Option<KeyData>,
    pub errors: Option<Vec<APIError>>
}

#[derive(Deserialize)]
pub struct APIStaffTracker {
    pub success: bool,
    pub id: String,
    pub data: Option<Vec<StaffTracker>>,
    pub errors: Option<Vec<APIError>>
}

#[derive(Deserialize)]
pub struct APIPunishmentData {
    pub success: bool,
    pub id: String,
    pub data: Option<PunishmentData>,
    pub errors: Option<Vec<APIError>>
}

impl Client {
    pub fn new(key: String, cache: bool) -> Self {
        Self {
            key,
            cache
        }
    }

    fn map_internal_error(error: String) -> Vec<InternalError> {
        Vec::from([
            InternalError {
                r#type: "failed-api-request".to_owned(),
                code: 500,
                message: error.to_string(),
                internal: true
            }
        ])
    }

    fn map_external_error(errors: Vec<APIError>) -> Vec<InternalError> {
        errors.iter().map(|error| InternalError {
            r#type: error.r#type.clone(),
            code: error.code,
            message: error.message.clone(),
            internal: false
        }).collect()
    }

    pub async fn nickname_history(&self, nickname: String) -> Result<Vec<NicknameHistory>, Vec<InternalError>> {
        let request = match reqwest::get(format!(
            "{}/nickname-history?key={}&cache={}&nickname={}",
            API.to_owned(), self.key, self.cache, nickname
        )).await {
            Ok(req) => req,
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        };
        match request.json::<APINicknameResponse>().await {
            Ok(json) => {
                if json.success {
                    Ok(json.data.unwrap())
                } else {
                    Err(Client::map_external_error(json.errors.unwrap()))
                }
            },
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        }
    }

    pub async fn player_data(&self, uuid: String) -> Result<PlayerData, Vec<InternalError>> {
        let request = match reqwest::get(format!(
            "{}/player-data?key={}&cache={}&uuid={}",
            API.to_owned(), self.key, self.cache, uuid
        )).await {
            Ok(req) => req,
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        };
        match request.json::<APIPlayerData>().await {
            Ok(json) => {
                if json.success {
                    Ok(json.data.unwrap())
                } else {
                    Err(Client::map_external_error(json.errors.unwrap()))
                }
            },
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        }
    }

    pub async fn staff_tracker(&self) -> Result<Vec<StaffTracker>, Vec<InternalError>> {
        let request = match reqwest::get(format!(
            "{}/staff-tracker?key={}&cache={}",
            API.to_owned(), self.key, self.cache
        )).await {
            Ok(req) => req,
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        };
        match request.json::<APIStaffTracker>().await {
            Ok(json) => {
                if json.success {
                    Ok(json.data.unwrap())
                } else {
                    Err(Client::map_external_error(json.errors.unwrap()))
                }
            },
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        }
    }

    pub async fn punishment_data(&self, id: String) -> Result<PunishmentData, Vec<InternalError>> {
        let request = match reqwest::get(format!(
            "{}/staff-tracker?key={}&cache={}&id={}",
            API.to_owned(), self.key, self.cache, id
        )).await {
            Ok(req) => req,
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        };
        match request.json::<APIPunishmentData>().await {
            Ok(json) => {
                if json.success {
                    Ok(json.data.unwrap())
                } else {
                    Err(Client::map_external_error(json.errors.unwrap()))
                }
            },
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        }
    }

    pub async fn key_data(&self) -> Result<KeyData, Vec<InternalError>> {
        let request = match reqwest::get(format!(
            "{}/key?key={}",
            API.to_owned(), self.key
        )).await {
            Ok(req) => req,
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        };
        match request.json::<APIKeyData>().await {
            Ok(json) => {
                if json.success {
                    Ok(json.data.unwrap())
                } else {
                    Err(Client::map_external_error(json.errors.unwrap()))
                }
            },
            Err(err) => return Err(Client::map_internal_error(err.to_string()))
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new("key".to_owned(), false);

    match client.nickname_history("k".to_owned()).await {
        Ok(_) => println!("Success"),
        Err(error) => println!("Error {}", error[0].message)
    }

    match client.key_data().await {
        Ok(data) => println!("Success {}", data.endpoints[0].id),
        Err(error) => println!("Error {}", error[0].message)
    }
}
