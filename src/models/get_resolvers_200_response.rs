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
pub struct GetResolvers200Response {
    #[serde(rename = "_version", skip_serializing_if = "Option::is_none")]
    pub _version: Option<i32>,
    /// HAProxy resolvers array
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Resolver>,
}

impl GetResolvers200Response {
    pub fn new(data: Vec<crate::models::Resolver>) -> GetResolvers200Response {
        GetResolvers200Response {
            _version: None,
            data,
        }
    }
}


