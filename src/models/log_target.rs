/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// LogTarget : Per-instance logging of events and traffic.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogTarget {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "facility", skip_serializing_if = "Option::is_none")]
    pub facility: Option<Facility>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<Format>,
    #[serde(rename = "global", skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    #[serde(rename = "index", deserialize_with = "Option::deserialize")]
    pub index: Option<i32>,
    #[serde(rename = "length", skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(rename = "minlevel", skip_serializing_if = "Option::is_none")]
    pub minlevel: Option<Minlevel>,
    #[serde(rename = "nolog", skip_serializing_if = "Option::is_none")]
    pub nolog: Option<bool>,
    #[serde(rename = "sample_range", skip_serializing_if = "Option::is_none")]
    pub sample_range: Option<String>,
    #[serde(rename = "sample_size", skip_serializing_if = "Option::is_none")]
    pub sample_size: Option<i32>,
}

impl LogTarget {
    /// Per-instance logging of events and traffic.
    pub fn new(index: Option<i32>) -> LogTarget {
        LogTarget {
            address: None,
            facility: None,
            format: None,
            global: None,
            index,
            length: None,
            level: None,
            minlevel: None,
            nolog: None,
            sample_range: None,
            sample_size: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Facility {
    #[serde(rename = "kern")]
    Kern,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "mail")]
    Mail,
    #[serde(rename = "daemon")]
    Daemon,
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "syslog")]
    Syslog,
    #[serde(rename = "lpr")]
    Lpr,
    #[serde(rename = "news")]
    News,
    #[serde(rename = "uucp")]
    Uucp,
    #[serde(rename = "cron")]
    Cron,
    #[serde(rename = "auth2")]
    Auth2,
    #[serde(rename = "ftp")]
    Ftp,
    #[serde(rename = "ntp")]
    Ntp,
    #[serde(rename = "audit")]
    Audit,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "cron2")]
    Cron2,
    #[serde(rename = "local0")]
    Local0,
    #[serde(rename = "local1")]
    Local1,
    #[serde(rename = "local2")]
    Local2,
    #[serde(rename = "local3")]
    Local3,
    #[serde(rename = "local4")]
    Local4,
    #[serde(rename = "local5")]
    Local5,
    #[serde(rename = "local6")]
    Local6,
    #[serde(rename = "local7")]
    Local7,
}

impl Default for Facility {
    fn default() -> Facility {
        Self::Kern
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "rfc3164")]
    Rfc3164,
    #[serde(rename = "rfc5424")]
    Rfc5424,
    #[serde(rename = "priority")]
    Priority,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "timed")]
    Timed,
    #[serde(rename = "iso")]
    Iso,
    #[serde(rename = "raw")]
    Raw,
}

impl Default for Format {
    fn default() -> Format {
        Self::Local
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
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Minlevel {
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

impl Default for Minlevel {
    fn default() -> Minlevel {
        Self::Emerg
    }
}
