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
pub struct AwsFilters {
    /// Key to use as filter, using the format specified at https://docs.aws.amazon.com/cli/latest/reference/ec2/describe-instances.html#options
    #[serde(rename = "key")]
    pub key: String,
    /// Value of the filter to use
    #[serde(rename = "value")]
    pub value: String,
}

impl AwsFilters {
    pub fn new(key: String, value: String) -> AwsFilters {
        AwsFilters {
            key,
            value,
        }
    }
}


