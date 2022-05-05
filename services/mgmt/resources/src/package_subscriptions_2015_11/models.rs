#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "List of availability zones shared by the subscriptions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AvailabilityZonePeers {
    #[doc = "The availabilityZone."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc = "Details of shared availability zone."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peers: Vec<Peers>,
}
impl AvailabilityZonePeers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Resource Name valid if not a reserved word, does not contain a reserved word and does not start with a reserved word"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckResourceNameResult {
    #[doc = "Name of Resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of Resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Is the resource name Allowed or Reserved"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<check_resource_name_result::Status>,
}
impl CheckResourceNameResult {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod check_resource_name_result {
    use super::*;
    #[doc = "Is the resource name Allowed or Reserved"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        Allowed,
        Reserved,
    }
}
#[doc = "Check zone peers request parameters."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckZonePeersRequest {
    #[doc = "The Microsoft location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The peer Microsoft Azure subscription ID."]
    #[serde(rename = "subscriptionIds", default, skip_serializing_if = "Vec::is_empty")]
    pub subscription_ids: Vec<String>,
}
impl CheckZonePeersRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the Check zone peers operation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct CheckZonePeersResult {
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "the location of the subscription."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The Availability Zones shared by the subscriptions."]
    #[serde(rename = "availabilityZonePeers", default, skip_serializing_if = "Vec::is_empty")]
    pub availability_zone_peers: Vec<AvailabilityZonePeers>,
}
impl CheckZonePeersResult {
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
#[doc = "Error description and code explaining why resource name is invalid."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorDefinition {
    #[doc = "Description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "Code of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}
impl ErrorDefinition {
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
#[doc = "Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error description and code explaining why resource name is invalid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDefinition>,
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Location {
    #[doc = "Gets or sets the ID of the resource (/subscriptions/SubscriptionId)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the location name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the display name of the location"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the latitude of the location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[doc = "Gets or sets the longitude of the location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}
impl Location {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Location list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LocationListResult {
    #[doc = "Gets the locations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Location>,
}
impl azure_core::Continuable for LocationListResult {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl LocationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Information about shared availability zone."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Peers {
    #[doc = "The subscription ID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "The availabilityZone."]
    #[serde(rename = "availabilityZone", default, skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
}
impl Peers {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Name and Type of the Resource"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceName {
    #[doc = "Name of the resource"]
    pub name: String,
    #[doc = "The type of the resource"]
    #[serde(rename = "type")]
    pub type_: String,
}
impl ResourceName {
    pub fn new(name: String, type_: String) -> Self {
        Self { name, type_ }
    }
}
#[doc = "Subscription information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Subscription {
    #[doc = "Gets or sets the ID of the resource (/subscriptions/SubscriptionId)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the subscription Id."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "Gets or sets the subscription display name"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Gets or sets the subscription state"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "Subscription policies."]
    #[serde(rename = "subscriptionPolicies", default, skip_serializing_if = "Option::is_none")]
    pub subscription_policies: Option<SubscriptionPolicies>,
}
impl Subscription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Subscription list operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubscriptionListResult {
    #[doc = "Gets or sets subscriptions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Subscription>,
    #[doc = "Gets or sets the URL to get the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for SubscriptionListResult {
    fn continuation(&self) -> Option<String> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl SubscriptionListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
#[doc = "Subscription policies."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SubscriptionPolicies {
    #[doc = "Gets or sets the subscription location placement Id."]
    #[serde(rename = "locationPlacementId", default, skip_serializing_if = "Option::is_none")]
    pub location_placement_id: Option<String>,
    #[doc = "Gets or sets the subscription quota Id."]
    #[serde(rename = "quotaId", default, skip_serializing_if = "Option::is_none")]
    pub quota_id: Option<String>,
}
impl SubscriptionPolicies {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant Id information"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct TenantIdDescription {
    #[doc = "Gets or sets Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets tenantId"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl TenantIdDescription {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Tenant Ids information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TenantListResult {
    #[doc = "Gets or sets tenant Ids."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<TenantIdDescription>,
    #[doc = "Gets or sets the URL to get the next set of results."]
    #[serde(rename = "nextLink")]
    pub next_link: String,
}
impl azure_core::Continuable for TenantListResult {
    fn continuation(&self) -> Option<String> {
        if self.next_link.is_empty() {
            None
        } else {
            Some(self.next_link.clone())
        }
    }
}
impl TenantListResult {
    pub fn new(next_link: String) -> Self {
        Self {
            value: Vec::new(),
            next_link,
        }
    }
}
