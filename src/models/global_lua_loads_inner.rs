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
pub struct GlobalLuaLoadsInner {
    #[serde(rename = "file")]
    pub file: String,
}

impl GlobalLuaLoadsInner {
    pub fn new(file: String) -> GlobalLuaLoadsInner {
        GlobalLuaLoadsInner {
            file,
        }
    }
}

