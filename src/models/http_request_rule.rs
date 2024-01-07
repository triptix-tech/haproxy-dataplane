/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// HttpRequestRule : HAProxy HTTP request rule configuration (corresponds to http-request directives)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpRequestRule {
    #[serde(rename = "acl_file", skip_serializing_if = "Option::is_none")]
    pub acl_file: Option<String>,
    #[serde(rename = "acl_keyfmt", skip_serializing_if = "Option::is_none")]
    pub acl_keyfmt: Option<String>,
    #[serde(rename = "auth_realm", skip_serializing_if = "Option::is_none")]
    pub auth_realm: Option<String>,
    #[serde(rename = "bandwidth_limit_limit", skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit_limit: Option<String>,
    #[serde(rename = "bandwidth_limit_name", skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit_name: Option<String>,
    #[serde(rename = "bandwidth_limit_period", skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit_period: Option<String>,
    #[serde(rename = "cache_name", skip_serializing_if = "Option::is_none")]
    pub cache_name: Option<String>,
    #[serde(rename = "capture_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub capture_id: Option<Option<i32>>,
    #[serde(rename = "capture_len", skip_serializing_if = "Option::is_none")]
    pub capture_len: Option<i32>,
    #[serde(rename = "capture_sample", skip_serializing_if = "Option::is_none")]
    pub capture_sample: Option<String>,
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    pub cond: Option<Cond>,
    #[serde(rename = "cond_test", skip_serializing_if = "Option::is_none")]
    pub cond_test: Option<String>,
    #[serde(rename = "deny_status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub deny_status: Option<Option<i32>>,
    #[serde(rename = "expr", skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    #[serde(rename = "hdr_format", skip_serializing_if = "Option::is_none")]
    pub hdr_format: Option<String>,
    #[serde(rename = "hdr_match", skip_serializing_if = "Option::is_none")]
    pub hdr_match: Option<String>,
    #[serde(rename = "hdr_method", skip_serializing_if = "Option::is_none")]
    pub hdr_method: Option<String>,
    #[serde(rename = "hdr_name", skip_serializing_if = "Option::is_none")]
    pub hdr_name: Option<String>,
    #[serde(rename = "hint_format", skip_serializing_if = "Option::is_none")]
    pub hint_format: Option<String>,
    #[serde(rename = "hint_name", skip_serializing_if = "Option::is_none")]
    pub hint_name: Option<String>,
    #[serde(rename = "index", deserialize_with = "Option::deserialize")]
    pub index: Option<i32>,
    #[serde(rename = "log_level", skip_serializing_if = "Option::is_none")]
    pub log_level: Option<LogLevel>,
    #[serde(rename = "lua_action", skip_serializing_if = "Option::is_none")]
    pub lua_action: Option<String>,
    #[serde(rename = "lua_params", skip_serializing_if = "Option::is_none")]
    pub lua_params: Option<String>,
    #[serde(rename = "map_file", skip_serializing_if = "Option::is_none")]
    pub map_file: Option<String>,
    #[serde(rename = "map_keyfmt", skip_serializing_if = "Option::is_none")]
    pub map_keyfmt: Option<String>,
    #[serde(rename = "map_valuefmt", skip_serializing_if = "Option::is_none")]
    pub map_valuefmt: Option<String>,
    #[serde(rename = "mark_value", skip_serializing_if = "Option::is_none")]
    pub mark_value: Option<String>,
    #[serde(rename = "method_fmt", skip_serializing_if = "Option::is_none")]
    pub method_fmt: Option<String>,
    #[serde(rename = "nice_value", skip_serializing_if = "Option::is_none")]
    pub nice_value: Option<i32>,
    #[serde(rename = "normalizer", skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<Normalizer>,
    #[serde(rename = "normalizer_full", skip_serializing_if = "Option::is_none")]
    pub normalizer_full: Option<bool>,
    #[serde(rename = "normalizer_strict", skip_serializing_if = "Option::is_none")]
    pub normalizer_strict: Option<bool>,
    #[serde(rename = "path_fmt", skip_serializing_if = "Option::is_none")]
    pub path_fmt: Option<String>,
    #[serde(rename = "path_match", skip_serializing_if = "Option::is_none")]
    pub path_match: Option<String>,
    #[serde(rename = "protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    #[serde(rename = "query-fmt", skip_serializing_if = "Option::is_none")]
    pub query_fmt: Option<String>,
    #[serde(rename = "redir_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub redir_code: Option<Option<RedirCode>>,
    #[serde(rename = "redir_option", skip_serializing_if = "Option::is_none")]
    pub redir_option: Option<String>,
    #[serde(rename = "redir_type", skip_serializing_if = "Option::is_none")]
    pub redir_type: Option<RedirType>,
    #[serde(rename = "redir_value", skip_serializing_if = "Option::is_none")]
    pub redir_value: Option<String>,
    #[serde(rename = "resolvers", skip_serializing_if = "Option::is_none")]
    pub resolvers: Option<String>,
    #[serde(rename = "return_content", skip_serializing_if = "Option::is_none")]
    pub return_content: Option<String>,
    #[serde(rename = "return_content_format", skip_serializing_if = "Option::is_none")]
    pub return_content_format: Option<ReturnContentFormat>,
    #[serde(rename = "return_content_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub return_content_type: Option<Option<String>>,
    #[serde(rename = "return_hdrs", skip_serializing_if = "Option::is_none")]
    pub return_hdrs: Option<Vec<crate::models::ReturnHeader>>,
    #[serde(rename = "return_status_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub return_status_code: Option<Option<i32>>,
    #[serde(rename = "sc_expr", skip_serializing_if = "Option::is_none")]
    pub sc_expr: Option<String>,
    #[serde(rename = "sc_id", skip_serializing_if = "Option::is_none")]
    pub sc_id: Option<i32>,
    #[serde(rename = "sc_idx", skip_serializing_if = "Option::is_none")]
    pub sc_idx: Option<i32>,
    #[serde(rename = "sc_int", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sc_int: Option<Option<i32>>,
    #[serde(rename = "service_name", skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "spoe_engine", skip_serializing_if = "Option::is_none")]
    pub spoe_engine: Option<String>,
    #[serde(rename = "spoe_group", skip_serializing_if = "Option::is_none")]
    pub spoe_group: Option<String>,
    #[serde(rename = "strict_mode", skip_serializing_if = "Option::is_none")]
    pub strict_mode: Option<StrictMode>,
    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(rename = "timeout_type", skip_serializing_if = "Option::is_none")]
    pub timeout_type: Option<TimeoutType>,
    #[serde(rename = "tos_value", skip_serializing_if = "Option::is_none")]
    pub tos_value: Option<String>,
    #[serde(rename = "track-sc0-key", skip_serializing_if = "Option::is_none")]
    pub track_sc0_key: Option<String>,
    #[serde(rename = "track-sc0-table", skip_serializing_if = "Option::is_none")]
    pub track_sc0_table: Option<String>,
    #[serde(rename = "track-sc1-key", skip_serializing_if = "Option::is_none")]
    pub track_sc1_key: Option<String>,
    #[serde(rename = "track-sc1-table", skip_serializing_if = "Option::is_none")]
    pub track_sc1_table: Option<String>,
    #[serde(rename = "track-sc2-key", skip_serializing_if = "Option::is_none")]
    pub track_sc2_key: Option<String>,
    #[serde(rename = "track-sc2-table", skip_serializing_if = "Option::is_none")]
    pub track_sc2_table: Option<String>,
    #[serde(rename = "track_sc_key", skip_serializing_if = "Option::is_none")]
    pub track_sc_key: Option<String>,
    #[serde(rename = "track_sc_stick_counter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub track_sc_stick_counter: Option<Option<i32>>,
    #[serde(rename = "track_sc_table", skip_serializing_if = "Option::is_none")]
    pub track_sc_table: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "uri-fmt", skip_serializing_if = "Option::is_none")]
    pub uri_fmt: Option<String>,
    #[serde(rename = "uri-match", skip_serializing_if = "Option::is_none")]
    pub uri_match: Option<String>,
    #[serde(rename = "var_expr", skip_serializing_if = "Option::is_none")]
    pub var_expr: Option<String>,
    #[serde(rename = "var_format", skip_serializing_if = "Option::is_none")]
    pub var_format: Option<String>,
    #[serde(rename = "var_name", skip_serializing_if = "Option::is_none")]
    pub var_name: Option<String>,
    #[serde(rename = "var_scope", skip_serializing_if = "Option::is_none")]
    pub var_scope: Option<String>,
    #[serde(rename = "wait_at_least", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wait_at_least: Option<Option<i32>>,
    #[serde(rename = "wait_time", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<Option<i32>>,
}

