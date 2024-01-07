/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Errorloc {
    #[serde(rename = "code")]
    pub code: Code,
    #[serde(rename = "url")]
    pub url: String,
}

impl Errorloc {
    pub fn new(code: Code, url: String) -> Errorloc {
        Errorloc {
            code,
            url,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "200")]
    Variant200,
    #[serde(rename = "400")]
    Variant400,
    #[serde(rename = "401")]
    Variant401,
    #[serde(rename = "403")]
    Variant403,
    #[serde(rename = "404")]
    Variant404,
    #[serde(rename = "405")]
    Variant405,
    #[serde(rename = "407")]
    Variant407,
    #[serde(rename = "408")]
    Variant408,
    #[serde(rename = "410")]
    Variant410,
    #[serde(rename = "413")]
    Variant413,
    #[serde(rename = "425")]
    Variant425,
    #[serde(rename = "429")]
    Variant429,
    #[serde(rename = "500")]
    Variant500,
    #[serde(rename = "501")]
    Variant501,
    #[serde(rename = "502")]
    Variant502,
    #[serde(rename = "503")]
    Variant503,
    #[serde(rename = "504")]
    Variant504,
}

impl Default for Code {
    fn default() -> Code {
        Self::Variant200
    }
}

