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

/// struct for passing parameters to the method [`create_acl`]
#[derive(Default, Clone, Debug)]
pub struct CreateAclParams {
    /// Parent name
    pub parent_name: String,
    /// Parent type
    pub parent_type: String,
    pub acl: crate::models::Acl,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}

/// struct for passing parameters to the method [`delete_acl`]
#[derive(Default, Clone, Debug)]
pub struct DeleteAclParams {
    /// ACL line Index
    pub index: i32,
    /// Parent name
    pub parent_name: String,
    /// Parent type
    pub parent_type: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}

/// struct for passing parameters to the method [`get_acl`]
#[derive(Default, Clone, Debug)]
pub struct GetAclParams {
    /// ACL line Index
    pub index: i32,
    /// Parent name
    pub parent_name: String,
    /// Parent type
    pub parent_type: String,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>
}

/// struct for passing parameters to the method [`get_acls`]
#[derive(Default, Clone, Debug)]
pub struct GetAclsParams {
    /// Parent name
    pub parent_name: String,
    /// Parent type
    pub parent_type: String,
    /// ACL name
    pub acl_name: Option<String>,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>
}

/// struct for passing parameters to the method [`replace_acl`]
#[derive(Default, Clone, Debug)]
pub struct ReplaceAclParams {
    /// ACL line Index
    pub index: i32,
    /// Parent name
    pub parent_name: String,
    /// Parent type
    pub parent_type: String,
    pub acl: crate::models::Acl,
    /// ID of the transaction where we want to add the operation. Cannot be used when version is specified.
    pub transaction_id: Option<String>,
    /// Version used for checking configuration version. Cannot be used when transaction is specified, transaction has it's own version.
    pub version: Option<i32>,
    /// If set, do a force reload, do not wait for the configured reload-delay. Cannot be used when transaction is specified, as changes in transaction are not applied directly to configuration.
    pub force_reload: Option<bool>
}


/// struct for typed errors of method [`create_acl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAclError {
    Status400(crate::models::Error),
    Status409(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_acl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAclError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_acl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAclError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_acls`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAclsError {
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_acl`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceAclError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Adds a new ACL line of the specified type in the specified parent.
pub async fn create_acl(configuration: &configuration::Configuration, params: CreateAclParams) -> Result<crate::models::Acl, Error<CreateAclError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let parent_name = params.parent_name;
    let parent_type = params.parent_type;
    let acl = params.acl;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/acls", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("parent_name", &parent_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("parent_type", &parent_type.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&acl);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAclError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Deletes a ACL line configuration by it's index from the specified parent.
pub async fn delete_acl(configuration: &configuration::Configuration, params: DeleteAclParams) -> Result<(), Error<DeleteAclError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let index = params.index;
    let parent_name = params.parent_name;
    let parent_type = params.parent_type;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/acls/{index}", local_var_configuration.base_path, index=index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("parent_name", &parent_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("parent_type", &parent_type.to_string())]);
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
        let local_var_entity: Option<DeleteAclError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns one ACL line configuration by it's index in the specified parent.
pub async fn get_acl(configuration: &configuration::Configuration, params: GetAclParams) -> Result<crate::models::GetAcl200Response, Error<GetAclError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let index = params.index;
    let parent_name = params.parent_name;
    let parent_type = params.parent_type;
    let transaction_id = params.transaction_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/acls/{index}", local_var_configuration.base_path, index=index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("parent_name", &parent_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("parent_type", &parent_type.to_string())]);
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
        let local_var_entity: Option<GetAclError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all ACL lines that are configured in specified parent.
pub async fn get_acls(configuration: &configuration::Configuration, params: GetAclsParams) -> Result<crate::models::GetAcls200Response, Error<GetAclsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let parent_name = params.parent_name;
    let parent_type = params.parent_type;
    let acl_name = params.acl_name;
    let transaction_id = params.transaction_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/acls", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("parent_name", &parent_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("parent_type", &parent_type.to_string())]);
    if let Some(ref local_var_str) = acl_name {
        local_var_req_builder = local_var_req_builder.query(&[("acl_name", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetAclsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replaces a ACL line configuration by it's index in the specified parent.
pub async fn replace_acl(configuration: &configuration::Configuration, params: ReplaceAclParams) -> Result<crate::models::Acl, Error<ReplaceAclError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let index = params.index;
    let parent_name = params.parent_name;
    let parent_type = params.parent_type;
    let acl = params.acl;
    let transaction_id = params.transaction_id;
    let version = params.version;
    let force_reload = params.force_reload;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/configuration/acls/{index}", local_var_configuration.base_path, index=index);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("parent_name", &parent_name.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("parent_type", &parent_type.to_string())]);
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
    local_var_req_builder = local_var_req_builder.json(&acl);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceAclError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

