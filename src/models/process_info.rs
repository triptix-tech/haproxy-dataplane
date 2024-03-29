/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessInfo {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<crate::models::ProcessInfoItem>>,
    #[serde(rename = "runtimeAPI", skip_serializing_if = "Option::is_none")]
    pub runtime_api: Option<String>,
}

impl ProcessInfo {
    pub fn new() -> ProcessInfo {
        ProcessInfo {
            error: None,
            info: None,
            runtime_api: None,
        }
    }
}


