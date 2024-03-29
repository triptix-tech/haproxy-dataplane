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
pub struct Table {
    #[serde(rename = "expire", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expire: Option<Option<String>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "no_purge", skip_serializing_if = "Option::is_none")]
    pub no_purge: Option<bool>,
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(rename = "store", skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "type_len", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub type_len: Option<Option<i32>>,
    #[serde(rename = "write_to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub write_to: Option<Option<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            expire: None,
            name: None,
            no_purge: None,
            size: None,
            store: None,
            r#type: None,
            type_len: None,
            write_to: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ip")]
    Ip,
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

