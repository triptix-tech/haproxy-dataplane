/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// GeneralFile : General use file



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralFile {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// File size in bytes.
    #[serde(rename = "size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<i32>>,
    #[serde(rename = "storage_name", skip_serializing_if = "Option::is_none")]
    pub storage_name: Option<String>,
}

impl GeneralFile {
    /// General use file
    pub fn new() -> GeneralFile {
        GeneralFile {
            description: None,
            file: None,
            id: None,
            size: None,
            storage_name: None,
        }
    }
}


