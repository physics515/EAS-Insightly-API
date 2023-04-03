use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineStage {
	#[serde(rename = "STAGE_ID")]
	pub stage_id: u64,
	#[serde(rename = "PIPELINE_ID")]
	pub pipeline_id: u64,
	#[serde(rename = "STAGE_NAME")]
	pub stage_name: String,
	#[serde(rename = "STAGE_ORDER")]
	pub stage_order: u64,
	#[serde(rename = "ACTIVITYSET_ID")]
	pub activityset_id: Option<u64>,
	#[serde(rename = "OWNER_USER_ID")]
	pub owner_user_id: Option<u64>,
}
