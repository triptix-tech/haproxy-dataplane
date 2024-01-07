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
pub struct GlobalDeviceAtlasOptions {
    #[serde(rename = "json_file", skip_serializing_if = "Option::is_none")]
    pub json_file: Option<String>,
    #[serde(rename = "log_level", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    #[serde(rename = "properties_cookie", skip_serializing_if = "Option::is_none")]
    pub properties_cookie: Option<String>,
    #[serde(rename = "separator", skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
}

impl GlobalDeviceAtlasOptions {
    pub fn new() -> GlobalDeviceAtlasOptions {
        GlobalDeviceAtlasOptions {
            json_file: None,
            log_level: None,
            properties_cookie: None,
            separator: None,
        }
    }
}

