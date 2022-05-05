#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "A disk access SAS uri."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessUri {
    #[doc = "A SAS uri for accessing a disk."]
    #[serde(rename = "accessSAS", default, skip_serializing_if = "Option::is_none")]
    pub access_sas: Option<String>,
}
impl AccessUri {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data used when creating a disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationData {
    #[doc = "This enumerates the possible sources of a disk's creation."]
    #[serde(rename = "createOption")]
    pub create_option: creation_data::CreateOption,
    #[doc = "If createOption is Import, the Azure Resource Manager identifier of the storage account containing the blob to import as a disk. Required only if the blob is in a different subscription"]
    #[serde(rename = "storageAccountId", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_id: Option<String>,
    #[doc = "The source image used for creating the disk."]
    #[serde(rename = "imageReference", default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<ImageDiskReference>,
    #[doc = "If createOption is Import, this is the URI of a blob to be imported into a managed disk."]
    #[serde(rename = "sourceUri", default, skip_serializing_if = "Option::is_none")]
    pub source_uri: Option<String>,
    #[doc = "If createOption is Copy, this is the ARM id of the source snapshot or disk."]
    #[serde(rename = "sourceResourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_resource_id: Option<String>,
}
impl CreationData {
    pub fn new(create_option: creation_data::CreateOption) -> Self {
        Self {
            create_option,
            storage_account_id: None,
            image_reference: None,
            source_uri: None,
            source_resource_id: None,
        }
    }
}
pub mod creation_data {
    use super::*;
    #[doc = "This enumerates the possible sources of a disk's creation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum CreateOption {
        Empty,
        Attach,
        FromImage,
        Import,
        Copy,
        Restore,
    }
}
#[doc = "Disk resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Disk {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "A relative URI containing the ID of the VM that has the disk attached."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "The disks sku name. Can be Standard_LRS, Premium_LRS, or StandardSSD_LRS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
    #[doc = "The Logical zone list for Disk."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub zones: Vec<String>,
    #[doc = "Disk resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}
