#![warn(clippy::pedantic, clippy::nursery, clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

use azure_security_keyvault::KeyvaultClient;
use base64::{
	alphabet, engine::{self, general_purpose}, Engine as _
};
use reqwest::header::HeaderValue;

pub use crate::opportunities::*;
pub use crate::pipeline_stages::*;
pub use crate::projects::*;
pub use crate::users::*;

pub mod opportunities;
pub mod pipeline_stages;
pub mod projects;
pub mod users;

pub struct Insightly {
	pub api_key_base64: String,
}

impl Insightly {
	/// # Errors
	/// todo
	pub async fn new() -> Result<Self, String> {
		const CUSTOM_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

		let azure_credentials = azure_identity::create_credential().map_err(|_| "Error creating Azure Credentials.".to_string())?;
		let azure_key_vault_client = KeyvaultClient::new("https://eggappserverkeyvault.vault.azure.net", azure_credentials).map_err(|_| "Error creating Key Vault Client.".to_string())?;
		let api_key = match azure_key_vault_client.secret_client().get("insightly-api-key").await {
			Ok(insightly_key) => insightly_key.value,
			Err(e) => return Err(format!("Error: {e:?}")),
		};
		let api_key_base64 = CUSTOM_ENGINE.encode(api_key.as_bytes());

		Ok(Self { api_key_base64 })
	}

	/// # Errors
	/// todo
	pub async fn get_user(&self, id: String) -> Result<users::User, String> {
		let client = reqwest::Client::new();
		let mut headers = reqwest::header::HeaderMap::new();
		let header_value = match HeaderValue::from_str(&("Basic ".to_owned() + self.api_key_base64.as_str())) {
			Ok(header_value) => header_value,
			Err(e) => return Err(format!("Error: {e:?}")),
		};
		headers.insert(reqwest::header::AUTHORIZATION, header_value);
		let uri = format!("https://api.insightly.com/v3.1/Users/{id}");

		let res = match client.get(uri).headers(headers).send().await {
			Ok(res) => res,
			Err(e) => return Err(format!("Error: {e:?}")),
		};

		let user: User = match res.json().await {
			Ok(user) => user,
			Err(e) => return Err(format!("Error: {e:?}")),
		};

		Ok(user)
	}

	/// # Errors
	/// todo
	pub async fn get_opportunity_list(&self) -> Result<Vec<Opportunity>, String> {
		let mut opportunities = Vec::new();
		let mut skip = 0;

		loop {
			let client = reqwest::Client::new();
			let mut headers = reqwest::header::HeaderMap::new();
			let header_value = match HeaderValue::from_str(&("Basic ".to_owned() + self.api_key_base64.as_str())) {
				Ok(header_value) => header_value,
				Err(e) => return Err(format!("Error: {e:?}")),
			};
			headers.insert(reqwest::header::AUTHORIZATION, header_value);

			let uri = format!("https://api.insightly.com/v3.1/Opportunities?brief=false&skip={skip}&top=500&count_total=true");
			let res = match client.get(uri).headers(headers).send().await {
				Ok(res) => res,
				Err(e) => return Err(format!("Error: {e:?}")),
			};

			let opportunity_temp: Vec<Opportunity> = match res.json().await {
				Ok(opportunity_list) => opportunity_list,
				Err(e) => return Err(format!("Error: {e:?}")),
			};

			if opportunity_temp.is_empty() {
				break;
			}

			opportunities.extend(opportunity_temp);
			skip += 500;
		}

		Ok(opportunities)
	}

	/// # Errors
	/// todo
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
