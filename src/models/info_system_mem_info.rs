/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InfoSystemMemInfo {
    #[serde(rename = "dataplaneapi_memory", skip_serializing_if = "Option::is_none")]
    pub dataplaneapi_memory: Option<i32>,
    #[serde(rename = "free_memory", skip_serializing_if = "Option::is_none")]
    pub free_memory: Option<i32>,
    #[serde(rename = "total_memory", skip_serializing_if = "Option::is_none")]
    pub total_memory: Option<i32>,
}

impl InfoSystemMemInfo {
    pub fn new() -> InfoSystemMemInfo {
        InfoSystemMemInfo {
            dataplaneapi_memory: None,
            free_memory: None,
            total_memory: None,
        }
    }
}

