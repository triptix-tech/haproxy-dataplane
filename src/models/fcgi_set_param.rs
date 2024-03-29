/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// FcgiSetParam : Sets a FastCGI parameter to pass to this application. Its value, defined by <format> can take a formatted string, the same as the log directive. Optionally, you can follow it with an ACL-based condition, in which case the FastCGI application evaluates it only if the condition is true.



#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FcgiSetParam {
    #[serde(rename = "cond", skip_serializing_if = "Option::is_none")]
    pub cond: Option<Cond>,
    #[serde(rename = "cond_test", skip_serializing_if = "Option::is_none")]
    pub cond_test: Option<String>,
    #[serde(rename = "format", skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FcgiSetParam {
    /// Sets a FastCGI parameter to pass to this application. Its value, defined by <format> can take a formatted string, the same as the log directive. Optionally, you can follow it with an ACL-based condition, in which case the FastCGI application evaluates it only if the condition is true.
    pub fn new() -> FcgiSetParam {
        FcgiSetParam {
            cond: None,
            cond_test: None,
            format: None,
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

