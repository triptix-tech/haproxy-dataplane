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

/// struct for passing parameters to the method [`create_ring`]
#[derive(Default, Clone, Debug)]
pub struct CreateRingParams {
    pub ring: crate::models::Ring,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}

/// struct for passing parameters to the method [`delete_ring`]
#[derive(Default, Clone, Debug)]
pub struct DeleteRingParams {
    /// Ring name
    pub name: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}

/// struct for passing parameters to the method [`get_ring`]
#[derive(Default, Clone, Debug)]
pub struct GetRingParams {
    /// Ring name
    pub name: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>
}

/// struct for passing parameters to the method [`get_rings`]
#[derive(Default, Clone, Debug)]
pub struct GetRingsParams {
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>
}

/// struct for passing parameters to the method [`replace_ring`]
#[derive(Default, Clone, Debug)]
pub struct ReplaceRingParams {
    /// Ring name
    pub name: String,
    pub ring: crate::models::Ring,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}


/// struct for typed errors of method [`create_ring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRingError {
    Status400(crate::models::Error),
    Status409(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_ring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRingError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRingError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_rings`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRingsError {
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_ring`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceRingError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Adds a new ring to the configuration file.
pub async fn create_ring(configuration: &configuration::Configuration, params: CreateRingParams) -> Result<crate::models::Ring, Error<CreateRingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let ring = params.ring;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/rings", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&ring);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateRingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a ring from the configuration by it's name.
pub async fn delete_ring(configuration: &configuration::Configuration, params: DeleteRingParams) -> Result<(), Error<DeleteRingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/rings/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

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
        let local_var_entity: Option<DeleteRingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns one ring configuration by it's name.
pub async fn get_ring(configuration: &configuration::Configuration, params: GetRingParams) -> Result<crate::models::GetRing200Response, Error<GetRingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let transaction_id = params.transaction_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/rings/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetRingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of all configured rings.
pub async fn get_rings(configuration: &configuration::Configuration, params: GetRingsParams) -> Result<crate::models::GetRings200Response, Error<GetRingsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let transaction_id = params.transaction_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/rings", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetRingsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replaces a ring configuration by it's name.
pub async fn replace_ring(configuration: &configuration::Configuration, params: ReplaceRingParams) -> Result<crate::models::Ring, Error<ReplaceRingError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let ring = params.ring;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/rings/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&ring);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceRingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

