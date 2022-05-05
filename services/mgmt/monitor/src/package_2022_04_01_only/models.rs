#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The action detail"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionDetail {
    #[doc = "The mechanism type"]
    #[serde(rename = "MechanismType", default, skip_serializing_if = "Option::is_none")]
    pub mechanism_type: Option<String>,
    #[doc = "The name of the action"]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The status of the action"]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The substatus of the action"]
    #[serde(rename = "SubState", default, skip_serializing_if = "Option::is_none")]
    pub sub_state: Option<String>,
    #[doc = "The send time"]
    #[serde(rename = "SendTime", default, skip_serializing_if = "Option::is_none")]
    pub send_time: Option<String>,
    #[doc = "The detail of the friendly error message"]
    #[serde(rename = "Detail", default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
impl ActionDetail {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure action group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroup {
    #[doc = "The short name of the action group. This will be used in SMS messages."]
    #[serde(rename = "groupShortName")]
    pub group_short_name: String,
    #[doc = "Indicates whether this action group is enabled. If an action group is not enabled, then none of its receivers will receive communications."]
    pub enabled: bool,
    #[doc = "The list of email receivers that are part of this action group."]
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[doc = "The list of SMS receivers that are part of this action group."]
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[doc = "The list of webhook receivers that are part of this action group."]
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[doc = "The list of ITSM receivers that are part of this action group."]
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[doc = "The list of AzureAppPush receivers that are part of this action group."]
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[doc = "The list of AutomationRunbook receivers that are part of this action group."]
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[doc = "The list of voice receivers that are part of this action group."]
    #[serde(rename = "voiceReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[doc = "The list of logic app receivers that are part of this action group."]
    #[serde(rename = "logicAppReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[doc = "The list of azure function receivers that are part of this action group."]
    #[serde(rename = "azureFunctionReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[doc = "The list of ARM role receivers that are part of this action group. Roles are Azure RBAC roles and only built-in roles are supported."]
    #[serde(rename = "armRoleReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
    #[doc = "The list of event hub receivers that are part of this action group."]
    #[serde(rename = "eventHubReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub event_hub_receivers: Vec<EventHubReceiver>,
}
impl ActionGroup {
    pub fn new(group_short_name: String, enabled: bool) -> Self {
        Self {
            group_short_name,
            enabled,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
            voice_receivers: Vec::new(),
            logic_app_receivers: Vec::new(),
            azure_function_receivers: Vec::new(),
            arm_role_receivers: Vec::new(),
            event_hub_receivers: Vec::new(),
        }
    }
}
#[doc = "A list of action groups."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupList {
    #[doc = "The list of action groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ActionGroupResource>,
    #[doc = "Provides the link to retrieve the next set of elements."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ActionGroupList {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl ActionGroupList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An Azure action group for patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatch {
    #[doc = "Indicates whether this action group is enabled. If an action group is not enabled, then none of its actions will be activated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl ActionGroupPatch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action group object for the body of patch operations."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ActionGroupPatchBody {
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "An Azure action group for patch operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroupPatch>,
}
impl ActionGroupPatchBody {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An action group resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionGroupResource {
    #[serde(flatten)]
    pub azure_resource: AzureResource,
    #[doc = "An Azure action group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ActionGroup>,
}
impl ActionGroupResource {
    pub fn new(azure_resource: AzureResource) -> Self {
        Self {
            azure_resource,
            properties: None,
        }
    }
}
#[doc = "An arm role receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmRoleReceiver {
    #[doc = "The name of the arm role receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The arm role id."]
    #[serde(rename = "roleId")]
    pub role_id: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl ArmRoleReceiver {
    pub fn new(name: String, role_id: String) -> Self {
        Self {
            name,
            role_id,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "The Azure Automation Runbook notification receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutomationRunbookReceiver {
    #[doc = "The Azure automation account Id which holds this runbook and authenticate to Azure resource."]
    #[serde(rename = "automationAccountId")]
    pub automation_account_id: String,
    #[doc = "The name for this runbook."]
    #[serde(rename = "runbookName")]
    pub runbook_name: String,
    #[doc = "The resource id for webhook linked to this runbook."]
    #[serde(rename = "webhookResourceId")]
    pub webhook_resource_id: String,
    #[doc = "Indicates whether this instance is global runbook."]
    #[serde(rename = "isGlobalRunbook")]
    pub is_global_runbook: bool,
    #[doc = "Indicates name of the webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The URI where webhooks should be sent."]
    #[serde(rename = "serviceUri", default, skip_serializing_if = "Option::is_none")]
    pub service_uri: Option<String>,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl AutomationRunbookReceiver {
    pub fn new(automation_account_id: String, runbook_name: String, webhook_resource_id: String, is_global_runbook: bool) -> Self {
        Self {
            automation_account_id,
            runbook_name,
            webhook_resource_id,
            is_global_runbook,
            name: None,
            service_uri: None,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "The Azure mobile App push notification receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureAppPushReceiver {
    #[doc = "The name of the Azure mobile app push receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The email address registered for the Azure mobile app."]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
}
impl AzureAppPushReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self { name, email_address }
    }
}
#[doc = "An azure function receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFunctionReceiver {
    #[doc = "The name of the azure function receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The azure resource id of the function app."]
    #[serde(rename = "functionAppResourceId")]
    pub function_app_resource_id: String,
    #[doc = "The function name in the function app."]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[doc = "The http trigger url where http request sent to."]
    #[serde(rename = "httpTriggerUrl")]
    pub http_trigger_url: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl AzureFunctionReceiver {
    pub fn new(name: String, function_app_resource_id: String, function_name: String, http_trigger_url: String) -> Self {
        Self {
            name,
            function_app_resource_id,
            function_name,
            http_trigger_url,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "An azure resource object"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureResource {
    #[doc = "Azure resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Azure resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Azure resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Azure resource kind"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[doc = "Azure resource identity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl AzureResource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            kind: None,
            identity: None,
            location,
            tags: None,
        }
    }
}
#[doc = "The context info"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Context {
    #[doc = "The source of the notification request"]
    #[serde(rename = "NotificationSource", default, skip_serializing_if = "Option::is_none")]
    pub notification_source: Option<String>,
    #[doc = "The context id type"]
    #[serde(rename = "ContextType", default, skip_serializing_if = "Option::is_none")]
    pub context_type: Option<String>,
}
impl Context {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An email receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailReceiver {
    #[doc = "The name of the email receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The email address of this receiver."]
    #[serde(rename = "emailAddress")]
    pub email_address: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl EmailReceiver {
    pub fn new(name: String, email_address: String) -> Self {
        Self {
            name,
            email_address,
            use_common_alert_schema: None,
            status: None,
        }
    }
}
#[doc = "Describes a receiver that should be resubscribed."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnableRequest {
    #[doc = "The name of the receiver to resubscribe."]
    #[serde(rename = "receiverName")]
    pub receiver_name: String,
}
impl EnableRequest {
    pub fn new(receiver_name: String) -> Self {
        Self { receiver_name }
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
#[doc = "An Event hub receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventHubReceiver {
    #[doc = "The name of the Event hub receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The Event Hub namespace"]
    #[serde(rename = "eventHubNameSpace")]
    pub event_hub_name_space: String,
    #[doc = "The name of the specific Event Hub queue"]
    #[serde(rename = "eventHubName")]
    pub event_hub_name: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "The tenant Id for the subscription containing this event hub"]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[doc = "The Id for the subscription containing this event hub"]
    #[serde(rename = "subscriptionId")]
    pub subscription_id: String,
}
impl EventHubReceiver {
    pub fn new(name: String, event_hub_name_space: String, event_hub_name: String, subscription_id: String) -> Self {
        Self {
            name,
            event_hub_name_space,
            event_hub_name,
            use_common_alert_schema: None,
            tenant_id: None,
            subscription_id,
        }
    }
}
#[doc = "An Itsm receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItsmReceiver {
    #[doc = "The name of the Itsm receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "OMS LA instance identifier."]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "Unique identification of ITSM connection among multiple defined in above workspace."]
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[doc = "JSON blob for the configurations of the ITSM action. CreateMultipleWorkItems option will be part of this blob as well."]
    #[serde(rename = "ticketConfiguration")]
    pub ticket_configuration: String,
    #[doc = "Region in which workspace resides. Supported values:'centralindia','japaneast','southeastasia','australiasoutheast','uksouth','westcentralus','canadacentral','eastus','westeurope'"]
    pub region: String,
}
impl ItsmReceiver {
    pub fn new(name: String, workspace_id: String, connection_id: String, ticket_configuration: String, region: String) -> Self {
        Self {
            name,
            workspace_id,
            connection_id,
            ticket_configuration,
            region,
        }
    }
}
#[doc = "A logic app receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogicAppReceiver {
    #[doc = "The name of the logic app receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The azure resource id of the logic app receiver."]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[doc = "The callback url where http request sent to."]
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
}
impl LogicAppReceiver {
    pub fn new(name: String, resource_id: String, callback_url: String) -> Self {
        Self {
            name,
            resource_id,
            callback_url,
            use_common_alert_schema: None,
        }
    }
}
#[doc = "The request body which contain contact detail metadata"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationRequestBody {
    #[doc = "The name of the supported alert type."]
    #[serde(rename = "alertType")]
    pub alert_type: String,
    #[doc = "The list of email receivers that are part of this action group."]
    #[serde(rename = "emailReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub email_receivers: Vec<EmailReceiver>,
    #[doc = "The list of SMS receivers that are part of this action group."]
    #[serde(rename = "smsReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub sms_receivers: Vec<SmsReceiver>,
    #[doc = "The list of webhook receivers that are part of this action group."]
    #[serde(rename = "webhookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub webhook_receivers: Vec<WebhookReceiver>,
    #[doc = "The list of ITSM receivers that are part of this action group."]
    #[serde(rename = "itsmReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub itsm_receivers: Vec<ItsmReceiver>,
    #[doc = "The list of AzureAppPush receivers that are part of this action group."]
    #[serde(rename = "azureAppPushReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_app_push_receivers: Vec<AzureAppPushReceiver>,
    #[doc = "The list of AutomationRunbook receivers that are part of this action group."]
    #[serde(rename = "automationRunbookReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub automation_runbook_receivers: Vec<AutomationRunbookReceiver>,
    #[doc = "The list of voice receivers that are part of this action group."]
    #[serde(rename = "voiceReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub voice_receivers: Vec<VoiceReceiver>,
    #[doc = "The list of logic app receivers that are part of this action group."]
    #[serde(rename = "logicAppReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub logic_app_receivers: Vec<LogicAppReceiver>,
    #[doc = "The list of azure function receivers that are part of this action group."]
    #[serde(rename = "azureFunctionReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub azure_function_receivers: Vec<AzureFunctionReceiver>,
    #[doc = "The list of ARM role receivers that are part of this action group. Roles are Azure RBAC roles and only built-in roles are supported."]
    #[serde(rename = "armRoleReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub arm_role_receivers: Vec<ArmRoleReceiver>,
    #[doc = "The list of event hub receivers that are part of this action group."]
    #[serde(rename = "eventHubReceivers", default, skip_serializing_if = "Vec::is_empty")]
    pub event_hub_receivers: Vec<EventHubReceiver>,
}
impl NotificationRequestBody {
    pub fn new(alert_type: String) -> Self {
        Self {
            alert_type,
            email_receivers: Vec::new(),
            sms_receivers: Vec::new(),
            webhook_receivers: Vec::new(),
            itsm_receivers: Vec::new(),
            azure_app_push_receivers: Vec::new(),
            automation_runbook_receivers: Vec::new(),
            voice_receivers: Vec::new(),
            logic_app_receivers: Vec::new(),
            azure_function_receivers: Vec::new(),
            arm_role_receivers: Vec::new(),
            event_hub_receivers: Vec::new(),
        }
    }
}
#[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReceiverStatus {
    NotSpecified,
    Enabled,
    Disabled,
}
#[doc = "An SMS receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmsReceiver {
    #[doc = "The name of the SMS receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The country code of the SMS receiver."]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[doc = "The phone number of the SMS receiver."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[doc = "Indicates the status of the receiver. Receivers that are not Enabled will not receive any communications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ReceiverStatus>,
}
impl SmsReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
            status: None,
        }
    }
}
#[doc = "The details of the test notification results."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TestNotificationDetailsResponse {
    #[doc = "The context info"]
    #[serde(rename = "Context", default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
    #[doc = "The overall state"]
    #[serde(rename = "State")]
    pub state: String,
    #[doc = "The completed time"]
    #[serde(rename = "CompletedTime", default, skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<String>,
    #[doc = "The created time"]
    #[serde(rename = "CreatedTime", default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[doc = "The list of action detail"]
    #[serde(rename = "ActionDetails", default, skip_serializing_if = "Vec::is_empty")]
    pub action_details: Vec<ActionDetail>,
}
impl TestNotificationDetailsResponse {
    pub fn new(state: String) -> Self {
        Self {
            context: None,
            state,
            completed_time: None,
            created_time: None,
            action_details: Vec::new(),
        }
    }
}
#[doc = "A voice receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VoiceReceiver {
    #[doc = "The name of the voice receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The country code of the voice receiver."]
    #[serde(rename = "countryCode")]
    pub country_code: String,
    #[doc = "The phone number of the voice receiver."]
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
}
impl VoiceReceiver {
    pub fn new(name: String, country_code: String, phone_number: String) -> Self {
        Self {
            name,
            country_code,
            phone_number,
        }
    }
}
#[doc = "A webhook receiver."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookReceiver {
    #[doc = "The name of the webhook receiver. Names must be unique across all receivers within an action group."]
    pub name: String,
    #[doc = "The URI where webhooks should be sent."]
    #[serde(rename = "serviceUri")]
    pub service_uri: String,
    #[doc = "Indicates whether to use common alert schema."]
    #[serde(rename = "useCommonAlertSchema", default, skip_serializing_if = "Option::is_none")]
    pub use_common_alert_schema: Option<bool>,
    #[doc = "Indicates whether or not use AAD authentication."]
    #[serde(rename = "useAadAuth", default, skip_serializing_if = "Option::is_none")]
    pub use_aad_auth: Option<bool>,
    #[doc = "Indicates the webhook app object Id for aad auth."]
    #[serde(rename = "objectId", default, skip_serializing_if = "Option::is_none")]
    pub object_id: Option<String>,
    #[doc = "Indicates the identifier uri for aad auth."]
    #[serde(rename = "identifierUri", default, skip_serializing_if = "Option::is_none")]
    pub identifier_uri: Option<String>,
    #[doc = "Indicates the tenant id for aad auth."]
    #[serde(rename = "tenantId", default, skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}
impl WebhookReceiver {
    pub fn new(name: String, service_uri: String) -> Self {
        Self {
            name,
            service_uri,
            use_common_alert_schema: None,
            use_aad_auth: None,
            object_id: None,
            identifier_uri: None,
            tenant_id: None,
        }
    }
}
