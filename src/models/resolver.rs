/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// Resolver : Runtime DNS configuration



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resolver {
    #[serde(rename = "accepted_payload_size", skip_serializing_if = "Option::is_none")]
    pub accepted_payload_size: Option<i32>,
    #[serde(rename = "hold_nx", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold_nx: Option<Option<i32>>,
    #[serde(rename = "hold_obsolete", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold_obsolete: Option<Option<i32>>,
    #[serde(rename = "hold_other", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold_other: Option<Option<i32>>,
    #[serde(rename = "hold_refused", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold_refused: Option<Option<i32>>,
    #[serde(rename = "hold_timeout", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold_timeout: Option<Option<i32>>,
    #[serde(rename = "hold_valid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub hold_valid: Option<Option<i32>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "parse-resolv-conf", skip_serializing_if = "Option::is_none")]
    pub parse_resolv_conf: Option<bool>,
    #[serde(rename = "resolve_retries", skip_serializing_if = "Option::is_none")]
    pub resolve_retries: Option<i32>,
    #[serde(rename = "timeout_resolve", skip_serializing_if = "Option::is_none")]
    pub timeout_resolve: Option<i32>,
    #[serde(rename = "timeout_retry", skip_serializing_if = "Option::is_none")]
    pub timeout_retry: Option<i32>,
}

impl Resolver {
    /// Runtime DNS configuration
    pub fn new(name: String) -> Resolver {
        Resolver {
            accepted_payload_size: None,
            hold_nx: None,
            hold_obsolete: None,
            hold_other: None,
            hold_refused: None,
            hold_timeout: None,
            hold_valid: None,
            name,
            parse_resolv_conf: None,
            resolve_retries: None,
            timeout_resolve: None,
            timeout_retry: None,
        }
    }
}


