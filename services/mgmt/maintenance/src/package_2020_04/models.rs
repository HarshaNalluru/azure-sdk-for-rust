#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Apply Update request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyUpdate {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Properties for apply update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplyUpdateProperties>,
}
impl ApplyUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for apply update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ApplyUpdateProperties {
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<apply_update_properties::Status>,
    #[doc = "The resourceId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "Last Update time"]
    #[serde(rename = "lastUpdateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
}
impl ApplyUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod apply_update_properties {
    use super::*;
    #[doc = "The status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
    }
}
#[doc = "Configuration Assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationAssignment {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Properties for configuration assignment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ConfigurationAssignmentProperties>,
}
impl ConfigurationAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for configuration assignment"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigurationAssignmentProperties {
    #[doc = "The maintenance configuration Id"]
    #[serde(rename = "maintenanceConfigurationId", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_configuration_id: Option<String>,
    #[doc = "The unique resourceId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl ConfigurationAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An error response details received from the Azure Maintenance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDetails {
    #[doc = "Service-defined error code. This code serves as a sub-status for the HTTP error code specified in the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Human-readable representation of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetails {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for ConfigurationAssignments list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListConfigurationAssignmentsResult {
    #[doc = "The list of configuration Assignments"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ConfigurationAssignment>,
}
impl azure_core::Continuable for ListConfigurationAssignmentsResult {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl ListConfigurationAssignmentsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for MaintenanceConfigurations list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListMaintenanceConfigurationsResult {
    #[doc = "The list of maintenance Configurations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<MaintenanceConfiguration>,
}
impl azure_core::Continuable for ListMaintenanceConfigurationsResult {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl ListMaintenanceConfigurationsResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Response for Updates list"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ListUpdatesResult {
    #[doc = "The pending updates"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Update>,
}
impl azure_core::Continuable for ListUpdatesResult {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl ListUpdatesResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance configuration record type"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfiguration {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Gets or sets location of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Gets or sets tags of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "Properties for maintenance configuration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MaintenanceConfigurationProperties>,
}
impl MaintenanceConfiguration {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Properties for maintenance configuration"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceConfigurationProperties {
    #[doc = "Gets or sets namespace of the resource e.g. Microsoft.Maintenance or Microsoft.Sql"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[doc = "Gets or sets extensionProperties of the maintenanceConfiguration. This is for future use only and would be a set of key value pairs for additional information e.g. whether to follow SDP etc."]
    #[serde(rename = "extensionProperties", default, skip_serializing_if = "Option::is_none")]
    pub extension_properties: Option<serde_json::Value>,
    #[doc = "Gets or sets maintenanceScope of the configuration. It represent the impact area of the maintenance"]
    #[serde(rename = "maintenanceScope", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<maintenance_configuration_properties::MaintenanceScope>,
}
impl MaintenanceConfigurationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod maintenance_configuration_properties {
    use super::*;
    #[doc = "Gets or sets maintenanceScope of the configuration. It represent the impact area of the maintenance"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaintenanceScope {
        All,
        Host,
        Resource,
        InResource,
    }
}
#[doc = "An error response received from the Azure Maintenance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MaintenanceError {
    #[doc = "An error response details received from the Azure Maintenance service."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
}
impl azure_core::Continuable for MaintenanceError {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl MaintenanceError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents an operation returned by the GetOperations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Operation {
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Information about an operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationInfo>,
    #[doc = "Origin of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "Properties of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}
impl Operation {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about an operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationInfo {
    #[doc = "Name of the provider"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[doc = "Name of the resource type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[doc = "Name of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[doc = "Description of the operation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Operations operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "A collection of operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
}
impl azure_core::Continuable for OperationsListResult {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Definition of a Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "Fully qualified identifier of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Maintenance update on a resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Update {
    #[doc = "The impact area"]
    #[serde(rename = "maintenanceScope", default, skip_serializing_if = "Option::is_none")]
    pub maintenance_scope: Option<update::MaintenanceScope>,
    #[doc = "The impact type"]
    #[serde(rename = "impactType", default, skip_serializing_if = "Option::is_none")]
    pub impact_type: Option<update::ImpactType>,
    #[doc = "The status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<update::Status>,
    #[doc = "Duration of impact in seconds"]
    #[serde(rename = "impactDurationInSec", default, skip_serializing_if = "Option::is_none")]
    pub impact_duration_in_sec: Option<i32>,
    #[doc = "Time when Azure will start force updates if not self-updated by customer before this time"]
    #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[doc = "Properties for update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<UpdateProperties>,
}
impl Update {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod update {
    use super::*;
    #[doc = "The impact area"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum MaintenanceScope {
        All,
        Host,
        Resource,
        InResource,
    }
    #[doc = "The impact type"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ImpactType {
        None,
        Freeze,
        Restart,
        Redeploy,
    }
    #[doc = "The status"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Pending,
        InProgress,
        Completed,
        RetryNow,
        RetryLater,
    }
}
#[doc = "Properties for update"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateProperties {
    #[doc = "The resourceId"]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}
impl UpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
