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

/// struct for passing parameters to the method [`create_tcp_response_rule`]
#[derive(Clone, Debug)]
pub struct CreateTcpResponseRuleParams {
    /// Parent backend name
    pub backend: String,
    pub tcp_response_rule: crate::models::TcpResponseRule,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}

/// struct for passing parameters to the method [`delete_tcp_response_rule`]
#[derive(Clone, Debug)]
pub struct DeleteTcpResponseRuleParams {
    /// TCP Response Rule Index
    pub index: i32,
    /// Parent backend name
    pub backend: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}

/// struct for passing parameters to the method [`get_tcp_response_rule`]
#[derive(Clone, Debug)]
pub struct GetTcpResponseRuleParams {
    /// TCP Response Rule Index
    pub index: i32,
    /// Parent backend name
    pub backend: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>
}

/// struct for passing parameters to the method [`get_tcp_response_rules`]
#[derive(Clone, Debug)]
pub struct GetTcpResponseRulesParams {
    /// Parent backend name
    pub backend: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>
}

/// struct for passing parameters to the method [`replace_tcp_response_rule`]
#[derive(Clone, Debug)]
pub struct ReplaceTcpResponseRuleParams {
    /// TCP Response Rule Index
    pub index: i32,
    /// Parent backend name
    pub backend: String,
    pub tcp_response_rule: crate::models::TcpResponseRule,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}


/// struct for typed errors of method [`create_tcp_response_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTcpResponseRuleError {
    Status400(crate::models::Error),
    Status409(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_tcp_response_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteTcpResponseRuleError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tcp_response_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTcpResponseRuleError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_tcp_response_rules`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTcpResponseRulesError {
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_tcp_response_rule`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceTcpResponseRuleError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Adds a new TCP Response Rule of the specified type in the specified backend.
pub async fn create_tcp_response_rule(configuration: &configuration::Configuration, params: CreateTcpResponseRuleParams) -> Result<crate::models::TcpResponseRule, Error<CreateTcpResponseRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let backend = params.backend;
    let tcp_response_rule = params.tcp_response_rule;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/tcp_response_rules", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("backend", &backend.to_string())]);
    if let Some(ref local_var_str) = transaction_id {
        local_var_req_builder = local_var_req_builder.query(&[("transaction_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = version {
        local_var_req_builder = local_var_req_builder.query(&[("version", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force_reload {
        local_var_req_builder = local_var_req_builder.query(&[("force_reload", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&tcp_response_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTcpResponseRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a TCP Response Rule configuration by it's index from the specified backend.
pub async fn delete_tcp_response_rule(configuration: &configuration::Configuration, params: DeleteTcpResponseRuleParams) -> Result<(), Error<DeleteTcpResponseRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let index = params.index;
    let backend = params.backend;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/tcp_response_rules/{index}", local_var_configuration.base_path, index=index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("backend", &backend.to_string())]);
    if let Some(ref local_var_str) = transaction_id {
        local_var_req_builder = local_var_req_builder.query(&[("transaction_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = version {
        local_var_req_builder = local_var_req_builder.query(&[("version", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force_reload {
        local_var_req_builder = local_var_req_builder.query(&[("force_reload", &local_var_str.to_string())]);
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
        Ok(())
    } else {
        let local_var_entity: Option<DeleteTcpResponseRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns one TCP Response Rule configuration by it's index in the specified backend.
pub async fn get_tcp_response_rule(configuration: &configuration::Configuration, params: GetTcpResponseRuleParams) -> Result<crate::models::GetTcpResponseRule200Response, Error<GetTcpResponseRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let index = params.index;
    let backend = params.backend;
    let transaction_id = params.transaction_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/tcp_response_rules/{index}", local_var_configuration.base_path, index=index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("backend", &backend.to_string())]);
    if let Some(ref local_var_str) = transaction_id {
        local_var_req_builder = local_var_req_builder.query(&[("transaction_id", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetTcpResponseRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all TCP Response Rules that are configured in specified backend.
pub async fn get_tcp_response_rules(configuration: &configuration::Configuration, params: GetTcpResponseRulesParams) -> Result<crate::models::GetTcpResponseRules200Response, Error<GetTcpResponseRulesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let backend = params.backend;
    let transaction_id = params.transaction_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/tcp_response_rules", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("backend", &backend.to_string())]);
    if let Some(ref local_var_str) = transaction_id {
        local_var_req_builder = local_var_req_builder.query(&[("transaction_id", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetTcpResponseRulesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replaces a TCP Response Rule configuration by it's Index in the specified backend.
pub async fn replace_tcp_response_rule(configuration: &configuration::Configuration, params: ReplaceTcpResponseRuleParams) -> Result<crate::models::TcpResponseRule, Error<ReplaceTcpResponseRuleError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let index = params.index;
    let backend = params.backend;
    let tcp_response_rule = params.tcp_response_rule;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/tcp_response_rules/{index}", local_var_configuration.base_path, index=index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("backend", &backend.to_string())]);
    if let Some(ref local_var_str) = transaction_id {
        local_var_req_builder = local_var_req_builder.query(&[("transaction_id", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = version {
        local_var_req_builder = local_var_req_builder.query(&[("version", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force_reload {
        local_var_req_builder = local_var_req_builder.query(&[("force_reload", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&tcp_response_rule);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceTcpResponseRuleError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