impl Disk {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            managed_by: None,
            sku: None,
            zones: Vec::new(),
            properties: None,
        }
    }
}
#[doc = "The List Disks operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskList {
    #[doc = "A list of disks."]
    pub value: Vec<Disk>,
    #[doc = "The uri to fetch the next page of disks. Call ListNext() with this to fetch the next page of disks."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for DiskList {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl DiskList {
    pub fn new(value: Vec<Disk>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "Disk resource properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskProperties {
    #[doc = "The time when the disk was created."]
    #[serde(rename = "timeCreated", default, skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[doc = "The Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_properties::OsType>,
    #[doc = "Data used when creating a disk."]
    #[serde(rename = "creationData")]
    pub creation_data: CreationData,
    #[doc = "If creationData.createOption is Empty, this field is mandatory and it indicates the size of the VHD to create. If this field is present for updates or creation with other options, it indicates a resize. Resizes are only allowed if the disk is not attached to a running VM, and can only increase the disk's size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Encryption settings for disk or snapshot"]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings: Option<EncryptionSettings>,
    #[doc = "The disk provisioning state."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
}
impl DiskProperties {
    pub fn new(creation_data: CreationData) -> Self {
        Self {
            time_created: None,
            os_type: None,
            creation_data,
            disk_size_gb: None,
            encryption_settings: None,
            provisioning_state: None,
        }
    }
}
pub mod disk_properties {
    use super::*;
    #[doc = "The Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "The disks sku name. Can be Standard_LRS, Premium_LRS, or StandardSSD_LRS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskSku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<disk_sku::Name>,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl DiskSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_sku {
    use super::*;
    #[doc = "The sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "StandardSSD_LRS")]
        StandardSsdLrs,
    }
}
#[doc = "Disk update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskUpdate {
    #[doc = "Disk resource update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskUpdateProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The disks sku name. Can be Standard_LRS, Premium_LRS, or StandardSSD_LRS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<DiskSku>,
}
impl DiskUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Disk resource update properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct DiskUpdateProperties {
    #[doc = "the Operating System type."]
    #[serde(rename = "osType", default, skip_serializing_if = "Option::is_none")]
    pub os_type: Option<disk_update_properties::OsType>,
    #[doc = "If creationData.createOption is Empty, this field is mandatory and it indicates the size of the VHD to create. If this field is present for updates or creation with other options, it indicates a resize. Resizes are only allowed if the disk is not attached to a running VM, and can only increase the disk's size."]
    #[serde(rename = "diskSizeGB", default, skip_serializing_if = "Option::is_none")]
    pub disk_size_gb: Option<i32>,
    #[doc = "Encryption settings for disk or snapshot"]
    #[serde(rename = "encryptionSettings", default, skip_serializing_if = "Option::is_none")]
    pub encryption_settings: Option<EncryptionSettings>,
}
impl DiskUpdateProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod disk_update_properties {
    use super::*;
    #[doc = "the Operating System type."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[doc = "Encryption settings for disk or snapshot"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EncryptionSettings {
    #[doc = "Set this flag to true and provide DiskEncryptionKey and optional KeyEncryptionKey to enable encryption. Set this flag to false and remove DiskEncryptionKey and KeyEncryptionKey to disable encryption. If EncryptionSettings is null in the request object, the existing settings remain unchanged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Key Vault Secret Url and vault id of the encryption key "]
    #[serde(rename = "diskEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub disk_encryption_key: Option<KeyVaultAndSecretReference>,
    #[doc = "Key Vault Key Url and vault id of KeK, KeK is optional and when provided is used to unwrap the encryptionKey"]
    #[serde(rename = "keyEncryptionKey", default, skip_serializing_if = "Option::is_none")]
    pub key_encryption_key: Option<KeyVaultAndKeyReference>,
}
impl EncryptionSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Data used for requesting a SAS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GrantAccessData {
    pub access: grant_access_data::Access,
    #[doc = "Time duration in seconds until the SAS access expires."]
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i32,
}
impl GrantAccessData {
    pub fn new(access: grant_access_data::Access, duration_in_seconds: i32) -> Self {
        Self {
            access,
            duration_in_seconds,
        }
    }
}
pub mod grant_access_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Access {
        None,
        Read,
    }
}
#[doc = "The source image used for creating the disk."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDiskReference {
    #[doc = "A relative uri containing either a Platform Image Repository or user image reference."]
    pub id: String,
    #[doc = "If the disk is created from an image's data disk, this is an index that indicates which of the data disks in the image to use. For OS disks, this field is null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lun: Option<i32>,
}
impl ImageDiskReference {
    pub fn new(id: String) -> Self {
        Self { id, lun: None }
    }
}
#[doc = "Key Vault Key Url and vault id of KeK, KeK is optional and when provided is used to unwrap the encryptionKey"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndKeyReference {
    #[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[doc = "Url pointing to a key or secret in KeyVault"]
    #[serde(rename = "keyUrl")]
    pub key_url: String,
}
impl KeyVaultAndKeyReference {
    pub fn new(source_vault: SourceVault, key_url: String) -> Self {
        Self { source_vault, key_url }
    }
}
#[doc = "Key Vault Secret Url and vault id of the encryption key "]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyVaultAndSecretReference {
    #[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
    #[serde(rename = "sourceVault")]
    pub source_vault: SourceVault,
    #[doc = "Url pointing to a key or secret in KeyVault"]
    #[serde(rename = "secretUrl")]
    pub secret_url: String,
}
impl KeyVaultAndSecretReference {
    pub fn new(source_vault: SourceVault, secret_url: String) -> Self {
        Self { source_vault, secret_url }
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Resource name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Resource type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Resource location"]
    pub location: String,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new(location: String) -> Self {
        Self {
            id: None,
            name: None,
            type_: None,
            location,
            tags: None,
        }
    }
}
#[doc = "Snapshot resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snapshot {
    #[serde(flatten)]
    pub resource: Resource,
    #[doc = "Unused. Always Null."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "The snapshots sku name. Can be Standard_LRS, Premium_LRS, or Standard_ZRS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
    #[doc = "Disk resource properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskProperties>,
}
impl Snapshot {
    pub fn new(resource: Resource) -> Self {
        Self {
            resource,
            managed_by: None,
            sku: None,
            properties: None,
        }
    }
}
#[doc = "The List Snapshots operation response."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SnapshotList {
    #[doc = "A list of snapshots."]
    pub value: Vec<Snapshot>,
    #[doc = "The uri to fetch the next page of snapshots. Call ListNext() with this to fetch the next page of snapshots."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for SnapshotList {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl SnapshotList {
    pub fn new(value: Vec<Snapshot>) -> Self {
        Self { value, next_link: None }
    }
}
#[doc = "The snapshots sku name. Can be Standard_LRS, Premium_LRS, or Standard_ZRS."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotSku {
    #[doc = "The sku name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<snapshot_sku::Name>,
    #[doc = "The sku tier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
}
impl SnapshotSku {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod snapshot_sku {
    use super::*;
    #[doc = "The sku name."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Name {
        #[serde(rename = "Standard_LRS")]
        StandardLrs,
        #[serde(rename = "Premium_LRS")]
        PremiumLrs,
        #[serde(rename = "Standard_ZRS")]
        StandardZrs,
    }
}
#[doc = "Snapshot update resource."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SnapshotUpdate {
    #[doc = "Disk resource update properties."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<DiskUpdateProperties>,
    #[doc = "Resource tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[doc = "The snapshots sku name. Can be Standard_LRS, Premium_LRS, or Standard_ZRS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SnapshotSku>,
}
impl SnapshotUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The vault id is an Azure Resource Manager Resource id in the form /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.KeyVault/vaults/{vaultName}"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SourceVault {
    #[doc = "Resource Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl SourceVault {
    pub fn new() -> Self {
        Self::default()
    }
}
