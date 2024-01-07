/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// HttpAfterResponseRule : HAProxy HTTP after response rule configuration (corresponds to http-after-response directives)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpAfterResponseRule {
    #[serde(rename = "acl_file", skip_serializing_if = "Option::is_none")]
    pub acl_file: Option<String>,
    #[serde(rename = "acl_keyfmt", skip_serializing_if = "Option::is_none")]
    pub acl_keyfmt: Option<String>,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    pub cond: Option<Cond>,
    #[serde(rename = "cond_test", skip_serializing_if = "Option::is_none")]
    pub cond_test: Option<String>,
    #[serde(rename = "hdr_format", skip_serializing_if = "Option::is_none")]
    pub hdr_format: Option<String>,
    #[serde(rename = "hdr_match", skip_serializing_if = "Option::is_none")]
    pub hdr_match: Option<String>,
    #[serde(rename = "hdr_method", skip_serializing_if = "Option::is_none")]
    pub hdr_method: Option<String>,
    #[serde(rename = "hdr_name", skip_serializing_if = "Option::is_none")]
    pub hdr_name: Option<String>,
    #[serde(rename = "index", deserialize_with = "Option::deserialize")]
    pub index: Option<i32>,
    #[serde(rename = "log_level", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<LogLevel>,
    #[serde(rename = "map_file", skip_serializing_if = "Option::is_none")]
    pub map_file: Option<String>,
    #[serde(rename = "map_keyfmt", skip_serializing_if = "Option::is_none")]
    pub map_keyfmt: Option<String>,
    #[serde(rename = "map_valuefmt", skip_serializing_if = "Option::is_none")]
    pub map_valuefmt: Option<String>,
    #[serde(rename = "sc_expr", skip_serializing_if = "Option::is_none")]
    pub sc_expr: Option<String>,
    #[serde(rename = "sc_id", skip_serializing_if = "Option::is_none")]
    pub sc_id: Option<i32>,
    #[serde(rename = "sc_idx", skip_serializing_if = "Option::is_none")]
    pub sc_idx: Option<i32>,
    #[serde(rename = "sc_int", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sc_int: Option<Option<i32>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "status_reason", skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "strict_mode", skip_serializing_if = "Option::is_none")]
    pub strict_mode: Option<StrictMode>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "var_expr", skip_serializing_if = "Option::is_none")]
    pub var_expr: Option<String>,
    #[serde(rename = "var_name", skip_serializing_if = "Option::is_none")]
    pub var_name: Option<String>,
    #[serde(rename = "var_scope", skip_serializing_if = "Option::is_none")]
    pub var_scope: Option<String>,
}

impl HttpAfterResponseRule {
    /// HAProxy HTTP after response rule configuration (corresponds to http-after-response directives)
    pub fn new(index: Option<i32>, r#type: Type) -> HttpAfterResponseRule {
        HttpAfterResponseRule {
            acl_file: None,
            acl_keyfmt: None,
            cond: None,
            cond_test: None,
            hdr_format: None,
            hdr_match: None,
            hdr_method: None,
            hdr_name: None,
            index,
            log_level: None,
            map_file: None,
            map_keyfmt: None,
            map_valuefmt: None,
            sc_expr: None,
            sc_id: None,
            sc_idx: None,
            sc_int: None,
            status: None,
            status_reason: None,
            strict_mode: None,
            r#type,
            var_expr: None,
            var_name: None,
            var_scope: None,
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
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LogLevel {
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
    #[serde(rename = "silent")]
    Silent,
}

impl Default for LogLevel {
    fn default() -> LogLevel {
        Self::Emerg
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StrictMode {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
}

impl Default for StrictMode {
    fn default() -> StrictMode {
        Self::On
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "add-header")]
    AddHeader,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "del-acl")]
    DelAcl,
    #[serde(rename = "del-header")]
    DelHeader,
    #[serde(rename = "del-map")]
    DelMap,
    #[serde(rename = "replace-header")]
    ReplaceHeader,
    #[serde(rename = "replace-value")]
    ReplaceValue,
    #[serde(rename = "sc-add-gpc")]
    ScAddGpc,
    #[serde(rename = "sc-inc-gpc")]
    ScIncGpc,
    #[serde(rename = "sc-inc-gpc0")]
    ScIncGpc0,
    #[serde(rename = "sc-inc-gpc1")]
    ScIncGpc1,
    #[serde(rename = "sc-set-gpt0")]
    ScSetGpt0,
    #[serde(rename = "set-header")]
    SetHeader,
    #[serde(rename = "set-log-level")]
    SetLogLevel,
    #[serde(rename = "set-map")]
    SetMap,
    #[serde(rename = "set-status")]
    SetStatus,
    #[serde(rename = "set-var")]
    SetVar,
    #[serde(rename = "strict-mode")]
    StrictMode,
    #[serde(rename = "unset-var")]
    UnsetVar,
}

impl Default for Type {
    fn default() -> Type {
        Self::AddHeader
    }
}
