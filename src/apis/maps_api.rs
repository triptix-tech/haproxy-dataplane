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

/// struct for passing parameters to the method [`add_map_entry`]
#[derive(Clone, Debug)]
pub struct AddMapEntryParams {
    /// Mapfile attribute storage_name
    pub map: String,
    pub map_entry: crate::models::MapEntry,
    /// If true, immediately syncs changes to disk
    pub force_sync: Option<bool>
}

/// struct for passing parameters to the method [`add_payload_runtime_map`]
#[derive(Clone, Debug)]
pub struct AddPayloadRuntimeMapParams {
    /// Map file name
    pub name: String,
    pub map_entry: Vec<crate::models::MapEntry>,
    /// If true, immediately syncs changes to disk
    pub force_sync: Option<bool>
}

/// struct for passing parameters to the method [`clear_runtime_map`]
#[derive(Clone, Debug)]
pub struct ClearRuntimeMapParams {
    /// Map file name
    pub name: String,
    /// If true, deletes file from disk
    pub force_delete: Option<bool>,
    /// If true, immediately syncs changes to disk
    pub force_sync: Option<bool>
}

/// struct for passing parameters to the method [`delete_runtime_map_entry`]
#[derive(Clone, Debug)]
pub struct DeleteRuntimeMapEntryParams {
    /// Map id
    pub id: String,
    /// Mapfile attribute storage_name
    pub map: String,
    /// If true, immediately syncs changes to disk
    pub force_sync: Option<bool>
}

/// struct for passing parameters to the method [`get_all_runtime_map_files`]
#[derive(Clone, Debug)]
pub struct GetAllRuntimeMapFilesParams {
    /// If true, also show unmanaged map files loaded in haproxy
    pub include_unmanaged: Option<bool>
}

/// struct for passing parameters to the method [`get_one_runtime_map`]
#[derive(Clone, Debug)]
pub struct GetOneRuntimeMapParams {
    /// Map file name
    pub name: String
}

/// struct for passing parameters to the method [`get_runtime_map_entry`]
#[derive(Clone, Debug)]
pub struct GetRuntimeMapEntryParams {
    /// Map id
    pub id: String,
    /// Mapfile attribute storage_name
    pub map: String
}

/// struct for passing parameters to the method [`replace_runtime_map_entry`]
#[derive(Clone, Debug)]
pub struct ReplaceRuntimeMapEntryParams {
    /// Map id
    pub id: String,
    /// Mapfile attribute storage_name
    pub map: String,
    pub replace_runtime_map_entry_request: crate::models::ReplaceRuntimeMapEntryRequest,
    /// If true, immediately syncs changes to disk
    pub force_sync: Option<bool>
}

/// struct for passing parameters to the method [`show_runtime_map`]
#[derive(Clone, Debug)]
pub struct ShowRuntimeMapParams {
    /// Mapfile attribute storage_name
    pub map: String
}


