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
pub struct SiteService {
    #[serde(rename = "http_connection_mode", skip_serializing_if = "Option::is_none")]
    pub http_connection_mode: Option<HttpConnectionMode>,
    #[serde(rename = "listeners", skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<crate::models::Bind>>,
    #[serde(rename = "maxconn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub maxconn: Option<Option<i32>>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}

impl SiteService {
    pub fn new() -> SiteService {
        SiteService {
            http_connection_mode: None,
            listeners: None,
            maxconn: None,
            mode: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum HttpConnectionMode {
    #[serde(rename = "http-tunnel")]
    HttpTunnel,
    #[serde(rename = "httpclose")]
    Httpclose,
    #[serde(rename = "forced-close")]
    ForcedClose,
    #[serde(rename = "http-server-close")]
    HttpServerClose,
    #[serde(rename = "http-keep-alive")]
    HttpKeepAlive,
}

impl Default for HttpConnectionMode {
    fn default() -> HttpConnectionMode {
        Self::HttpTunnel
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "tcp")]
    Tcp,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Http
    }
}

