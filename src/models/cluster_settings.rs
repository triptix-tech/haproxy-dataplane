/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// ClusterSettings : Settings related to a cluster.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterSettings {
    #[serde(rename = "bootstrap_key", skip_serializing_if = "Option::is_none")]
    pub bootstrap_key: Option<String>,
    #[serde(rename = "cluster", skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Box<crate::models::ClusterControllerInformation>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl ClusterSettings {
    /// Settings related to a cluster.
    pub fn new() -> ClusterSettings {
        ClusterSettings {
            bootstrap_key: None,
            cluster: None,
            mode: None,
            status: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "cluster")]
    Cluster,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Single
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "unreachable")]
    Unreachable,
    #[serde(rename = "waiting_approval")]
    WaitingApproval,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

