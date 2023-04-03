use azure_identity::ImdsManagedIdentityCredential;
use azure_security_keyvault::KeyvaultClient;
use base64::{
	alphabet,
	engine::{self, general_purpose},
	Engine as _,
};
use reqwest::header::HeaderValue;
use std::sync::Arc;

pub use crate::opportunities::*;
pub use crate::pipeline_stages::*;
pub use crate::users::*;

mod opportunities;
mod pipeline_stages;
mod users;

pub struct Insightly {
	pub api_key_base64: String,
}

impl Insightly {
	pub async fn new() -> Self {
		let azure_credentials = ImdsManagedIdentityCredential::default();
		let azure_key_vault_client = KeyvaultClient::new("https://eggappserverkeyvault.vault.azure.net", Arc::new(azure_credentials)).unwrap();
		let api_key = match azure_key_vault_client.secret_client().get("insightly-api-key").await {
			Ok(insightly_key) => insightly_key.value,
			Err(e) => panic!("Error: {e:?}"),
		};

		const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
		let api_key_base64 = CUSTOM_ENGINE.encode(api_key.as_bytes());

		Self { api_key_base64 }
	}

	pub async fn get_user(&self, id: String) -> Result<users::User, String> {
		let client = reqwest::Client::new();
		let mut headers = reqwest::header::HeaderMap::new();
		let header_value = match HeaderValue::from_str(&("Basic ".to_owned() + self.api_key_base64.as_str())) {
			Ok(header_value) => header_value,
			Err(e) => panic!("Error: {e:?}"),
		};
		headers.insert(reqwest::header::AUTHORIZATION, header_value);
		let uri = format!("https://api.insightly.com/v3.1/Users/{id}");

		let res = match client.get(uri).headers(headers).send().await {
			Ok(res) => res,
			Err(e) => panic!("Error: {e:?}"),
		};

		let user: User = match res.json().await {
			Ok(user) => user,
			Err(e) => panic!("Error: {e:?}"),
		};

		Ok(user)
	}

	pub async fn get_opportunity_list(&self) -> Result<Vec<Opportunity>, String> {
		let mut opportunities = Vec::new();
		let mut skip = 0;

		loop {
			let client = reqwest::Client::new();
			let mut headers = reqwest::header::HeaderMap::new();
			let header_value = match HeaderValue::from_str(&("Basic ".to_owned() + self.api_key_base64.as_str())) {
				Ok(header_value) => header_value,
				Err(e) => panic!("Error: {e:?}"),
			};
			headers.insert(reqwest::header::AUTHORIZATION, header_value);

			let uri = format!("https://api.insightly.com/v3.1/Opportunities?brief=false&skip={skip}&top=500&count_total=true");
			let res = match client.get(uri).headers(headers).send().await {
				Ok(res) => res,
				Err(e) => panic!("Error: {e:?}"),
			};

			let opportunity_temp: Vec<Opportunity> = match res.json().await {
				Ok(opportunity_list) => opportunity_list,
				Err(e) => panic!("Error: {e:?}"),
			};

			if opportunity_temp.is_empty() {
				break;
			} else {
				opportunities.extend(opportunity_temp);
				skip += 500;
			}
		}

		Ok(opportunities)
	}

	pub fn get_pipeline_stage(&self, stage_id: u64) -> Result<PipelineStage, String> {
		let client = reqwest::blocking::Client::new();
		let mut headers = reqwest::header::HeaderMap::new();
		let header_value = match HeaderValue::from_str(&("Basic ".to_owned() + self.api_key_base64.as_str())) {
			Ok(header_value) => header_value,
			Err(e) => return Err(format!("Error: {e:?}")),
		};
		headers.insert(reqwest::header::AUTHORIZATION, header_value);

		let uri = format!("https://api.insightly.com/v3.1/PipelineStages/{stage_id}");
		let res = match client.get(uri).headers(headers).send() {
			Ok(res) => res,
			Err(e) => return Err(format!("Error: {e:?}")),
		};

		let pipeline_stage: PipelineStage = match res.json() {
			Ok(pipeline_stage) => pipeline_stage,
			Err(e) => return Err(format!("Error: {e:?}")),
		};

		Ok(pipeline_stage)
	}
}
