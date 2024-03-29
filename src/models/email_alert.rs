/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// EmailAlert : Send emails for important log messages.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailAlert {
    #[serde(rename = "from")]
    pub from: String,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(rename = "mailers")]
    pub mailers: String,
    #[serde(rename = "myhostname", skip_serializing_if = "Option::is_none")]
    pub myhostname: Option<String>,
    #[serde(rename = "to")]
    pub to: String,
}

impl EmailAlert {
    /// Send emails for important log messages.
    pub fn new(from: String, mailers: String, to: String) -> EmailAlert {
        EmailAlert {
            from,
            level: None,
            mailers,
            myhostname: None,
            to,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "emerg")]
    Emerg,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "crit")]
    Crit,
    #[serde(rename = "err")]
    Err,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
}

impl Default for Level {
    fn default() -> Level {
        Self::Emerg
    }
}

