use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Opportunity {
	#[serde(rename = "OPPORTUNITY_ID")]
	pub opportunity_id: u64,
	#[serde(rename = "OPPORTUNITY_NAME")]
	pub opportunity_name: Option<String>,
	#[serde(rename = "OPPORTUNITY_DETAILS")]
	pub opportunity_details: Option<String>,
	#[serde(rename = "OPPORTUNITY_STATE")]
	pub opportunity_state: Option<String>,
	#[serde(rename = "RESPONSIBLE_USER_ID")]
	pub responsible_user_id: Option<u64>,
	#[serde(rename = "CATEGORY_ID")]
	pub category_id: Option<u64>,
	#[serde(rename = "IMAGE_URL")]
	pub image_url: Option<String>,
	#[serde(rename = "BID_CURRENCY")]
	pub bid_currency: Option<String>,
	#[serde(rename = "BID_AMOUNT")]
	pub bid_amount: Option<f64>,
	#[serde(rename = "BID_TYPE")]
	pub bid_type: Option<String>,
	#[serde(rename = "BID_DURATION")]
	pub bid_duration: Option<String>,
	#[serde(rename = "ACTUAL_CLOSE_DATE")]
	pub actual_close_date: Option<String>,
	#[serde(rename = "DATE_CREATED_UTC")]
	pub date_created_utc: Option<String>,
	#[serde(rename = "DATE_UPDATED_UTC")]
	pub date_updated_utc: Option<String>,
	#[serde(rename = "OPPORTUNITY_VALUE")]
	pub opportunity_value: Option<f64>,
	#[serde(rename = "PROBABILITY")]
	pub probability: Option<f64>,
	#[serde(rename = "FORECAST_CLOSE_DATE")]
	pub forecast_close_date: Option<String>,
	#[serde(rename = "OWNER_USER_ID")]
	pub owner_user_id: Option<u64>,
	#[serde(rename = "LAST_ACTIVITY_DATE_UTC")]
	pub last_activity_date_utc: Option<String>,
	#[serde(rename = "NEXT_ACTIVITY_DATE_UTC")]
	pub next_activity_date_utc: Option<String>,
	#[serde(rename = "PIPELINE_ID")]
	pub pipeline_id: Option<u64>,
	#[serde(rename = "STAGE_ID")]
	pub stage_id: Option<u64>,
	#[serde(rename = "CREATED_USER_ID")]
	pub created_user_id: Option<u64>,
	#[serde(rename = "ORGANISATION_ID")]
	pub organisation_id: Option<u64>,
	#[serde(rename = "CUSTOMFIELDS")]
	pub customfields: Option<Vec<OpportunityCustomField>>,
	#[serde(rename = "TAGS")]
	pub tags: Option<Vec<OpportunityTag>>,
	#[serde(rename = "LINKS")]
	pub links: Option<Vec<OpportunityLink>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpportunityTag {
	#[serde(rename = "TAG_NAME")]
	pub tag_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpportunityCustomField {
	#[serde(rename = "FIELD_NAME")]
	pub field_name: Option<String>,
	#[serde(rename = "FIELD_VALUE")]
	pub field_value: Option<OpportunityCustomFieldValue>,
	#[serde(rename = "CUSTOM_FIELD_ID")]
	pub custom_field_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum OpportunityCustomFieldValue {
	Bool(bool),
	String(String),
	Number(f64),
	VecString(Vec<String>),
	VecNumber(Vec<f64>),
	VecBool(Vec<bool>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpportunityLink {
	#[serde(rename = "DETAILS")]
	pub details: Option<String>,
	#[serde(rename = "ROLE")]
	pub role: Option<String>,
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
}