/// struct for typed errors of method [`add_map_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddMapEntryError {
    Status400(crate::models::Error),
    Status409(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`add_payload_runtime_map`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddPayloadRuntimeMapError {
    Status400(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`clear_runtime_map`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClearRuntimeMapError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_runtime_map_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteRuntimeMapEntryError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_all_runtime_map_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetAllRuntimeMapFilesError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_one_runtime_map`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOneRuntimeMapError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_runtime_map_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetRuntimeMapEntryError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_runtime_map_entry`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceRuntimeMapEntryError {
    Status400(crate::models::Error),
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`show_runtime_map`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShowRuntimeMapError {
    Status404(crate::models::Error),
    DefaultResponse(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Adds an entry into the map file.
pub async fn add_map_entry(configuration: &configuration::Configuration, params: AddMapEntryParams) -> Result<crate::models::MapEntry, Error<AddMapEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let map = params.map;
    let map_entry = params.map_entry;
    let force_sync = params.force_sync;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps_entries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("map", &map.to_string())]);
    if let Some(ref local_var_str) = force_sync {
        local_var_req_builder = local_var_req_builder.query(&[("force_sync", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&map_entry);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddMapEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Adds a new map payload.
pub async fn add_payload_runtime_map(configuration: &configuration::Configuration, params: AddPayloadRuntimeMapParams) -> Result<Vec<crate::models::MapEntry>, Error<AddPayloadRuntimeMapError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let map_entry = params.map_entry;
    let force_sync = params.force_sync;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = force_sync {
        local_var_req_builder = local_var_req_builder.query(&[("force_sync", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&map_entry);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddPayloadRuntimeMapError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Remove all map entries from the map file.
pub async fn clear_runtime_map(configuration: &configuration::Configuration, params: ClearRuntimeMapParams) -> Result<(), Error<ClearRuntimeMapError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;
    let force_delete = params.force_delete;
    let force_sync = params.force_sync;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = force_delete {
        local_var_req_builder = local_var_req_builder.query(&[("forceDelete", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = force_sync {
        local_var_req_builder = local_var_req_builder.query(&[("force_sync", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ClearRuntimeMapError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete all the map entries from the map by its id.
pub async fn delete_runtime_map_entry(configuration: &configuration::Configuration, params: DeleteRuntimeMapEntryParams) -> Result<(), Error<DeleteRuntimeMapEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let map = params.map;
    let force_sync = params.force_sync;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps_entries/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("map", &map.to_string())]);
    if let Some(ref local_var_str) = force_sync {
        local_var_req_builder = local_var_req_builder.query(&[("force_sync", &local_var_str.to_string())]);
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
        let local_var_entity: Option<DeleteRuntimeMapEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns runtime map files.
pub async fn get_all_runtime_map_files(configuration: &configuration::Configuration, params: GetAllRuntimeMapFilesParams) -> Result<Vec<crate::models::Map>, Error<GetAllRuntimeMapFilesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let include_unmanaged = params.include_unmanaged;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = include_unmanaged {
        local_var_req_builder = local_var_req_builder.query(&[("include_unmanaged", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetAllRuntimeMapFilesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns one runtime map file.
pub async fn get_one_runtime_map(configuration: &configuration::Configuration, params: GetOneRuntimeMapParams) -> Result<crate::models::Map, Error<GetOneRuntimeMapError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let name = params.name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps/{name}", local_var_configuration.base_path, name=crate::apis::urlencode(name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetOneRuntimeMapError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns one map runtime setting by it's id.
pub async fn get_runtime_map_entry(configuration: &configuration::Configuration, params: GetRuntimeMapEntryParams) -> Result<crate::models::MapEntry, Error<GetRuntimeMapEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let map = params.map;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps_entries/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("map", &map.to_string())]);
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
        let local_var_entity: Option<GetRuntimeMapEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Replaces the value corresponding to each id in a map.
pub async fn replace_runtime_map_entry(configuration: &configuration::Configuration, params: ReplaceRuntimeMapEntryParams) -> Result<crate::models::MapEntry, Error<ReplaceRuntimeMapEntryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let id = params.id;
    let map = params.map;
    let replace_runtime_map_entry_request = params.replace_runtime_map_entry_request;
    let force_sync = params.force_sync;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps_entries/{id}", local_var_configuration.base_path, id=crate::apis::urlencode(id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("map", &map.to_string())]);
    if let Some(ref local_var_str) = force_sync {
        local_var_req_builder = local_var_req_builder.query(&[("force_sync", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };
    local_var_req_builder = local_var_req_builder.json(&replace_runtime_map_entry_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ReplaceRuntimeMapEntryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of all entries in a given runtime map file.
pub async fn show_runtime_map(configuration: &configuration::Configuration, params: ShowRuntimeMapParams) -> Result<Vec<crate::models::MapEntry>, Error<ShowRuntimeMapError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let map = params.map;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/services/haproxy/runtime/maps_entries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("map", &map.to_string())]);
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
        let local_var_entity: Option<ShowRuntimeMapError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
