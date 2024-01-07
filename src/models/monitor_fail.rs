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
pub struct MonitorFail {
    #[serde(rename = "cond")]
    pub cond: Cond,
    #[serde(rename = "cond_test")]
    pub cond_test: String,
}

impl MonitorFail {
    pub fn new(cond: Cond, cond_test: String) -> MonitorFail {
        MonitorFail {
            cond,
            cond_test,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Cond {
    #[serde(rename = "if")]
    If,
    #[serde(rename = "unless")]
    Unless,
}

impl Default for Cond {
    fn default() -> Cond {
        Self::If
    }
}

