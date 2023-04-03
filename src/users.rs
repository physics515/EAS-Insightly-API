use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
	#[serde(rename = "USER_ID")]
	user_id: u64,

	#[serde(rename = "CONTACT_ID")]
	contact_id: Option<u64>,

	#[serde(rename = "FIRST_NAME")]
	first_name: Option<String>,

	#[serde(rename = "LAST_NAME")]
	last_name: Option<String>,

	#[serde(rename = "TIMEZONE_ID")]
	timezone_id: Option<String>,

	#[serde(rename = "EMAIL_ADDRESS")]
	email_address: Option<String>,

	#[serde(rename = "EMAIL_DROPBOX_IDENTIFIER")]
	email_dropbox_identifier: Option<String>,

	#[serde(rename = "EMAIL_DROPBOX_ADDRESS")]
	email_dropbox_address: Option<String>,

	#[serde(rename = "ADMINISTRATOR")]
	administrator: Option<bool>,

	#[serde(rename = "ACCOUNT_OWNER")]
	account_owner: Option<bool>,

	#[serde(rename = "ACTIVE")]
	active: Option<bool>,

	#[serde(rename = "DATE_CREATED_UTC")]
	date_created_utc: Option<String>,

	#[serde(rename = "DATE_UPDATED_UTC")]
	date_updated_utc: Option<String>,

	#[serde(rename = "USER_CURRENCY")]
	user_currency: Option<String>,

	#[serde(rename = "CONTACT_DISPLAY")]
	contact_display: Option<String>,

	#[serde(rename = "CONTACT_ORDER")]
	contact_order: Option<String>,

	#[serde(rename = "TASK_WEEK_START")]
	task_week_start: Option<u64>,

	#[serde(rename = "INSTANCE_ID")]
	instance_id: Option<u64>,

	#[serde(rename = "PROFILE_ID")]
	profile_id: Option<u64>,

	#[serde(rename = "ROLE_ID")]
	role_id: Option<u64>,
}
