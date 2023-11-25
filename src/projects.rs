use std::collections::HashMap;
use std::fmt::Display;

use rocket::data::{self, Data, FromData, ToByteUnit};
use rocket::request::Request;
use serde::{Deserialize, Serialize};

/* {
  "PROJECT_ID": 0,
  "PROJECT_NAME": "string",
  "STATUS": "string",
  "PROJECT_DETAILS": "string",
  "STARTED_DATE": "2023-04-03T15:39:59.326Z",
  "COMPLETED_DATE": "2023-04-03T15:39:59.326Z",
  "OPPORTUNITY_ID": 0,
  "CATEGORY_ID": 0,
  "PIPELINE_ID": 0,
  "STAGE_ID": 0,
  "IMAGE_URL": "string",
  "OWNER_USER_ID": 0,
  "DATE_CREATED_UTC": "2023-04-03T15:39:59.326Z",
  "DATE_UPDATED_UTC": "2023-04-03T15:39:59.326Z",
  "LAST_ACTIVITY_DATE_UTC": "2023-04-03T15:39:59.326Z",
  "NEXT_ACTIVITY_DATE_UTC": "2023-04-03T15:39:59.326Z",
  "CREATED_USER_ID": 0,
  "RESPONSIBLE_USER_ID": 0,
  "CUSTOMFIELDS": [
	{
	  "FIELD_NAME": "string",
	  "FIELD_VALUE": {}
	}
  ],
  "TAGS": [
	{
	  "TAG_NAME": "string"
	}
  ],
  "LINKS": [
	{
	  "LINK_ID": 0,
	  "OBJECT_NAME": "string",
	  "OBJECT_ID": 0,
	  "LINK_OBJECT_NAME": "string",
	  "LINK_OBJECT_ID": 0,
	  "ROLE": "string",
	  "DETAILS": "string",
	  "RELATIONSHIP_ID": 0,
	  "IS_FORWARD": true
	}
  ]
} */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
	#[serde(rename = "PROJECT_ID")]
	pub project_id: u64,

	#[serde(rename = "PROJECT_NAME")]
	pub project_name: Option<String>,

	#[serde(rename = "STATUS")]
	pub status: Option<String>,

	#[serde(rename = "PROJECT_DETAILS")]
	pub project_details: Option<String>,

	#[serde(rename = "STARTED_DATE")]
	pub start_date: Option<String>,

	#[serde(rename = "COMPLETED_DATE")]
	pub complete_date: Option<String>,

	#[serde(rename = "OPPORTUNITY_ID")]
	pub opportunity_id: Option<u64>,

	#[serde(rename = "CATEGORY_ID")]
	pub category_id: Option<u64>,

	#[serde(rename = "PIPELINE_ID")]
	pub pipeline_id: Option<u64>,

	#[serde(rename = "STAGE_ID")]
	pub stage_id: Option<u64>,

	#[serde(rename = "IMAGE_URL")]
	pub image_url: Option<String>,

	#[serde(rename = "OWNER_USER_ID")]
	pub owner_user_id: Option<u64>,

	#[serde(rename = "DATE_CREATED_UTC")]
	pub date_created_utc: Option<String>,

	#[serde(rename = "DATE_UPDATED_UTC")]
	pub date_updated_utc: Option<String>,

	#[serde(rename = "LAST_ACTIVITY_DATE_UTC")]
	pub last_activity_date_utc: Option<String>,

	#[serde(rename = "NEXT_ACTIVITY_DATE_UTC")]
	pub next_activity_date_utc: Option<String>,

	#[serde(rename = "CREATED_USER_ID")]
	pub created_user_id: Option<u64>,

	#[serde(rename = "RESPONSIBLE_USER_ID")]
	pub responsible_user_id: Option<u64>,

	#[serde(rename = "CUSTOMFIELDS")]
	pub custom_fields: Option<Vec<ProjectCustomField>>,

	#[serde(rename = "TAGS")]
	pub tags: Option<Vec<HashMap<String, String>>>,

	#[serde(rename = "LINKS")]
	pub links: Option<Vec<ProjectLinks>>,
}

impl Display for Project {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let project_json = serde_json::to_string(&self).unwrap();
		write!(f, "{}", project_json)
	}
}

#[rocket::async_trait]
impl<'r> FromData<'r> for Project {
	type Error = String;

	async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
		use rocket::outcome::Outcome::*;

		let limit = req.limits().get("project").unwrap_or(1024_i32.megabytes());

		let string = match data.open(limit).into_string().await {
			Ok(string) if string.is_complete() => string.into_inner(),
			Ok(_) => return Error((rocket::http::Status::PayloadTooLarge, "Payload too large".to_string())),
			Err(_) => return Error((rocket::http::Status::InternalServerError, "Internal Server Error".to_string())),
		};

		let project = match serde_json::from_str::<WorkflowAutomationProject>(&string) {
			Ok(project) => project.entity.clone(),
			Err(e) => return Error((rocket::http::Status::BadRequest, format!("Bad Request: {}", e))),
		};

		Success(project)
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCustomField {
	#[serde(rename = "FIELD_NAME")]
	pub field_name: Option<String>,

	#[serde(rename = "FIELD_VALUE")]
	pub field_value: Option<ProjectCustomFieldValue>,

	#[serde(rename = "CUSTOM_FIELD_ID")]
	pub custom_field_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProjectCustomFieldValue {
	String(String),
	Bool(bool),
	Number(i64),
	Float(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLinks {
	#[serde(rename = "LINK_ID")]
	pub link_id: Option<u64>,

	#[serde(rename = "OBJECT_NAME")]
	pub object_name: Option<String>,

	#[serde(rename = "OBJECT_ID")]
	pub object_id: Option<u64>,

	#[serde(rename = "LINK_OBJECT_NAME")]
	pub link_object_name: Option<String>,

	#[serde(rename = "LINK_OBJECT_ID")]
	pub link_object_id: Option<u64>,

	#[serde(rename = "ROLE")]
	pub role: Option<String>,

	#[serde(rename = "DETAILS")]
	pub details: Option<String>,

	#[serde(rename = "RELATIONSHIP_ID")]
	pub relationship_id: Option<u64>,

	#[serde(rename = "IS_FORWARD")]
	pub is_forward: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAutomationProject {
	pub entity: Project,
}
