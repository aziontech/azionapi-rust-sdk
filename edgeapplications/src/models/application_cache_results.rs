/*
 * Edge Application API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApplicationCacheResults {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "browser_cache_settings")]
    pub browser_cache_settings: String,
    #[serde(rename = "browser_cache_settings_maximum_ttl")]
    pub browser_cache_settings_maximum_ttl: i64,
    #[serde(rename = "cdn_cache_settings")]
    pub cdn_cache_settings: String,
    #[serde(rename = "cdn_cache_settings_maximum_ttl")]
    pub cdn_cache_settings_maximum_ttl: i64,
    #[serde(rename = "cache_by_query_string")]
    pub cache_by_query_string: String,
    #[serde(rename = "query_string_fields")]
    pub query_string_fields: Vec<String>,
    #[serde(rename = "enable_query_string_sort")]
    pub enable_query_string_sort: bool,
    #[serde(rename = "cache_by_cookies")]
    pub cache_by_cookies: String,
    #[serde(rename = "cookie_names")]
    pub cookie_names: Vec<String>,
    #[serde(rename = "adaptive_delivery_action")]
    pub adaptive_delivery_action: String,
    #[serde(rename = "device_group")]
    pub device_group: Vec<String>,
    #[serde(rename = "enable_caching_for_post")]
    pub enable_caching_for_post: bool,
    #[serde(rename = "l2_caching_enabled")]
    pub l2_caching_enabled: bool,
}

impl ApplicationCacheResults {
    pub fn new(id: i64, name: String, browser_cache_settings: String, browser_cache_settings_maximum_ttl: i64, cdn_cache_settings: String, cdn_cache_settings_maximum_ttl: i64, cache_by_query_string: String, query_string_fields: Vec<String>, enable_query_string_sort: bool, cache_by_cookies: String, cookie_names: Vec<String>, adaptive_delivery_action: String, device_group: Vec<String>, enable_caching_for_post: bool, l2_caching_enabled: bool) -> ApplicationCacheResults {
        ApplicationCacheResults {
            id,
            name,
            browser_cache_settings,
            browser_cache_settings_maximum_ttl,
            cdn_cache_settings,
            cdn_cache_settings_maximum_ttl,
            cache_by_query_string,
            query_string_fields,
            enable_query_string_sort,
            cache_by_cookies,
            cookie_names,
            adaptive_delivery_action,
            device_group,
            enable_caching_for_post,
            l2_caching_enabled,
        }
    }
}


