/*
 * Data Streaming - OpenAPI
 *
 * The Data Streaming API allows you to manage your existing data streamings and templates. Data Streaming allows you to feed your stream processing, SIEM, and big data platforms with the event logs from your applications on Azion in real time. 
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateNewDataStreamingRequest {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Options:  * `2` - Edge Applications Event Collector  * `4` - WAF Event Collector  * `86` - Edge Functions Event Collector  * `184` - Edge Applications + WAF Event Collector  * `251` - Activity History Collector 
    #[serde(rename = "template_id", skip_serializing_if = "Option::is_none")]
    pub template_id: Option<TemplateId>,
    /// Options:  * `http` - Edge Applications (default)  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History   
    #[serde(rename = "data_source", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data_source: Option<Option<DataSource>>,
    #[serde(rename = "active", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub active: Option<Option<bool>>,
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Box<crate::models::DataStreamingEndpointTypeStandard>>,
    /// Note:  * Field not used with the rtm_activity data source. 
    #[serde(rename = "domains_ids", skip_serializing_if = "Option::is_none")]
    pub domains_ids: Option<Vec<i32>>,
    /// Note:  * Field not used with the rtm_activity data source. 
    #[serde(rename = "all_domains", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub all_domains: Option<Option<bool>>,
    /// Note:  * `Range` - From 0 to 100.  * `To use:` [Contact the sales team](https://www.azion.com/en/contact-sales/) to activate this feature in your account. 
    #[serde(rename = "sampling_percentage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<Option<i32>>,
    /// Note:  * Add all variables and values that will be used to stream according to the data source you choose to use.    * All data streaming [variables can be found on the reference documentation](https://www.azion.com/en/documentation/products/data-streaming/#selecting-data-sources).   
    #[serde(rename = "template_model", skip_serializing_if = "Option::is_none")]
    pub template_model: Option<String>,
}

impl CreateNewDataStreamingRequest {
    pub fn new() -> CreateNewDataStreamingRequest {
        CreateNewDataStreamingRequest {
            name: None,
            template_id: None,
            data_source: None,
            active: None,
            endpoint: None,
            domains_ids: None,
            all_domains: None,
            sampling_percentage: None,
            template_model: None,
        }
    }
}

/// Options:  * `2` - Edge Applications Event Collector  * `4` - WAF Event Collector  * `86` - Edge Functions Event Collector  * `184` - Edge Applications + WAF Event Collector  * `251` - Activity History Collector 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TemplateId {
    #[serde(rename = "2")]
    Variant2,
    #[serde(rename = "4")]
    Variant4,
    #[serde(rename = "86")]
    Variant86,
    #[serde(rename = "184")]
    Variant184,
    #[serde(rename = "251")]
    Variant251,
}

impl Default for TemplateId {
    fn default() -> TemplateId {
        Self::Variant2
    }
}
/// Options:  * `http` - Edge Applications (default)  * `waf` - WAF Events  * `cells_console` - Edge Functions  * `rtm_activity` - Activity History   
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataSource {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "waf")]
    Waf,
    #[serde(rename = "cells_console")]
    CellsConsole,
    #[serde(rename = "rtm_activity")]
    RtmActivity,
}

impl Default for DataSource {
    fn default() -> DataSource {
        Self::Http
    }
}

