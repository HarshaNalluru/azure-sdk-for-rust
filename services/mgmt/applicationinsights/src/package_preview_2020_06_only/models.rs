#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ErrorResponseLinkedStorage {
    #[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<error_response_linked_storage::Error>,
}
impl ErrorResponseLinkedStorage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod error_response_linked_storage {
    use super::*;
    #[doc = "Error response indicates Insights service is not able to process the incoming request. The reason is provided in the error message."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Error {
        #[doc = "Error code."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub code: Option<String>,
        #[doc = "Error message indicating why the operation failed."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub message: Option<String>,
    }
    impl Error {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response to a live token query."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LiveTokenResponse {
    #[doc = "JWT token for accessing live metrics stream data."]
    #[serde(rename = "liveToken", default, skip_serializing_if = "Option::is_none")]
    pub live_token: Option<String>,
}
impl LiveTokenResponse {
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
#[doc = "Represents an operation returned by the GetOperations request"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationLive {
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
impl OperationLive {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Result of the List Operations operation"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationsListResult {
    #[doc = "A collection of operations"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationLive>,
    #[doc = "URL to get the next set of operation list results if there are any."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationsListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl OperationsListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
