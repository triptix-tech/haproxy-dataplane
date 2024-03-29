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
pub struct Sample {
    #[serde(rename = "ranges")]
    pub ranges: String,
    #[serde(rename = "size")]
    pub size: i32,
}

impl Sample {
    pub fn new(ranges: String, size: i32) -> Sample {
        Sample {
            ranges,
            size,
        }
    }
}


