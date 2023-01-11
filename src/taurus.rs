use crate::config::Wallet;
use anyhow::bail;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use std::time::Duration;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct NodeInfo {
    pub name: String,
    pub version: String,
    pub runtime_environment: String,
    pub id: String,
    pub commit: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    id: String,
    provider: String,
    #[serde(rename(deserialize = "type"))]
    score_type: String,
    score: String,
    update_date: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyInfo {
    name: String,
    symbol: String,
    blockchain: String,
    decimals: String,
    contract_address: Option<String>,
    is_u_t_x_o_based: Option<bool>,
    enabled: bool,
    id: String,
    display_name: String,
    #[serde(rename(deserialize = "type"))]
    currency_type: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct WalletInfo {
    pub id: String,
    pub balance: Balance,
    pub currency: String,
    pub coin: String,
    pub name: String,
    pub container: Option<String>,
    pub account_path: String,
    pub is_omnibus: Option<bool>,
    pub creation_date: String,
    pub update_date: String,
    pub blockchain: String,
    pub currency_info: CurrencyInfo,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    key: String,
    value: String,
    id: String,
    content_type: String,
    owner: String,
    #[serde(rename(deserialize = "type"))]
    attribute_type: String,
    subtype: String,
    isfile: bool,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_confirmed: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub total_unconfirmed: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub available_confirmed: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub available_unconfirmed: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub reserved_confirmed: u128,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub reserved_unconfirmed: u128,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Addresses {
    pub id: String,
    pub wallet_id: String,
    pub address_path: String,
    pub address: String,
    pub label: String,
    pub signature: String,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct WalletResponse {
    pub result: Option<Vec<WalletInfo>>,
    pub total_items: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct AddressesResponse {
    pub result: Option<Vec<Addresses>>,
    pub total_items: Option<String>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub result: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq, Default)]
pub struct TokenParams {
    pub email: String,
    pub password: String,
    pub totp: Option<String>,
    pub username: Option<String>,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub sequence: String,
    pub account_number: String,
}

#[derive(Serialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestParams {
    pub chain_id: String,
    pub signers: Vec<u16>,
    pub broadcast_kind: String,
    pub fee_denom: String,
    pub gas_limit: String,
    pub fee: String,
    pub accounts_info: Vec<AccountInfo>,
    pub messages: Vec<crate::payload::Message>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct SignedRequests {
    pub id: String,
    pub signed_request: String,
    pub status: String,
    pub creation_date: String,
    pub update_date: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trails {
    pub user_id: String,
    pub external_user_id: String,
    pub action: String,
    pub date: Option<String>,
    pub request_status: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestInfos {
    pub id: String,
    pub tenant_id: String,
    pub currency: String,
    pub envelope: String,
    pub status: String,
    #[serde(rename(deserialize = "type"))]
    pub type_request: String,
    pub signed_requests: Option<Vec<SignedRequests>>,
    pub trails: Vec<Trails>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct RequestResponse {
    pub result: RequestInfos,
}

pub struct Taurus {
    address: String,
    client: Client,
    token: Option<String>,
}

impl Taurus {
    pub fn new(cfg: &crate::config::Taurus) -> Result<Self, anyhow::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(120))
            .build()?;

        let mut taurus = Taurus {
            address: cfg.api_url.clone(),
            client,
            token: None,
        };

        taurus.login(cfg.mail.as_str(), cfg.passwd.as_str())?;

        Ok(taurus)
    }

    pub fn login(&mut self, email: &str, password: &str) -> Result<(), anyhow::Error> {
        let token = self.token(TokenParams {
            email: email.to_string(),
            password: password.to_string(),
            ..Default::default()
        })?;

        log::info!("Token generated");

        self.token = Some(format!("Bearer {}", token.result));

        Ok(())
    }

    fn get<T: serde::de::DeserializeOwned + Clone>(
        &self,
        endpoint: &str,
    ) -> Result<T, anyhow::Error> {
        log::debug!("GET {}", endpoint);
        let mut request_builder = self.client.get(format!("{}{}", self.address, endpoint));

        if let Some(bearer) = self.token.clone() {
            request_builder = request_builder.header("Authorization", bearer);
        }
        let request = request_builder.send()?;

        let data = &request.text()?;
        log::trace!("-> payload\n{}", data);

        let output = serde_json::from_str::<T>(data);

        Ok(output.unwrap())
    }

    fn post<T: serde::de::DeserializeOwned + Clone, U: serde::ser::Serialize + Clone>(
        &self,
        endpoint: &str,
        data: &U,
    ) -> Result<T, anyhow::Error> {
        log::debug!("POST {}", endpoint);
        let body = serde_json::to_string(data)?;
        log::debug!("\t Body {}", body);
        let mut request_builder = self
            .client
            .post(format!("{}{}", self.address, endpoint))
            .body(body)
            .header("Content-Type", "application/json");
        if let Some(bearer) = self.token.clone() {
            request_builder = request_builder.header("Authorization", bearer);
        }

        let request = request_builder.send()?;

        let data = &request.text()?;
        log::trace!("-> payload\n{}", data);

        let output = serde_json::from_str::<T>(data);

        Ok(output.unwrap())
    }

    fn token(&self, params: TokenParams) -> Result<Token, anyhow::Error> {
        self.post("/api/rest/v1/authentication/token", &params)
    }

    pub fn addresses(&self) -> Result<AddressesResponse, anyhow::Error> {
        self.get("/api/rest/v1/addresses")
    }

    pub fn addresses_by_address(&self, wallet: Wallet) -> Result<Addresses, anyhow::Error> {
        let addresses = self.addresses()?;

        if addresses.result.is_none() {
            bail!("no matching addresses");
        }

        let addresses = addresses.result.unwrap();
        let pos = addresses.iter().position(|x| x.address == wallet.address);

        if pos.is_none() {
            bail!("no matching addresses");
        }

        Ok(addresses[pos.unwrap()].clone())
    }

    pub fn request(&self, params: RequestParams) -> Result<RequestResponse, anyhow::Error> {
        self.post(
            "/api/rest/v1/requests/outgoing/cosmos/generic_request",
            &params,
        )
    }

    pub fn request_by_id(&self, id: u64) -> Result<RequestResponse, anyhow::Error> {
        self.get(format!("/api/rest/v1/requests/{}", id).as_str())
    }
}