impl HttpRequestRule {
    /// HAProxy HTTP request rule configuration (corresponds to http-request directives)
    pub fn new(index: Option<i32>, r#type: Type) -> HttpRequestRule {
        HttpRequestRule {
            acl_file: None,
            acl_keyfmt: None,
            auth_realm: None,
            bandwidth_limit_limit: None,
            bandwidth_limit_name: None,
            bandwidth_limit_period: None,
            cache_name: None,
            capture_id: None,
            capture_len: None,
            capture_sample: None,
            cond: None,
            cond_test: None,
            deny_status: None,
            expr: None,
            hdr_format: None,
            hdr_match: None,
            hdr_method: None,
            hdr_name: None,
            hint_format: None,
            hint_name: None,
            index,
            log_level: None,
            lua_action: None,
            lua_params: None,
            map_file: None,
            map_keyfmt: None,
            map_valuefmt: None,
            mark_value: None,
            method_fmt: None,
            nice_value: None,
            normalizer: None,
            normalizer_full: None,
            normalizer_strict: None,
            path_fmt: None,
            path_match: None,
            protocol: None,
            query_fmt: None,
            redir_code: None,
            redir_option: None,
            redir_type: None,
            redir_value: None,
            resolvers: None,
            return_content: None,
            return_content_format: None,
            return_content_type: None,
            return_hdrs: None,
            return_status_code: None,
            sc_expr: None,
            sc_id: None,
            sc_idx: None,
            sc_int: None,
            service_name: None,
            spoe_engine: None,
            spoe_group: None,
            strict_mode: None,
            timeout: None,
            timeout_type: None,
            tos_value: None,
            track_sc0_key: None,
            track_sc0_table: None,
            track_sc1_key: None,
            track_sc1_table: None,
            track_sc2_key: None,
            track_sc2_table: None,
            track_sc_key: None,
            track_sc_stick_counter: None,
            track_sc_table: None,
            r#type,
            uri_fmt: None,
            uri_match: None,
            var_expr: None,
            var_format: None,
            var_name: None,
            var_scope: None,
            wait_at_least: None,
            wait_time: None,
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
pub enum Normalizer {
    #[serde(rename = "fragment-encode")]
    FragmentEncode,
    #[serde(rename = "fragment-strip")]
    FragmentStrip,
    #[serde(rename = "path-merge-slashes")]
    PathMergeSlashes,
    #[serde(rename = "path-strip-dot")]
    PathStripDot,
    #[serde(rename = "path-strip-dotdot")]
    PathStripDotdot,
    #[serde(rename = "percent-decode-unreserved")]
    PercentDecodeUnreserved,
    #[serde(rename = "percent-to-upercase")]
    PercentToUpercase,
    #[serde(rename = "query-sort-by-name")]
    QuerySortByName,
}

impl Default for Normalizer {
    fn default() -> Normalizer {
        Self::FragmentEncode
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "ipv4")]
    Ipv4,
    #[serde(rename = "ipv6")]
    Ipv6,
}

impl Default for Protocol {
    fn default() -> Protocol {
        Self::Ipv4
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedirCode {
    #[serde(rename = "301")]
    Variant301,
    #[serde(rename = "302")]
    Variant302,
    #[serde(rename = "303")]
    Variant303,
    #[serde(rename = "307")]
    Variant307,
    #[serde(rename = "308")]
    Variant308,
}

impl Default for RedirCode {
    fn default() -> RedirCode {
        Self::Variant301
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RedirType {
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "prefix")]
    Prefix,
    #[serde(rename = "scheme")]
    Scheme,
}

impl Default for RedirType {
    fn default() -> RedirType {
        Self::Location
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ReturnContentFormat {
    #[serde(rename = "default-errorfiles")]
    DefaultErrorfiles,
    #[serde(rename = "errorfile")]
    Errorfile,
    #[serde(rename = "errorfiles")]
    Errorfiles,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "lf-file")]
    LfFile,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "lf-string")]
    LfString,
}

impl Default for ReturnContentFormat {
    fn default() -> ReturnContentFormat {
        Self::DefaultErrorfiles
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
pub enum TimeoutType {
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "tunnel")]
    Tunnel,
    #[serde(rename = "client")]
    Client,
}

impl Default for TimeoutType {
    fn default() -> TimeoutType {
        Self::Server
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "add-acl")]
    AddAcl,
    #[serde(rename = "add-header")]
    AddHeader,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "auth")]
    Auth,
    #[serde(rename = "cache-use")]
    CacheUse,
    #[serde(rename = "capture")]
    Capture,
    #[serde(rename = "del-acl")]
    DelAcl,
    #[serde(rename = "del-header")]
    DelHeader,
    #[serde(rename = "del-map")]
    DelMap,
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "disable-l7-retry")]
    DisableL7Retry,
    #[serde(rename = "do-resolve")]
    DoResolve,
    #[serde(rename = "early-hint")]
    EarlyHint,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "normalize-uri")]
    NormalizeUri,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "reject")]
    Reject,
    #[serde(rename = "replace-header")]
    ReplaceHeader,
    #[serde(rename = "replace-path")]
    ReplacePath,
    #[serde(rename = "replace-pathq")]
    ReplacePathq,
    #[serde(rename = "replace-uri")]
    ReplaceUri,
    #[serde(rename = "replace-value")]
    ReplaceValue,
    #[serde(rename = "return")]
    Return,
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
    #[serde(rename = "send-spoe-group")]
    SendSpoeGroup,
    #[serde(rename = "set-dst")]
    SetDst,
    #[serde(rename = "set-dst-port")]
    SetDstPort,
    #[serde(rename = "set-header")]
    SetHeader,
    #[serde(rename = "set-log-level")]
    SetLogLevel,
    #[serde(rename = "set-map")]
    SetMap,
    #[serde(rename = "set-mark")]
    SetMark,
    #[serde(rename = "set-method")]
    SetMethod,
    #[serde(rename = "set-nice")]
    SetNice,
    #[serde(rename = "set-path")]
    SetPath,
    #[serde(rename = "set-pathq")]
    SetPathq,
    #[serde(rename = "set-priority-class")]
    SetPriorityClass,
    #[serde(rename = "set-priority-offset")]
    SetPriorityOffset,
    #[serde(rename = "set-query")]
    SetQuery,
    #[serde(rename = "set-src")]
    SetSrc,
    #[serde(rename = "set-src-port")]
    SetSrcPort,
    #[serde(rename = "set-timeout")]
    SetTimeout,
    #[serde(rename = "set-tos")]
    SetTos,
    #[serde(rename = "set-uri")]
    SetUri,
    #[serde(rename = "set-var")]
    SetVar,
    #[serde(rename = "silent-drop")]
    SilentDrop,
    #[serde(rename = "strict-mode")]
    StrictMode,
    #[serde(rename = "tarpit")]
    Tarpit,
    #[serde(rename = "track-sc0")]
    TrackSc0,
    #[serde(rename = "track-sc1")]
    TrackSc1,
    #[serde(rename = "track-sc2")]
    TrackSc2,
    #[serde(rename = "track-sc")]
    TrackSc,
    #[serde(rename = "unset-var")]
    UnsetVar,
    #[serde(rename = "use-service")]
    UseService,
    #[serde(rename = "wait-for-body")]
    WaitForBody,
    #[serde(rename = "wait-for-handshake")]
    WaitForHandshake,
    #[serde(rename = "set-bandwidth-limit")]
    SetBandwidthLimit,
}

impl Default for Type {
    fn default() -> Type {
        Self::AddAcl
    }
}

