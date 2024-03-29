/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// Filter : HAProxy filters



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Filter {
    /// Name of the fcgi-app section this filter will use.
    #[serde(rename = "app_name", skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// Filter name that will be used by 'set-bandwidth-limit' actions to reference a specific bandwidth limitation filter
    #[serde(rename = "bandwidth_limit_name", skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit_name: Option<String>,
    #[serde(rename = "cache_name", skip_serializing_if = "Option::is_none")]
    pub cache_name: Option<String>,
    /// The max number of bytes that can be forwarded over the period. The value must be specified for per-stream and shared bandwidth limitation filters. It follows the HAProxy size format and is expressed in bytes.
    #[serde(rename = "default_limit", skip_serializing_if = "Option::is_none")]
    pub default_limit: Option<i32>,
    /// The default time period used to evaluate the bandwidth limitation rate. It can be specified for per-stream bandwidth limitation filters only. It follows the HAProxy time format and is expressed in milliseconds.
    #[serde(rename = "default_period", skip_serializing_if = "Option::is_none")]
    pub default_period: Option<i32>,
    #[serde(rename = "index", deserialize_with = "Option::deserialize")]
    pub index: Option<i32>,
    /// A sample expression rule. It describes what elements will be analyzed, extracted, combined, and used to select which table entry to update the counters. It must be specified for shared bandwidth limitation filters only.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The max number of bytes that can be forwarded over the period. The value must be specified for per-stream and shared bandwidth limitation filters. It follows the HAProxy size format and is expressed in bytes.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// The optional minimum number of bytes forwarded at a time by a stream excluding the last packet that may be smaller. This value can be specified for per-stream and shared bandwidth limitation filters. It follows the HAProxy size format and is expressed in bytes.
    #[serde(rename = "min_size", skip_serializing_if = "Option::is_none")]
    pub min_size: Option<i32>,
    #[serde(rename = "spoe_config", skip_serializing_if = "Option::is_none")]
    pub spoe_config: Option<String>,
    #[serde(rename = "spoe_engine", skip_serializing_if = "Option::is_none")]
    pub spoe_engine: Option<String>,
    /// An optional table to be used instead of the default one, which is the stick-table declared in the current proxy. It can be specified for shared bandwidth limitation filters only.
    #[serde(rename = "table", skip_serializing_if = "Option::is_none")]
    pub table: Option<String>,
    #[serde(rename = "trace_hexdump", skip_serializing_if = "Option::is_none")]
    pub trace_hexdump: Option<bool>,
    #[serde(rename = "trace_name", skip_serializing_if = "Option::is_none")]
    pub trace_name: Option<String>,
    #[serde(rename = "trace_rnd_forwarding", skip_serializing_if = "Option::is_none")]
    pub trace_rnd_forwarding: Option<bool>,
    #[serde(rename = "trace_rnd_parsing", skip_serializing_if = "Option::is_none")]
    pub trace_rnd_parsing: Option<bool>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl Filter {
    /// HAProxy filters
    pub fn new(index: Option<i32>, r#type: Type) -> Filter {
        Filter {
            app_name: None,
            bandwidth_limit_name: None,
            cache_name: None,
            default_limit: None,
            default_period: None,
            index,
            key: None,
            limit: None,
            min_size: None,
            spoe_config: None,
            spoe_engine: None,
            table: None,
            trace_hexdump: None,
            trace_name: None,
            trace_rnd_forwarding: None,
            trace_rnd_parsing: None,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "compression")]
    Compression,
    #[serde(rename = "spoe")]
    Spoe,
    #[serde(rename = "cache")]
    Cache,
    #[serde(rename = "fcgi-app")]
    FcgiApp,
    #[serde(rename = "bwlim-in")]
    BwlimIn,
    #[serde(rename = "bwlim-out")]
    BwlimOut,
}

impl Default for Type {
    fn default() -> Type {
        Self::Trace
    }
}

