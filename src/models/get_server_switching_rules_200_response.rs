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
pub struct GetServerSwitchingRules200Response {
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub _version: Option<i32>,
    /// HAProxy backend server switching rules array (corresponds to use-server directives)
    #[serde(rename = "data")]
    pub data: Vec<crate::models::ServerSwitchingRule>,
}

impl GetServerSwitchingRules200Response {
    pub fn new(data: Vec<crate::models::ServerSwitchingRule>) -> GetServerSwitchingRules200Response {
        GetServerSwitchingRules200Response {
            _version: None,
            data,
        }
    }
}


