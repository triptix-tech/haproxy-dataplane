/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// StickTable : Stick Table Information



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StickTable {
    #[serde(rename = "fields", skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::models::StickTableFieldsInner>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Process number if master-worker mode
    #[serde(rename = "process", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub process: Option<Option<i32>>,
    #[serde(rename = "size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<i32>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "used", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub used: Option<Option<i32>>,
}

impl StickTable {
    /// Stick Table Information
    pub fn new() -> StickTable {
        StickTable {
            fields: None,
            name: None,
            process: None,
            size: None,
            r#type: None,
            used: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ip")]
    Ip,
    #[serde(rename = "ipv6")]
    Ipv6,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "binary")]
    Binary,
}

impl Default for Type {
    fn default() -> Type {
        Self::Ip
    }
}

