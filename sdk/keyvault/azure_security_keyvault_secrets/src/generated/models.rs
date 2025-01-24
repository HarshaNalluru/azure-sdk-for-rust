// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use crate::models::DeletionRecoveryLevel;
use azure_core::base64;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;

/// The backup secret result, containing the backup blob.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct BackupSecretResult {
    /// The backup blob containing the backed up secret.
    #[serde(
        default,
        deserialize_with = "base64::deserialize_url_safe",
        serialize_with = "base64::serialize_url_safe",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Vec<u8>>,
}

/// A Deleted Secret consisting of its previous id, attributes and its tags, as well as information on when it will be purged.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct DeletedSecretBundle {
    /// The secret management attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,

    /// The content type of the secret.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The time when the secret was deleted, in UTC
    #[serde(
        default,
        rename = "deletedDate",
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub deleted_date: Option<OffsetDateTime>,

    /// The secret id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If this is a secret backing a KV certificate, then this field specifies the corresponding key backing the KV certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,

    /// True if the secret's lifetime is managed by key vault. If this is a secret backing a certificate, then managed will be
    /// true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    /// The url of the recovery object, used to identify and recover the deleted secret.
    #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,

    /// The time when the secret is scheduled to be purged, in UTC
    #[serde(
        default,
        rename = "scheduledPurgeDate",
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub scheduled_purge_date: Option<OffsetDateTime>,

    /// Application specific metadata in the form of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,

    /// The secret value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The deleted secret item containing metadata about the deleted secret.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct DeletedSecretItem {
    /// The secret management attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,

    /// Type of the secret value such as a password.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The time when the secret was deleted, in UTC
    #[serde(
        default,
        rename = "deletedDate",
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub deleted_date: Option<OffsetDateTime>,

    /// Secret identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// True if the secret's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    /// The url of the recovery object, used to identify and recover the deleted secret.
    #[serde(rename = "recoveryId", skip_serializing_if = "Option::is_none")]
    pub recovery_id: Option<String>,

    /// The time when the secret is scheduled to be purged, in UTC
    #[serde(
        default,
        rename = "scheduledPurgeDate",
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub scheduled_purge_date: Option<OffsetDateTime>,

    /// Application specific metadata in the form of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

/// The deleted secret list result
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct DeletedSecretListResult {
    /// The URL to get the next set of deleted secrets.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,

    /// A response message containing a list of deleted secrets in the key vault along with a link to the next page of deleted
    /// secrets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<DeletedSecretItem>>,
}

/// The secret management attributes.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretAttributes {
    /// Creation time in UTC.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub created: Option<OffsetDateTime>,

    /// Determines whether the object is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Expiry date in UTC.
    #[serde(
        default,
        rename = "exp",
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub expires: Option<OffsetDateTime>,

    /// Not before date in UTC.
    #[serde(
        default,
        rename = "nbf",
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub not_before: Option<OffsetDateTime>,

    /// softDelete data retention days. Value should be >=7 and <=90 when softDelete enabled, otherwise 0.
    #[serde(rename = "recoverableDays", skip_serializing_if = "Option::is_none")]
    pub recoverable_days: Option<i32>,

    /// Reflects the deletion recovery level currently in effect for secrets in the current vault. If it contains 'Purgeable',
    /// the secret can be permanently deleted by a privileged user; otherwise, only the system can purge the secret, at the end
    /// of the retention interval.
    #[serde(rename = "recoveryLevel", skip_serializing_if = "Option::is_none")]
    pub recovery_level: Option<DeletionRecoveryLevel>,

    /// Last updated time in UTC.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "azure_core::date::unix_time::option"
    )]
    pub updated: Option<OffsetDateTime>,
}

/// A secret consisting of a value, id and its attributes.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretBundle {
    /// The secret management attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,

    /// The content type of the secret.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The secret id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// If this is a secret backing a KV certificate, then this field specifies the corresponding key backing the KV certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,

    /// True if the secret's lifetime is managed by key vault. If this is a secret backing a certificate, then managed will be
    /// true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    /// Application specific metadata in the form of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,

    /// The secret value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The secret item containing secret metadata.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretItem {
    /// The secret management attributes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SecretAttributes>,

    /// Type of the secret value such as a password.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Secret identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// True if the secret's lifetime is managed by key vault. If this is a key backing a certificate, then managed will be true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed: Option<bool>,

    /// Application specific metadata in the form of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

/// The secret list result.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretListResult {
    /// The URL to get the next set of secrets.
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,

    /// A response message containing a list of secrets in the key vault along with a link to the next page of secrets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<SecretItem>>,
}

/// The secret restore parameters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretRestoreParameters {
    /// The backup blob associated with a secret bundle.
    #[serde(
        default,
        deserialize_with = "base64::deserialize_url_safe",
        rename = "value",
        serialize_with = "base64::serialize_url_safe",
        skip_serializing_if = "Option::is_none"
    )]
    pub secret_bundle_backup: Option<Vec<u8>>,
}

/// The secret set parameters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretSetParameters {
    /// Type of the secret value such as a password.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The secret management attributes.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub secret_attributes: Option<SecretAttributes>,

    /// Application specific metadata in the form of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,

    /// The value of the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// The secret update parameters.
#[derive(Clone, Debug, Default, Deserialize, Serialize, azure_core::Model)]
#[non_exhaustive]
pub struct SecretUpdateParameters {
    /// Type of the secret value such as a password.
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// The secret management attributes.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub secret_attributes: Option<SecretAttributes>,

    /// Application specific metadata in the form of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}
