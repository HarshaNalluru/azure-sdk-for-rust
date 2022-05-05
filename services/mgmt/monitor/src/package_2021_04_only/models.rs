#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Azure Monitor Metrics destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureMonitorMetricsDestination {
    #[doc = "A friendly name for the destination. \r\nThis name should be unique across all destinations (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl AzureMonitorMetricsDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the endpoint used for accessing configuration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationAccessEndpointSpec {
    #[doc = "The endpoint. This property is READ-ONLY."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl ConfigurationAccessEndpointSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of data collection endpoint."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionEndpoint {
    #[doc = "Description of the data collection endpoint."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The immutable ID of this data collection endpoint resource. This property is READ-ONLY."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[doc = "The endpoint used by agents to access their configuration."]
    #[serde(rename = "configurationAccess", default, skip_serializing_if = "Option::is_none")]
    pub configuration_access: Option<serde_json::Value>,
    #[doc = "The endpoint used by clients to ingest logs."]
    #[serde(rename = "logsIngestion", default, skip_serializing_if = "Option::is_none")]
    pub logs_ingestion: Option<serde_json::Value>,
    #[doc = "Network access control rules for the endpoints."]
    #[serde(rename = "networkAcls", default, skip_serializing_if = "Option::is_none")]
    pub network_acls: Option<serde_json::Value>,
    #[doc = "The resource provisioning state. This property is READ-ONLY."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_collection_endpoint::ProvisioningState>,
}
impl DataCollectionEndpoint {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_collection_endpoint {
    use super::*;
    #[doc = "The resource provisioning state. This property is READ-ONLY."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
    }
}
#[doc = "Definition of ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionEndpointResource {
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The kind of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<data_collection_endpoint_resource::Kind>,
    #[doc = "Fully qualified ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource entity tag (ETag)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DataCollectionEndpointResource {
    pub fn new(location: String) -> Self {
        Self {
            properties: None,
            location,
            tags: None,
            kind: None,
            id: None,
            name: None,
            type_: None,
            etag: None,
            system_data: None,
        }
    }
}
pub mod data_collection_endpoint_resource {
    use super::*;
    #[doc = "The kind of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Linux,
        Windows,
    }
}
#[doc = "A pageable list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionEndpointResourceListResult {
    #[doc = "A list of resources."]
    pub value: Vec<DataCollectionEndpointResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataCollectionEndpointResourceListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl DataCollectionEndpointResourceListResult {
    pub fn new(value: Vec<DataCollectionEndpointResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Definition of what monitoring data to collect and where that data should be sent."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionRule {
    #[doc = "Description of the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The immutable ID of this data collection rule. This property is READ-ONLY."]
    #[serde(rename = "immutableId", default, skip_serializing_if = "Option::is_none")]
    pub immutable_id: Option<String>,
    #[doc = "The specification of data sources. \r\nThis property is optional and can be omitted if the rule is meant to be used via direct calls to the provisioned endpoint."]
    #[serde(rename = "dataSources", default, skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<serde_json::Value>,
    #[doc = "The specification of destinations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<serde_json::Value>,
    #[doc = "The specification of data flows."]
    #[serde(rename = "dataFlows", default, skip_serializing_if = "Vec::is_empty")]
    pub data_flows: Vec<DataFlow>,
    #[doc = "The resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_collection_rule::ProvisioningState>,
}
impl DataCollectionRule {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_collection_rule {
    use super::*;
    #[doc = "The resource provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
    }
}
#[doc = "Definition of association of a data collection rule with a monitored Azure resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionRuleAssociation {
    #[doc = "Description of the association."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The resource ID of the data collection rule that is to be associated."]
    #[serde(rename = "dataCollectionRuleId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_rule_id: Option<String>,
    #[doc = "The resource ID of the data collection endpoint that is to be associated."]
    #[serde(rename = "dataCollectionEndpointId", default, skip_serializing_if = "Option::is_none")]
    pub data_collection_endpoint_id: Option<String>,
    #[doc = "The resource provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<data_collection_rule_association::ProvisioningState>,
}
impl DataCollectionRuleAssociation {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod data_collection_rule_association {
    use super::*;
    #[doc = "The resource provisioning state."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        Creating,
        Updating,
        Deleting,
        Succeeded,
        Failed,
    }
}
#[doc = "Definition of generic ARM proxy resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataCollectionRuleAssociationProxyOnlyResource {
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "Fully qualified ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource entity tag (ETag)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DataCollectionRuleAssociationProxyOnlyResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A pageable list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionRuleAssociationProxyOnlyResourceListResult {
    #[doc = "A list of resources."]
    pub value: Vec<DataCollectionRuleAssociationProxyOnlyResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataCollectionRuleAssociationProxyOnlyResourceListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl DataCollectionRuleAssociationProxyOnlyResourceListResult {
    pub fn new(value: Vec<DataCollectionRuleAssociationProxyOnlyResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Definition of ARM tracked top level resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionRuleResource {
    #[doc = "Resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The geo-location where the resource lives."]
    pub location: String,
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The kind of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<data_collection_rule_resource::Kind>,
    #[doc = "Fully qualified ID of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource entity tag (ETag)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[doc = "Metadata pertaining to creation and last modification of the resource."]
    #[serde(rename = "systemData", default, skip_serializing_if = "Option::is_none")]
    pub system_data: Option<serde_json::Value>,
}
impl DataCollectionRuleResource {
    pub fn new(location: String) -> Self {
        Self {
            properties: None,
            location,
            tags: None,
            kind: None,
            id: None,
            name: None,
            type_: None,
            etag: None,
            system_data: None,
        }
    }
}
pub mod data_collection_rule_resource {
    use super::*;
    #[doc = "The kind of the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        Linux,
        Windows,
    }
}
#[doc = "A pageable list of resources."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataCollectionRuleResourceListResult {
    #[doc = "A list of resources."]
    pub value: Vec<DataCollectionRuleResource>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DataCollectionRuleResourceListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl DataCollectionRuleResourceListResult {
    pub fn new(value: Vec<DataCollectionRuleResource>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Definition of which streams are sent to which destinations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataFlow {
    #[doc = "List of streams for this data flow."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<String>,
    #[doc = "List of destinations for this data flow."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub destinations: Vec<String>,
}
impl DataFlow {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification of data sources that will be collected."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DataSourcesSpec {
    #[doc = "The list of performance counter data source configurations."]
    #[serde(rename = "performanceCounters", default, skip_serializing_if = "Vec::is_empty")]
    pub performance_counters: Vec<PerfCounterDataSource>,
    #[doc = "The list of Windows Event Log data source configurations."]
    #[serde(rename = "windowsEventLogs", default, skip_serializing_if = "Vec::is_empty")]
    pub windows_event_logs: Vec<WindowsEventLogDataSource>,
    #[doc = "The list of Syslog data source configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub syslog: Vec<SyslogDataSource>,
    #[doc = "The list of Azure VM extension data source configurations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<ExtensionDataSource>,
}
impl DataSourcesSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Specification of destinations that can be used in data flows."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DestinationsSpec {
    #[doc = "List of Log Analytics destinations."]
    #[serde(rename = "logAnalytics", default, skip_serializing_if = "Vec::is_empty")]
    pub log_analytics: Vec<LogAnalyticsDestination>,
    #[doc = "Azure Monitor Metrics destination."]
    #[serde(rename = "azureMonitorMetrics", default, skip_serializing_if = "Option::is_none")]
    pub azure_monitor_metrics: Option<serde_json::Value>,
}
impl DestinationsSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource management error additional info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorAdditionalInfo {
    #[doc = "The additional info type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The additional info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<serde_json::Value>,
}
impl ErrorAdditionalInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The error detail."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    #[doc = "The error code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The error target."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[doc = "The error details."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
    #[doc = "The error additional info."]
    #[serde(rename = "additionalInfo", default, skip_serializing_if = "Vec::is_empty")]
    pub additional_info: Vec<ErrorAdditionalInfo>,
}
impl ErrorDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Common error response for all Azure Resource Manager APIs to return error details for failed operations. (This also follows the OData error response format.)."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseCommonV2 {
    #[doc = "The error detail."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}
impl azure_core::Continuable for ErrorResponseCommonV2 {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl ErrorResponseCommonV2 {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of which data will be collected from a separate VM extension that integrates with the Azure Monitor Agent.\r\nCollected from either Windows and Linux machines, depending on which extension is defined."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExtensionDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<String>,
    #[doc = "The name of the VM extension."]
    #[serde(rename = "extensionName")]
    pub extension_name: String,
    #[doc = "The extension settings. The format is specific for particular extension."]
    #[serde(rename = "extensionSettings", default, skip_serializing_if = "Option::is_none")]
    pub extension_settings: Option<serde_json::Value>,
    #[doc = "The list of data sources this extension needs data from."]
    #[serde(rename = "inputDataSources", default, skip_serializing_if = "Vec::is_empty")]
    pub input_data_sources: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl ExtensionDataSource {
    pub fn new(extension_name: String) -> Self {
        Self {
            streams: Vec::new(),
            extension_name,
            extension_settings: None,
            input_data_sources: Vec::new(),
            name: None,
        }
    }
}
#[doc = "Log Analytics destination."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogAnalyticsDestination {
    #[doc = "The resource ID of the Log Analytics workspace."]
    #[serde(rename = "workspaceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_resource_id: Option<String>,
    #[doc = "The Customer ID of the Log Analytics workspace."]
    #[serde(rename = "workspaceId", default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[doc = "A friendly name for the destination. \r\nThis name should be unique across all destinations (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl LogAnalyticsDestination {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the endpoint used for ingesting logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LogsIngestionEndpointSpec {
    #[doc = "The endpoint. This property is READ-ONLY."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl LogsIngestionEndpointSpec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of the network rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct NetworkRuleSet {
    #[doc = "The configuration to set whether network access from public internet to the endpoints are allowed."]
    #[serde(rename = "publicNetworkAccess", default, skip_serializing_if = "Option::is_none")]
    pub public_network_access: Option<network_rule_set::PublicNetworkAccess>,
}
impl NetworkRuleSet {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod network_rule_set {
    use super::*;
    #[doc = "The configuration to set whether network access from public internet to the endpoints are allowed."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PublicNetworkAccess {
        Enabled,
        Disabled,
    }
}
#[doc = "Definition of which performance counters will be collected and how they will be collected by this data collection rule.\r\nCollected from both Windows and Linux machines where the counter is present."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PerfCounterDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<String>,
    #[doc = "The number of seconds between consecutive counter measurements (samples)."]
    #[serde(rename = "samplingFrequencyInSeconds", default, skip_serializing_if = "Option::is_none")]
    pub sampling_frequency_in_seconds: Option<i32>,
    #[doc = "A list of specifier names of the performance counters you want to collect.\r\nUse a wildcard (*) to collect a counter for all instances.\r\nTo get a list of performance counters on Windows, run the command 'typeperf'."]
    #[serde(rename = "counterSpecifiers", default, skip_serializing_if = "Vec::is_empty")]
    pub counter_specifiers: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PerfCounterDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of ARM tracked top level resource properties for update operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceForUpdate {
    #[doc = "Resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl ResourceForUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of which syslog data will be collected and how it will be collected.\r\nOnly collected from Linux machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SyslogDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<String>,
    #[doc = "The list of facility names."]
    #[serde(rename = "facilityNames", default, skip_serializing_if = "Vec::is_empty")]
    pub facility_names: Vec<String>,
    #[doc = "The log levels to collect."]
    #[serde(rename = "logLevels", default, skip_serializing_if = "Vec::is_empty")]
    pub log_levels: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SyslogDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of which Windows Event Log events will be collected and how they will be collected.\r\nOnly collected from Windows machines."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct WindowsEventLogDataSource {
    #[doc = "List of streams that this data source will be sent to.\r\nA stream indicates what schema will be used for this data and usually what table in Log Analytics the data will be sent to."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<String>,
    #[doc = "A list of Windows Event Log queries in XPATH format."]
    #[serde(rename = "xPathQueries", default, skip_serializing_if = "Vec::is_empty")]
    pub x_path_queries: Vec<String>,
    #[doc = "A friendly name for the data source. \r\nThis name should be unique across all data sources (regardless of type) within the data collection rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl WindowsEventLogDataSource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Metadata pertaining to creation and last modification of the resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SystemData {
    #[doc = "The identity that created the resource."]
    #[serde(rename = "createdBy", default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The type of identity that created the resource."]
    #[serde(rename = "createdByType", default, skip_serializing_if = "Option::is_none")]
    pub created_by_type: Option<system_data::CreatedByType>,
    #[doc = "The timestamp of resource creation (UTC)."]
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The identity that last modified the resource."]
    #[serde(rename = "lastModifiedBy", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc = "The type of identity that last modified the resource."]
    #[serde(rename = "lastModifiedByType", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by_type: Option<system_data::LastModifiedByType>,
    #[doc = "The timestamp of resource last modification (UTC)"]
    #[serde(rename = "lastModifiedAt", default, skip_serializing_if = "Option::is_none")]
    pub last_modified_at: Option<String>,
}
impl SystemData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod system_data {
    use super::*;
    #[doc = "The type of identity that created the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreatedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
    #[doc = "The type of identity that last modified the resource."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum LastModifiedByType {
        User,
        Application,
        ManagedIdentity,
        Key,
    }
}
