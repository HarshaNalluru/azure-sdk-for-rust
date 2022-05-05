#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "A pointer to an Azure Action Group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[doc = "The resource ID of the Action Group. This cannot be null or empty."]
    #[serde(rename = "actionGroupId")]
    pub action_group_id: String,
    #[doc = "the dictionary of custom properties to include with the post operation. These data are appended to the webhook payload."]
    #[serde(rename = "webhookProperties", default, skip_serializing_if = "Option::is_none")]
    pub webhook_properties: Option<serde_json::Value>,
}
impl ActionGroup {
    pub fn new(action_group_id: String) -> Self {
        Self {
            action_group_id,
            webhook_properties: None,
        }
    }
}
#[doc = "A list of Activity Log Alert rule actions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionList {
    #[doc = "The list of the Action Groups."]
    #[serde(rename = "actionGroups", default, skip_serializing_if = "Vec::is_empty")]
    pub action_groups: Vec<ActionGroup>,
}
impl ActionList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActivityLogAlertResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[doc = "An Azure Activity Log Alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRuleProperties>,
}
impl ActivityLogAlertResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule condition that is met when all its member conditions are met."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleAllOfCondition {
    #[doc = "The list of Activity Log Alert rule conditions."]
    #[serde(rename = "allOf")]
    pub all_of: Vec<AlertRuleAnyOfOrLeafCondition>,
}
impl AlertRuleAllOfCondition {
    pub fn new(all_of: Vec<AlertRuleAnyOfOrLeafCondition>) -> Self {
        Self { all_of }
    }
}
#[doc = "An Activity Log Alert rule condition that is met when all its member conditions are met.\nEach condition can be of one of the following types:\n__Important__: Each type has its unique subset of properties. Properties from different types CANNOT exist in one condition.\n   * __Leaf Condition -__ must contain 'field' and either 'equals' or 'containsAny'.\n  _Please note, 'anyOf' should __not__ be set in a Leaf Condition._\n  * __AnyOf Condition -__ must contain __only__ 'anyOf' (which is an array of Leaf Conditions).\n  _Please note, 'field', 'equals' and 'containsAny' should __not__ be set in an AnyOf Condition._\n"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleAnyOfOrLeafCondition {
    #[serde(flatten)]
    pub alert_rule_leaf_condition: AlertRuleLeafCondition,
    #[doc = "An Activity Log Alert rule condition that is met when at least one of its member leaf conditions are met."]
    #[serde(rename = "anyOf", default, skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<AlertRuleLeafCondition>,
}
impl AlertRuleAnyOfOrLeafCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule condition that is met by comparing the field and value of an Activity Log event.\nThis condition must contain 'field' and either 'equals' or 'containsAny'."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleLeafCondition {
    #[doc = "The name of the Activity Log event's field that this condition will examine.\nThe possible values for this field are (case-insensitive): 'resourceId', 'category', 'caller', 'level', 'operationName', 'resourceGroup', 'resourceProvider', 'status', 'subStatus', 'resourceType', or anything beginning with 'properties'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[doc = "The value of the event's field will be compared to this value (case-insensitive) to determine if the condition is met."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub equals: Option<String>,
    #[doc = "The value of the event's field will be compared to the values in this array (case-insensitive) to determine if the condition is met."]
    #[serde(rename = "containsAny", default, skip_serializing_if = "Vec::is_empty")]
    pub contains_any: Vec<String>,
}
impl AlertRuleLeafCondition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A list of Activity Log Alert rules."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRuleList {
    #[doc = "The list of Activity Log Alert rules."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActivityLogAlertResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for AlertRuleList {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl AlertRuleList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule object for the body of patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulePatchObject {
    #[doc = "The resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An Activity Log Alert rule properties for patch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AlertRulePatchProperties>,
}
impl AlertRulePatchObject {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Activity Log Alert rule properties for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AlertRulePatchProperties {
    #[doc = "Indicates whether this Activity Log Alert rule is enabled. If an Activity Log Alert rule is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl AlertRulePatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure Activity Log Alert rule."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AlertRuleProperties {
    #[doc = "A list of resource IDs that will be used as prefixes. The alert will only apply to Activity Log events with resource IDs that fall under one of these prefixes. This list must include at least one item."]
    pub scopes: Vec<String>,
    #[doc = "An Activity Log Alert rule condition that is met when all its member conditions are met."]
    pub condition: AlertRuleAllOfCondition,
    #[doc = "A list of Activity Log Alert rule actions."]
    pub actions: ActionList,
    #[doc = "Indicates whether this Activity Log Alert rule is enabled. If an Activity Log Alert rule is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "A description of this Activity Log Alert rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl AlertRuleProperties {
    pub fn new(scopes: Vec<String>, condition: AlertRuleAllOfCondition, actions: ActionList) -> Self {
        Self {
            scopes,
            condition,
            actions,
            enabled: None,
            description: None,
        }
    }
}
#[doc = "An Azure resource object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AzureResource {
    #[doc = "The resource Id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The type of the resource."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The location of the resource. Since Azure Activity Log Alerts is a global service, the location of the rules should always be 'global'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The tags of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Describes the format of Error response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    #[doc = "Error code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Error message indicating why the operation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for ErrorResponse {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl ErrorResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The Azure event log entries are of type EventData"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EventData {
    #[doc = "the authorization used by the user who has performed the operation that led to this event. This captures the RBAC properties of the event. These usually include the 'action', 'role' and the 'scope'"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<SenderAuthorization>,
    #[doc = "key value pairs to identify ARM permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<serde_json::Value>,
    #[doc = "the email address of the user who has performed the operation, the UPN claim or SPN claim based on availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,
    #[doc = "the description of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "the Id of this event as required by ARM for RBAC. It contains the EventDataID and a timestamp information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "the event data Id. This is a unique identifier for an event."]
    #[serde(rename = "eventDataId", default, skip_serializing_if = "Option::is_none")]
    pub event_data_id: Option<String>,
    #[doc = "the correlation Id, usually a GUID in the string format. The correlation Id is shared among the events that belong to the same uber operation."]
    #[serde(rename = "correlationId", default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "eventName", default, skip_serializing_if = "Option::is_none")]
    pub event_name: Option<LocalizableString>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<LocalizableString>,
    #[doc = "The Http request info."]
    #[serde(rename = "httpRequest", default, skip_serializing_if = "Option::is_none")]
    pub http_request: Option<HttpRequestInfo>,
    #[doc = "the event level"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<event_data::Level>,
    #[doc = "the resource group name of the impacted resource."]
    #[serde(rename = "resourceGroupName", default, skip_serializing_if = "Option::is_none")]
    pub resource_group_name: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "resourceProviderName", default, skip_serializing_if = "Option::is_none")]
    pub resource_provider_name: Option<LocalizableString>,
    #[doc = "the resource uri that uniquely identifies the resource that caused this event."]
    #[serde(rename = "resourceId", default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<LocalizableString>,
    #[doc = "It is usually a GUID shared among the events corresponding to single operation. This value should not be confused with EventName."]
    #[serde(rename = "operationId", default, skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[doc = "The localizable string class."]
    #[serde(rename = "operationName", default, skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<LocalizableString>,
    #[doc = "the set of <Key, Value> pairs (usually a Dictionary<String, String>) that includes details about the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
    #[doc = "The localizable string class."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LocalizableString>,
    #[doc = "The localizable string class."]
    #[serde(rename = "subStatus", default, skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<LocalizableString>,
    #[doc = "the timestamp of when the event was generated by the Azure service processing the request corresponding the event. It in ISO 8601 format."]
    #[serde(rename = "eventTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub event_timestamp: Option<String>,
    #[doc = "the timestamp of when the event became available for querying via this API. It is in ISO 8601 format. This value should not be confused eventTimestamp. As there might be a delay between the occurrence time of the event, and the time that the event is submitted to the Azure logging infrastructure."]
    #[serde(rename = "submissionTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub submission_timestamp: Option<String>,
    #[doc = "the Azure subscription Id usually a GUID."]
    #[serde(rename = "subscriptionId", default, skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[doc = "the Azure tenant Id"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl EventData {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod event_data {
    use super::*;
    #[doc = "the event level"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Level {
        Critical,
        Error,
        Warning,
        Informational,
        Verbose,
    }
}
#[doc = "Represents collection of events."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventDataCollection {
    #[doc = "this list that includes the Azure audit logs."]
    pub value: Vec<EventData>,
    #[doc = "Provides the link to retrieve the next set of events."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for EventDataCollection {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl EventDataCollection {
    pub fn new(value: Vec<EventData>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The Http request info."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct HttpRequestInfo {
    #[doc = "the client request id."]
    #[serde(rename = "clientRequestId", default, skip_serializing_if = "Option::is_none")]
    pub client_request_id: Option<String>,
    #[doc = "the client Ip Address"]
    #[serde(rename = "clientIpAddress", default, skip_serializing_if = "Option::is_none")]
    pub client_ip_address: Option<String>,
    #[doc = "the Http request method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[doc = "the Uri."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl HttpRequestInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The localizable string class."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizableString {
    #[doc = "the invariant value."]
    pub value: String,
    #[doc = "the locale specific value."]
    #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
impl LocalizableString {
    pub fn new(value: String) -> Self {
        Self {
            value,
            localized_value: None,
        }
    }
}
#[doc = "the authorization used by the user who has performed the operation that led to this event. This captures the RBAC properties of the event. These usually include the 'action', 'role' and the 'scope'"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SenderAuthorization {
    #[doc = "the permissible actions. For instance: microsoft.support/supporttickets/write"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "the role of the user. For instance: Subscription Admin"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "the scope."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
impl SenderAuthorization {
    pub fn new() -> Self {
        Self::default()
    }
}
