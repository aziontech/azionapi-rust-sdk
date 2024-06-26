/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`edge_applications_edge_application_id_functions_instances_functions_instances_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdDeleteError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_functions_instances_functions_instances_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdGetError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_functions_instances_functions_instances_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdPatchError {
    Status400(),
    Status403(),
    Status404(),
    Status405(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_functions_instances_functions_instances_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdPutError {
    Status400(),
    Status403(),
    Status404(),
    Status405(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_functions_instances_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdFunctionsInstancesGetError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_functions_instances_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdFunctionsInstancesPostError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}


pub async fn edge_applications_edge_application_id_functions_instances_functions_instances_id_delete(configuration: &configuration::Configuration, edge_application_id: &str, functions_instances_id: &str, accept: Option<&str>, content_type: Option<&str>) -> Result<(), Error<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdDeleteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/edge_applications/{edge_application_id}/functions_instances/{functions_instances_id}", local_var_configuration.base_path, edge_application_id=crate::apis::urlencode(edge_application_id), functions_instances_id=crate::apis::urlencode(functions_instances_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept {
        local_var_req_builder = local_var_req_builder.header("Accept", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = content_type {
        local_var_req_builder = local_var_req_builder.header("Content-Type", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdDeleteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn edge_applications_edge_application_id_functions_instances_functions_instances_id_get(configuration: &configuration::Configuration, edge_application_id: i64, functions_instances_id: i64, accept: Option<&str>) -> Result<models::ApplicationInstancesGetOneResponse, Error<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/edge_applications/{edge_application_id}/functions_instances/{functions_instances_id}", local_var_configuration.base_path, edge_application_id=edge_application_id, functions_instances_id=functions_instances_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept {
        local_var_req_builder = local_var_req_builder.header("Accept", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn edge_applications_edge_application_id_functions_instances_functions_instances_id_patch(configuration: &configuration::Configuration, edge_application_id: &str, functions_instances_id: &str, accept: Option<&str>, content_type: Option<&str>, application_update_instance_request: Option<models::ApplicationUpdateInstanceRequest>) -> Result<models::ApplicationInstanceResults, Error<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdPatchError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/edge_applications/{edge_application_id}/functions_instances/{functions_instances_id}", local_var_configuration.base_path, edge_application_id=crate::apis::urlencode(edge_application_id), functions_instances_id=crate::apis::urlencode(functions_instances_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept {
        local_var_req_builder = local_var_req_builder.header("Accept", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = content_type {
        local_var_req_builder = local_var_req_builder.header("Content-Type", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&application_update_instance_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdPatchError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn edge_applications_edge_application_id_functions_instances_functions_instances_id_put(configuration: &configuration::Configuration, edge_application_id: &str, functions_instances_id: &str, accept: Option<&str>, content_type: Option<&str>, application_put_instance_request: Option<models::ApplicationPutInstanceRequest>) -> Result<models::ApplicationInstanceResults, Error<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdPutError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/edge_applications/{edge_application_id}/functions_instances/{functions_instances_id}", local_var_configuration.base_path, edge_application_id=crate::apis::urlencode(edge_application_id), functions_instances_id=crate::apis::urlencode(functions_instances_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept {
        local_var_req_builder = local_var_req_builder.header("Accept", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = content_type {
        local_var_req_builder = local_var_req_builder.header("Content-Type", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&application_put_instance_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EdgeApplicationsEdgeApplicationIdFunctionsInstancesFunctionsInstancesIdPutError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn edge_applications_edge_application_id_functions_instances_get(configuration: &configuration::Configuration, edge_application_id: i64, page: Option<i64>, page_size: Option<i64>, filter: Option<&str>, order_by: Option<&str>, sort: Option<&str>, accept: Option<&str>) -> Result<models::ApplicationInstancesGetResponse, Error<EdgeApplicationsEdgeApplicationIdFunctionsInstancesGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/edge_applications/{edge_application_id}/functions_instances", local_var_configuration.base_path, edge_application_id=edge_application_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("page_size", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter {
        local_var_req_builder = local_var_req_builder.query(&[("filter", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = order_by {
        local_var_req_builder = local_var_req_builder.query(&[("order_by", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept {
        local_var_req_builder = local_var_req_builder.header("Accept", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EdgeApplicationsEdgeApplicationIdFunctionsInstancesGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn edge_applications_edge_application_id_functions_instances_post(configuration: &configuration::Configuration, edge_application_id: i64, accept: Option<&str>, content_type: Option<&str>, application_create_instance_request: Option<models::ApplicationCreateInstanceRequest>) -> Result<models::ApplicationInstanceResults, Error<EdgeApplicationsEdgeApplicationIdFunctionsInstancesPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/edge_applications/{edge_application_id}/functions_instances", local_var_configuration.base_path, edge_application_id=edge_application_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = accept {
        local_var_req_builder = local_var_req_builder.header("Accept", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = content_type {
        local_var_req_builder = local_var_req_builder.header("Content-Type", local_var_param_value.to_string());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Authorization", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&application_create_instance_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<EdgeApplicationsEdgeApplicationIdFunctionsInstancesPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

