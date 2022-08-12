use serde::Deserialize;

const API: &str = "https://hypixel.cactive.network/api/v3";

/// Hello
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
pub struct APIKeyData {
    pub success: bool,
    pub id: String,
    pub data: Option<KeyData>,
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
