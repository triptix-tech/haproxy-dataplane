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
pub struct Compression {
    #[serde(rename = "algorithms", skip_serializing_if = "Option::is_none")]
    pub algorithms: Option<Vec<Algorithms>>,
    #[serde(rename = "offload", skip_serializing_if = "Option::is_none")]
    pub offload: Option<bool>,
    #[serde(rename = "types", skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl Compression {
    pub fn new() -> Compression {
        Compression {
            algorithms: None,
            offload: None,
            types: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Algorithms {
    #[serde(rename = "identity")]
    Identity,
    #[serde(rename = "gzip")]
    Gzip,
    #[serde(rename = "deflate")]
    Deflate,
    #[serde(rename = "raw-deflate")]
    RawDeflate,
}

impl Default for Algorithms {
    fn default() -> Algorithms {
        Self::Identity
    }
}

