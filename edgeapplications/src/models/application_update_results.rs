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
pub struct ApplicationUpdateResults {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "delivery_protocol")]
    pub delivery_protocol: String,
    #[serde(rename = "http_port", deserialize_with = "Option::deserialize")]
    pub http_port: Option<serde_json::Value>,
    #[serde(rename = "https_port", deserialize_with = "Option::deserialize")]
    pub https_port: Option<serde_json::Value>,
    #[serde(rename = "minimum_tls_version")]
    pub minimum_tls_version: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "debug_rules")]
    pub debug_rules: bool,
    #[serde(rename = "http3")]
    pub http3: bool,
    #[serde(rename = "supported_ciphers")]
    pub supported_ciphers: String,
    #[serde(rename = "application_acceleration")]
    pub application_acceleration: bool,
    #[serde(rename = "caching")]
    pub caching: bool,
    #[serde(rename = "device_detection")]
    pub device_detection: bool,
    #[serde(rename = "edge_firewall")]
    pub edge_firewall: bool,
    #[serde(rename = "edge_functions")]
    pub edge_functions: bool,
    #[serde(rename = "image_optimization")]
    pub image_optimization: bool,
    #[serde(rename = "l2_caching")]
    pub l2_caching: bool,
    #[serde(rename = "load_balancer")]
    pub load_balancer: bool,
    #[serde(rename = "raw_logs")]
    pub raw_logs: bool,
    #[serde(rename = "web_application_firewall")]
    pub web_application_firewall: bool,
    #[serde(rename = "websocket", skip_serializing_if = "Option::is_none")]
    pub websocket: Option<bool>,
}

impl ApplicationUpdateResults {
    pub fn new(id: i64, name: String, delivery_protocol: String, http_port: Option<serde_json::Value>, https_port: Option<serde_json::Value>, minimum_tls_version: String, active: bool, debug_rules: bool, http3: bool, supported_ciphers: String, application_acceleration: bool, caching: bool, device_detection: bool, edge_firewall: bool, edge_functions: bool, image_optimization: bool, l2_caching: bool, load_balancer: bool, raw_logs: bool, web_application_firewall: bool) -> ApplicationUpdateResults {
        ApplicationUpdateResults {
            id,
            name,
            delivery_protocol,
            http_port,
            https_port,
            minimum_tls_version,
            active,
            debug_rules,
            http3,
            supported_ciphers,
            application_acceleration,
            caching,
            device_detection,
            edge_firewall,
            edge_functions,
            image_optimization,
            l2_caching,
            load_balancer,
            raw_logs,
            web_application_firewall,
            websocket: None,
        }
    }
}

