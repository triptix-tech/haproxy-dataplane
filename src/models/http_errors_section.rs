/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// HttpErrorsSection : A globally declared group of HTTP errors



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HttpErrorsSection {
    #[serde(rename = "error_files")]
    pub error_files: Vec<crate::models::Errorfile>,
    #[serde(rename = "name")]
    pub name: String,
}

impl HttpErrorsSection {
    /// A globally declared group of HTTP errors
    pub fn new(error_files: Vec<crate::models::Errorfile>, name: String) -> HttpErrorsSection {
        HttpErrorsSection {
            error_files,
            name,
        }
    }
}


