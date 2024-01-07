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
pub struct ConfigStickTable {
    #[serde(rename = "expire", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expire: Option<Option<i32>>,
    #[serde(rename = "keylen", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub keylen: Option<Option<i32>>,
    #[serde(rename = "nopurge", skip_serializing_if = "Option::is_none")]
    pub nopurge: Option<bool>,
    #[serde(rename = "peers", skip_serializing_if = "Option::is_none")]
    pub peers: Option<String>,
    #[serde(rename = "size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<i32>>,
    #[serde(rename = "srvkey", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub srvkey: Option<Option<Srvkey>>,
    #[serde(rename = "store", skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "write_to", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub write_to: Option<Option<String>>,
}

impl ConfigStickTable {
    pub fn new() -> ConfigStickTable {
        ConfigStickTable {
            expire: None,
            keylen: None,
            nopurge: None,
            peers: None,
            size: None,
            srvkey: None,
            store: None,
            r#type: None,
            write_to: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Srvkey {
    #[serde(rename = "addr")]
    Addr,
    #[serde(rename = "name")]
    Name,
}

impl Default for Srvkey {
    fn default() -> Srvkey {
        Self::Addr
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

