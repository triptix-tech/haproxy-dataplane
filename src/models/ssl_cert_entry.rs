/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// SslCertEntry : One SSL/TLS certificate



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SslCertEntry {
    #[serde(rename = "algorithm", skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    #[serde(rename = "chain_issuer", skip_serializing_if = "Option::is_none")]
    pub chain_issuer: Option<String>,
    #[serde(rename = "chain_subject", skip_serializing_if = "Option::is_none")]
    pub chain_subject: Option<String>,
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "not_after", skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    #[serde(rename = "not_before", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(rename = "serial", skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "sha1_finger_print", skip_serializing_if = "Option::is_none")]
    pub sha1_finger_print: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "storage_name", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
    #[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "subject_alternative_names", skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
}

impl SslCertEntry {
    /// One SSL/TLS certificate
    pub fn new() -> SslCertEntry {
        SslCertEntry {
            algorithm: None,
            chain_issuer: None,
            chain_subject: None,
            issuer: None,
            not_after: None,
            not_before: None,
            serial: None,
            sha1_finger_print: None,
            status: None,
            storage_name: None,
            subject: None,
            subject_alternative_names: None,
        }
    }
}

