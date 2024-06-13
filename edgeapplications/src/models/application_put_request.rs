/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationPutRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "delivery_protocol", skip_serializing_if = "Option::is_none")]
    pub delivery_protocol: Option<String>,
    #[serde(rename = "http_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub http_port: Option<Option<serde_json::Value>>,
    #[serde(rename = "https_port", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub https_port: Option<Option<serde_json::Value>>,
    #[serde(rename = "minimum_tls_version", skip_serializing_if = "Option::is_none")]
    pub minimum_tls_version: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "application_acceleration", skip_serializing_if = "Option::is_none")]
    pub application_acceleration: Option<bool>,
    #[serde(rename = "device_detection", skip_serializing_if = "Option::is_none")]
    pub device_detection: Option<bool>,
    #[serde(rename = "edge_firewall", skip_serializing_if = "Option::is_none")]
    pub edge_firewall: Option<bool>,
    #[serde(rename = "edge_functions", skip_serializing_if = "Option::is_none")]
    pub edge_functions: Option<bool>,
    #[serde(rename = "image_optimization", skip_serializing_if = "Option::is_none")]
    pub image_optimization: Option<bool>,
    #[serde(rename = "l2_caching", skip_serializing_if = "Option::is_none")]
    pub l2_caching: Option<bool>,
    #[serde(rename = "load_balancer", skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<bool>,
    #[serde(rename = "raw_logs", skip_serializing_if = "Option::is_none")]
    pub raw_logs: Option<bool>,
    #[serde(rename = "web_application_firewall", skip_serializing_if = "Option::is_none")]
    pub web_application_firewall: Option<bool>,
    #[serde(rename = "debug_rules", skip_serializing_if = "Option::is_none")]
    pub debug_rules: Option<bool>,
    #[serde(rename = "http3", skip_serializing_if = "Option::is_none")]
    pub http3: Option<bool>,
    #[serde(rename = "websocket", skip_serializing_if = "Option::is_none")]
    pub websocket: Option<bool>,
    #[serde(rename = "supported_ciphers", skip_serializing_if = "Option::is_none")]
    pub supported_ciphers: Option<String>,
}

impl ApplicationPutRequest {
    pub fn new(name: String) -> ApplicationPutRequest {
        ApplicationPutRequest {
            name,
            delivery_protocol: None,
            http_port: None,
            https_port: None,
            minimum_tls_version: None,
            active: None,
            application_acceleration: None,
            device_detection: None,
            edge_firewall: None,
            edge_functions: None,
            image_optimization: None,
            l2_caching: None,
            load_balancer: None,
            raw_logs: None,
            web_application_firewall: None,
            debug_rules: None,
            http3: None,
            websocket: None,
            supported_ciphers: None,
        }
    }
}

