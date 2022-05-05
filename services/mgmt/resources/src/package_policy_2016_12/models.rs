#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The policy assignment."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignment {
    #[doc = "The policy assignment properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyAssignmentProperties>,
    #[doc = "The ID of the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The type of the policy assignment."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The name of the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PolicyAssignment {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy assignments."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentListResult {
    #[doc = "An array of policy assignments."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyAssignment>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyAssignmentListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl PolicyAssignmentListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy assignment properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyAssignmentProperties {
    #[doc = "The display name of the policy assignment."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the policy definition."]
    #[serde(rename = "policyDefinitionId", default, skip_serializing_if = "Option::is_none")]
    pub policy_definition_id: Option<String>,
    #[doc = "The scope for the policy assignment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "Required if a parameter is used in policy rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[doc = "This message will be part of response in case of policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl PolicyAssignmentProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinition {
    #[doc = "The policy definition properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PolicyDefinitionProperties>,
    #[doc = "The ID of the policy definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the policy definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PolicyDefinition {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "List of policy definitions."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionListResult {
    #[doc = "An array of policy definitions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<PolicyDefinition>,
    #[doc = "The URL to use for getting the next set of results."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for PolicyDefinitionListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl PolicyDefinitionListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The policy definition properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyDefinitionProperties {
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, and Custom."]
    #[serde(rename = "policyType", default, skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<policy_definition_properties::PolicyType>,
    #[doc = "The policy definition mode. Possible values are NotSpecified, Indexed, and All."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<policy_definition_properties::Mode>,
    #[doc = "The display name of the policy definition."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The policy definition description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The policy rule."]
    #[serde(rename = "policyRule", default, skip_serializing_if = "Option::is_none")]
    pub policy_rule: Option<serde_json::Value>,
    #[doc = "The policy definition metadata."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[doc = "Required if a parameter is used in policy rule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}
impl PolicyDefinitionProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod policy_definition_properties {
    use super::*;
    #[doc = "The type of policy definition. Possible values are NotSpecified, BuiltIn, and Custom."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PolicyType {
        NotSpecified,
        BuiltIn,
        Custom,
    }
    #[doc = "The policy definition mode. Possible values are NotSpecified, Indexed, and All."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Mode {
        NotSpecified,
        Indexed,
        All,
    }
}
