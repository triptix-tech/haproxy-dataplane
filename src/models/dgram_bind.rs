/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// DgramBind : HAProxy log forward dgram bind configuration



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DgramBind {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "interface", skip_serializing_if = "Option::is_none")]
    pub interface: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "namespace", skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<i32>>,
    #[serde(rename = "port-range-end", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port_range_end: Option<Option<i32>>,
    #[serde(rename = "transparent", skip_serializing_if = "Option::is_none")]
    pub transparent: Option<bool>,
}

impl DgramBind {
    /// HAProxy log forward dgram bind configuration
    pub fn new() -> DgramBind {
        DgramBind {
            address: None,
            interface: None,
            name: None,
            namespace: None,
            port: None,
            port_range_end: None,
            transparent: None,
        }
    }
}

