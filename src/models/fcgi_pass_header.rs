/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// FcgiPassHeader : Specifies the name of a request header to pass to the FastCGI application. Optionally, you can follow it with an ACL-based condition, in which case the FastCGI application evaluates it only if the condition is true. Most request headers are already available to the FastCGI application with the prefix \"HTTP\". Thus, you only need this directive to pass headers that are purposefully omitted. Currently, the headers \"Authorization\", \"Proxy-Authorization\", and hop-by-hop headers are omitted. Note that the headers \"Content-type\" and \"Content-length\" never pass to the FastCGI application because they are already converted into parameters.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FcgiPassHeader {
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    pub cond: Option<Cond>,
    #[serde(rename = "cond_test", skip_serializing_if = "Option::is_none")]
    pub cond_test: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FcgiPassHeader {
    /// Specifies the name of a request header to pass to the FastCGI application. Optionally, you can follow it with an ACL-based condition, in which case the FastCGI application evaluates it only if the condition is true. Most request headers are already available to the FastCGI application with the prefix \"HTTP\". Thus, you only need this directive to pass headers that are purposefully omitted. Currently, the headers \"Authorization\", \"Proxy-Authorization\", and hop-by-hop headers are omitted. Note that the headers \"Content-type\" and \"Content-length\" never pass to the FastCGI application because they are already converted into parameters.
    pub fn new() -> FcgiPassHeader {
        FcgiPassHeader {
            cond: None,
            cond_test: None,
            name: None,
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

