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
pub struct GetHttpResponseRule200Response {
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub _version: Option<i32>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<crate::models::HttpResponseRule>>,
}

impl GetHttpResponseRule200Response {
    pub fn new() -> GetHttpResponseRule200Response {
        GetHttpResponseRule200Response {
            _version: None,
            data: None,
        }
    }
}


