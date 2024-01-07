/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */

/// Site : Site configuration. Sites are considered as one service and all farms connected to that service. Farms are connected to service using use-backend and default_backend directives. Sites let you configure simple HAProxy configurations, for more advanced options use /haproxy/configuration endpoints. 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Site {
    #[serde(rename = "farms", skip_serializing_if = "Option::is_none")]
    pub farms: Option<Vec<crate::models::SiteFarmsInner>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "service", skip_serializing_if = "Option::is_none")]
    pub service: Option<Box<crate::models::SiteService>>,
}

impl Site {
    /// Site configuration. Sites are considered as one service and all farms connected to that service. Farms are connected to service using use-backend and default_backend directives. Sites let you configure simple HAProxy configurations, for more advanced options use /haproxy/configuration endpoints. 
    pub fn new(name: String) -> Site {
        Site {
            farms: None,
            name,
            service: None,
        }
    }
}


