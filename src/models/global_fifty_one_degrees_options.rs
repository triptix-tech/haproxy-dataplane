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
pub struct GlobalFiftyOneDegreesOptions {
    #[serde(rename = "cache_size", skip_serializing_if = "Option::is_none")]
    pub cache_size: Option<i32>,
    #[serde(rename = "data_file", skip_serializing_if = "Option::is_none")]
    pub data_file: Option<String>,
    #[serde(rename = "property_name_list", skip_serializing_if = "Option::is_none")]
    pub property_name_list: Option<String>,
    #[serde(rename = "property_separator", skip_serializing_if = "Option::is_none")]
    pub property_separator: Option<String>,
}

impl GlobalFiftyOneDegreesOptions {
    pub fn new() -> GlobalFiftyOneDegreesOptions {
        GlobalFiftyOneDegreesOptions {
            cache_size: None,
            data_file: None,
            property_name_list: None,
            property_separator: None,
        }
    }
}


