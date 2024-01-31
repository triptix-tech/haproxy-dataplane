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
pub struct GetPrograms200Response {
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub _version: Option<i32>,
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Program>,
}

impl GetPrograms200Response {
    pub fn new(data: Vec<crate::models::Program>) -> GetPrograms200Response {
        GetPrograms200Response {
            _version: None,
            data,
        }
    }
}


