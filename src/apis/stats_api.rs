/*
 * HAProxy Data Plane API
 *
 * API for editing and managing haproxy instances. Provides process information, configuration management, haproxy stats and logs. 
 *
 * The version of the OpenAPI document: 2.9
 * Contact: support@haproxy.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`get_stats`]
#[derive(Default, Clone, Debug)]
pub struct GetStatsParams {
    /// Object type to get stats for (one of frontend, backend, server)
    pub r#type: Option<String>,
    /// Object name to get stats for
    pub name: Option<String>,
    /// Object parent name to get stats for, in case the object is a server
    pub parent: Option<String>
}


/// struct for typed errors of method [`get_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetStatsError {
    Status500(Vec<crate::models::NativeStatsCollection>),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Getting stats from the HAProxy.
pub async fn get_stats(configuration: &configuration::Configuration, params: GetStatsParams) -> Result<Vec<crate::models::NativeStatsCollection>, Error<GetStatsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let r#type = params.r#type;
    let name = params.name;
    let parent = params.parent;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/stats/native", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = r#type {
        local_var_req_builder = local_var_req_builder.query(&[("type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = name {
        local_var_req_builder = local_var_req_builder.query(&[("name", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = parent {
        local_var_req_builder = local_var_req_builder.query(&[("parent", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetStatsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

