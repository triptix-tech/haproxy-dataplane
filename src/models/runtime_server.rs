/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// RuntimeServer : Runtime transient server properties



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeServer {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "admin_state", skip_serializing_if = "Option::is_none")]
    pub admin_state: Option<AdminState>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "operational_state", skip_serializing_if = "Option::is_none")]
    pub operational_state: Option<OperationalState>,
    #[serde(rename = "port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub port: Option<Option<i32>>,
}

impl RuntimeServer {
    /// Runtime transient server properties
    pub fn new() -> RuntimeServer {
        RuntimeServer {
            address: None,
            admin_state: None,
            id: None,
            name: None,
            operational_state: None,
            port: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AdminState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "maint")]
    Maint,
    #[serde(rename = "drain")]
    Drain,
}

impl Default for AdminState {
    fn default() -> AdminState {
        Self::Ready
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OperationalState {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "stopping")]
    Stopping,
}

impl Default for OperationalState {
    fn default() -> OperationalState {
        Self::Up
    }
}

