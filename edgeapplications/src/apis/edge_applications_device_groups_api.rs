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
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`edge_applications_edge_application_id_device_groups_device_group_id_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdDeleteError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_device_groups_device_group_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdGetError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_device_groups_device_group_id_patch`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdPatchError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_device_groups_device_group_id_put`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdPutError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_device_groups_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdDeviceGroupsGetError {
    Status400(),
    Status403(),
    Status404(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`edge_applications_edge_application_id_device_groups_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EdgeApplicationsEdgeApplicationIdDeviceGroupsPostError {
    Status400(),
    Status403(),
    Status404(),
    Status415(),
    Status422(),
    Status500(),
    UnknownValue(serde_json::Value),
}


pub async fn edge_applications_edge_application_id_device_groups_device_group_id_delete(configuration: &configuration::Configuration, edge_application_id: i64, device_group_id: i64, accept: Option<&str>) -> Result<(), Error<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdDeleteError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_edge_application_id = edge_application_id;
    let p_device_group_id = device_group_id;
    let p_accept = accept;

    let uri_str = format!("{}/edge_applications/{edge_application_id}/device_groups/{device_group_id}", configuration.base_path, edge_application_id=p_edge_application_id, device_group_id=p_device_group_id);
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text().await?;
        let entity: Option<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdDeleteError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edge_applications_edge_application_id_device_groups_device_group_id_get(configuration: &configuration::Configuration, edge_application_id: i64, device_group_id: i64, accept: Option<&str>) -> Result<models::DeviceGroupsIdResponse, Error<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_edge_application_id = edge_application_id;
    let p_device_group_id = device_group_id;
    let p_accept = accept;

    let uri_str = format!("{}/edge_applications/{edge_application_id}/device_groups/{device_group_id}", configuration.base_path, edge_application_id=p_edge_application_id, device_group_id=p_device_group_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeviceGroupsIdResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeviceGroupsIdResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edge_applications_edge_application_id_device_groups_device_group_id_patch(configuration: &configuration::Configuration, edge_application_id: i64, device_group_id: i64, accept: Option<&str>, content_type: Option<&str>, patch_device_groups_request: Option<models::PatchDeviceGroupsRequest>) -> Result<models::DeviceGroupsIdResponse, Error<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdPatchError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_edge_application_id = edge_application_id;
    let p_device_group_id = device_group_id;
    let p_accept = accept;
    let p_content_type = content_type;
    let p_patch_device_groups_request = patch_device_groups_request;

    let uri_str = format!("{}/edge_applications/{edge_application_id}/device_groups/{device_group_id}", configuration.base_path, edge_application_id=p_edge_application_id, device_group_id=p_device_group_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PATCH, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(param_value) = p_content_type {
        req_builder = req_builder.header("Content-Type", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_patch_device_groups_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeviceGroupsIdResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeviceGroupsIdResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdPatchError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edge_applications_edge_application_id_device_groups_device_group_id_put(configuration: &configuration::Configuration, edge_application_id: i64, device_group_id: i64, accept: Option<&str>, content_type: Option<&str>, update_device_groups_request: Option<models::UpdateDeviceGroupsRequest>) -> Result<models::DeviceGroupsIdResponse, Error<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdPutError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_edge_application_id = edge_application_id;
    let p_device_group_id = device_group_id;
    let p_accept = accept;
    let p_content_type = content_type;
    let p_update_device_groups_request = update_device_groups_request;

    let uri_str = format!("{}/edge_applications/{edge_application_id}/device_groups/{device_group_id}", configuration.base_path, edge_application_id=p_edge_application_id, device_group_id=p_device_group_id);
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(param_value) = p_content_type {
        req_builder = req_builder.header("Content-Type", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_update_device_groups_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeviceGroupsIdResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeviceGroupsIdResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EdgeApplicationsEdgeApplicationIdDeviceGroupsDeviceGroupIdPutError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edge_applications_edge_application_id_device_groups_get(configuration: &configuration::Configuration, edge_application_id: i64, page: Option<i64>, page_size: Option<i64>, filter: Option<&str>, order_by: Option<&str>, sort: Option<&str>, accept: Option<&str>) -> Result<models::DeviceGroupsResponse, Error<EdgeApplicationsEdgeApplicationIdDeviceGroupsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_edge_application_id = edge_application_id;
    let p_page = page;
    let p_page_size = page_size;
    let p_filter = filter;
    let p_order_by = order_by;
    let p_sort = sort;
    let p_accept = accept;

    let uri_str = format!("{}/edge_applications/{edge_application_id}/device_groups", configuration.base_path, edge_application_id=p_edge_application_id);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page_size {
        req_builder = req_builder.query(&[("page_size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_filter {
        req_builder = req_builder.query(&[("filter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_order_by {
        req_builder = req_builder.query(&[("order_by", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeviceGroupsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeviceGroupsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EdgeApplicationsEdgeApplicationIdDeviceGroupsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn edge_applications_edge_application_id_device_groups_post(configuration: &configuration::Configuration, edge_application_id: i64, accept: Option<&str>, content_type: Option<&str>, create_device_groups_request: Option<models::CreateDeviceGroupsRequest>) -> Result<models::DeviceGroupsIdResponse, Error<EdgeApplicationsEdgeApplicationIdDeviceGroupsPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_edge_application_id = edge_application_id;
    let p_accept = accept;
    let p_content_type = content_type;
    let p_create_device_groups_request = create_device_groups_request;

    let uri_str = format!("{}/edge_applications/{edge_application_id}/device_groups", configuration.base_path, edge_application_id=p_edge_application_id);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = p_accept {
        req_builder = req_builder.header("Accept", param_value.to_string());
    }
    if let Some(param_value) = p_content_type {
        req_builder = req_builder.header("Content-Type", param_value.to_string());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("Authorization", value);
    };
    req_builder = req_builder.json(&p_create_device_groups_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeviceGroupsIdResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeviceGroupsIdResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<EdgeApplicationsEdgeApplicationIdDeviceGroupsPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

