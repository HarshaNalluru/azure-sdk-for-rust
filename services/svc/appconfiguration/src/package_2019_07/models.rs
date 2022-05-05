#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "Azure App Configuration error object."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[doc = "The type of the error."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "A brief summary of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The name of the parameter that resulted in the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A detailed description of the error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[doc = "The HTTP status code that the error maps to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}
impl azure_core::Continuable for Error {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl Error {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Key {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Key {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Key>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl KeyListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl KeyValue {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct KeyValueListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeyValue>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for KeyValueListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl KeyValueListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Label {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl Label {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The result of a list request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct LabelListResult {
    #[doc = "The collection value."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Label>,
    #[doc = "The URI that can be used to request the next set of paged results."]
    #[serde(rename = "@nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for LabelListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl LabelListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
