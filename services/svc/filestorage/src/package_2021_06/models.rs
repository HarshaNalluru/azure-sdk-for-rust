#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "An Access policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AccessPolicy {
    #[doc = "The date-time the policy is active."]
    #[serde(rename = "Start", default, skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[doc = "The date-time the policy expires."]
    #[serde(rename = "Expiry", default, skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[doc = "The permissions for the ACL policy."]
    #[serde(rename = "Permission", default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}
impl AccessPolicy {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClearRange {
    #[serde(rename = "Start")]
    pub start: i64,
    #[serde(rename = "End")]
    pub end: i64,
}
impl ClearRange {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}
#[doc = "CORS is an HTTP feature that enables a web application running under one domain to access resources in another domain. Web browsers implement a security restriction known as same-origin policy that prevents a web page from calling APIs in a different domain; CORS provides a secure way to allow one domain (the origin domain) to call APIs in another domain."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CorsRule {
    #[doc = "The origin domains that are permitted to make a request against the storage service via CORS. The origin domain is the domain from which the request originates. Note that the origin must be an exact case-sensitive match with the origin that the user age sends to the service. You can also use the wildcard character '*' to allow all origin domains to make requests via CORS."]
    #[serde(rename = "AllowedOrigins")]
    pub allowed_origins: String,
    #[doc = "The methods (HTTP request verbs) that the origin domain may use for a CORS request. (comma separated)"]
    #[serde(rename = "AllowedMethods")]
    pub allowed_methods: String,
    #[doc = "The request headers that the origin domain may specify on the CORS request."]
    #[serde(rename = "AllowedHeaders")]
    pub allowed_headers: String,
    #[doc = "The response headers that may be sent in the response to the CORS request and exposed by the browser to the request issuer."]
    #[serde(rename = "ExposedHeaders")]
    pub exposed_headers: String,
    #[doc = "The maximum amount time that a browser should cache the preflight OPTIONS request."]
    #[serde(rename = "MaxAgeInSeconds")]
    pub max_age_in_seconds: i64,
}
impl CorsRule {
    pub fn new(
        allowed_origins: String,
        allowed_methods: String,
        allowed_headers: String,
        exposed_headers: String,
        max_age_in_seconds: i64,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
            exposed_headers,
            max_age_in_seconds,
        }
    }
}
#[doc = "A listed directory item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "FileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[doc = "File properties."]
    #[serde(rename = "Properties", default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<FileProperty>,
    #[serde(rename = "Attributes", default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "PermissionKey", default, skip_serializing_if = "Option::is_none")]
    pub permission_key: Option<String>,
}
impl DirectoryItem {
    pub fn new(name: String) -> Self {
        Self {
            name,
            file_id: None,
            properties: None,
            attributes: None,
            permission_key: None,
        }
    }
}
#[doc = "Error codes returned by the service"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorCode {
    AccountAlreadyExists,
    AccountBeingCreated,
    AccountIsDisabled,
    AuthenticationFailed,
    AuthorizationFailure,
    ConditionHeadersNotSupported,
    ConditionNotMet,
    EmptyMetadataKey,
    InsufficientAccountPermissions,
    InternalError,
    InvalidAuthenticationInfo,
    InvalidHeaderValue,
    InvalidHttpVerb,
    InvalidInput,
    InvalidMd5,
    InvalidMetadata,
    InvalidQueryParameterValue,
    InvalidRange,
    InvalidResourceName,
    InvalidUri,
    InvalidXmlDocument,
    InvalidXmlNodeValue,
    Md5Mismatch,
    MetadataTooLarge,
    MissingContentLengthHeader,
    MissingRequiredQueryParameter,
    MissingRequiredHeader,
    MissingRequiredXmlNode,
    MultipleConditionHeadersNotSupported,
    OperationTimedOut,
    OutOfRangeInput,
    OutOfRangeQueryParameterValue,
    RequestBodyTooLarge,
    ResourceTypeMismatch,
    RequestUrlFailedToParse,
    ResourceAlreadyExists,
    ResourceNotFound,
    ServerBusy,
    UnsupportedHeader,
    UnsupportedXmlNode,
    UnsupportedQueryParameter,
    UnsupportedHttpVerb,
    CannotDeleteFileOrDirectory,
    ClientCacheFlushDelay,
    DeletePending,
    DirectoryNotEmpty,
    FileLockConflict,
    InvalidFileOrDirectoryPathName,
    ParentNotFound,
    ReadOnlyAttribute,
    ShareAlreadyExists,
    ShareBeingDeleted,
    ShareDisabled,
    ShareNotFound,
    SharingViolation,
    ShareSnapshotInProgress,
    ShareSnapshotCountExceeded,
    ShareSnapshotOperationNotSupported,
    ShareHasSnapshots,
    ContainerQuotaDowngradeNotAllowed,
    #[serde(rename = "AuthorizationSourceIPMismatch")]
    AuthorizationSourceIpMismatch,
    AuthorizationProtocolMismatch,
    AuthorizationPermissionMismatch,
    AuthorizationServiceMismatch,
    AuthorizationResourceTypeMismatch,
    FeatureVersionMismatch,
}
#[doc = "A listed file item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileItem {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "FileId", default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[doc = "File properties."]
    #[serde(rename = "Properties")]
    pub properties: FileProperty,
    #[serde(rename = "Attributes", default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "PermissionKey", default, skip_serializing_if = "Option::is_none")]
    pub permission_key: Option<String>,
}
impl FileItem {
    pub fn new(name: String, properties: FileProperty) -> Self {
        Self {
            name,
            file_id: None,
            properties,
            attributes: None,
            permission_key: None,
        }
    }
}
#[doc = "File properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileProperty {
    #[doc = "Content length of the file. This value may not be up-to-date since an SMB client may have modified the file locally. The value of Content-Length may not reflect that fact until the handle is closed or the op-lock is broken. To retrieve current property values, call Get File Properties."]
    #[serde(rename = "Content-Length")]
    pub content_length: i64,
    #[serde(rename = "CreationTime", default, skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "LastAccessTime", default, skip_serializing_if = "Option::is_none")]
    pub last_access_time: Option<String>,
    #[serde(rename = "LastWriteTime", default, skip_serializing_if = "Option::is_none")]
    pub last_write_time: Option<String>,
    #[serde(rename = "ChangeTime", default, skip_serializing_if = "Option::is_none")]
    pub change_time: Option<String>,
    #[serde(rename = "Last-Modified", default, skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "Etag", default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
}
impl FileProperty {
    pub fn new(content_length: i64) -> Self {
        Self {
            content_length,
            creation_time: None,
            last_access_time: None,
            last_write_time: None,
            change_time: None,
            last_modified: None,
            etag: None,
        }
    }
}
#[doc = "An Azure Storage file range."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileRange {
    #[doc = "Start of the range."]
    #[serde(rename = "Start")]
    pub start: i64,
    #[doc = "End of the range."]
    #[serde(rename = "End")]
    pub end: i64,
}
impl FileRange {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}
#[doc = "Abstract for entries that can be listed from Directory."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilesAndDirectoriesListSegment {
    #[serde(rename = "DirectoryItems")]
    pub directory_items: Vec<DirectoryItem>,
    #[serde(rename = "FileItems")]
    pub file_items: Vec<FileItem>,
}
impl FilesAndDirectoriesListSegment {
    pub fn new(directory_items: Vec<DirectoryItem>, file_items: Vec<FileItem>) -> Self {
        Self {
            directory_items,
            file_items,
        }
    }
}
#[doc = "A listed Azure Storage handle item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HandleItem {
    #[doc = "XSMB service handle ID"]
    #[serde(rename = "HandleId")]
    pub handle_id: String,
    #[doc = "File or directory name including full path starting from share root"]
    #[serde(rename = "Path")]
    pub path: String,
    #[doc = "FileId uniquely identifies the file or directory."]
    #[serde(rename = "FileId")]
    pub file_id: String,
    #[doc = "ParentId uniquely identifies the parent directory of the object."]
    #[serde(rename = "ParentId", default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "SMB session ID in context of which the file handle was opened"]
    #[serde(rename = "SessionId")]
    pub session_id: String,
    #[doc = "Client IP that opened the handle"]
    #[serde(rename = "ClientIp")]
    pub client_ip: String,
    #[doc = "Time when the session that previously opened the handle has last been reconnected. (UTC)"]
    #[serde(rename = "OpenTime")]
    pub open_time: String,
    #[doc = "Time handle was last connected to (UTC)"]
    #[serde(rename = "LastReconnectTime", default, skip_serializing_if = "Option::is_none")]
    pub last_reconnect_time: Option<String>,
}
impl HandleItem {
    pub fn new(handle_id: String, path: String, file_id: String, session_id: String, client_ip: String, open_time: String) -> Self {
        Self {
            handle_id,
            path,
            file_id,
            parent_id: None,
            session_id,
            client_ip,
            open_time,
            last_reconnect_time: None,
        }
    }
}
#[doc = "When a share is leased, specifies whether the lease is of infinite or fixed duration."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaseDuration {
    #[serde(rename = "infinite")]
    Infinite,
    #[serde(rename = "fixed")]
    Fixed,
}
#[doc = "Lease state of the share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaseState {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "leased")]
    Leased,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "breaking")]
    Breaking,
    #[serde(rename = "broken")]
    Broken,
}
#[doc = "The current lease status of the share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LeaseStatus {
    #[serde(rename = "locked")]
    Locked,
    #[serde(rename = "unlocked")]
    Unlocked,
}
#[doc = "An enumeration of directories and files."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFilesAndDirectoriesSegmentResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "ShareName")]
    pub share_name: String,
    #[serde(rename = "ShareSnapshot", default, skip_serializing_if = "Option::is_none")]
    pub share_snapshot: Option<String>,
    #[serde(rename = "DirectoryPath")]
    pub directory_path: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[doc = "Abstract for entries that can be listed from Directory."]
    #[serde(rename = "Segment")]
    pub segment: FilesAndDirectoriesListSegment,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
    #[serde(rename = "DirectoryId", default, skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
}
impl azure_core::Continuable for ListFilesAndDirectoriesSegmentResponse {
    fn continuation(&self) -> Option<String> {
        if self.next_marker.is_empty() {
            None
        } else {
            Some(self.next_marker.clone())
        }
    }
}
impl ListFilesAndDirectoriesSegmentResponse {
    pub fn new(
        service_endpoint: String,
        share_name: String,
        directory_path: String,
        prefix: String,
        segment: FilesAndDirectoriesListSegment,
        next_marker: String,
    ) -> Self {
        Self {
            service_endpoint,
            share_name,
            share_snapshot: None,
            directory_path,
            prefix,
            marker: None,
            max_results: None,
            segment,
            next_marker,
            directory_id: None,
        }
    }
}
#[doc = "An enumeration of handles."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListHandlesResponse {
    #[serde(rename = "HandleList", default, skip_serializing_if = "Vec::is_empty")]
    pub handle_list: Vec<HandleItem>,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
}
impl ListHandlesResponse {
    pub fn new(next_marker: String) -> Self {
        Self {
            handle_list: Vec::new(),
            next_marker,
        }
    }
}
#[doc = "An enumeration of shares."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListSharesResponse {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Prefix", default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker", default, skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults", default, skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "ShareItems", default, skip_serializing_if = "Vec::is_empty")]
    pub share_items: Vec<ShareItemInternal>,
    #[serde(rename = "NextMarker")]
    pub next_marker: String,
}
impl azure_core::Continuable for ListSharesResponse {
    fn continuation(&self) -> Option<String> {
        if self.next_marker.is_empty() {
            None
        } else {
            Some(self.next_marker.clone())
        }
    }
}
impl ListSharesResponse {
    pub fn new(service_endpoint: String, next_marker: String) -> Self {
        Self {
            service_endpoint,
            prefix: None,
            marker: None,
            max_results: None,
            share_items: Vec::new(),
            next_marker,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Metadata {}
impl Metadata {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage Analytics metrics for file service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Metrics {
    #[doc = "The version of Storage Analytics to configure."]
    #[serde(rename = "Version")]
    pub version: String,
    #[doc = "Indicates whether metrics are enabled for the File service."]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates whether metrics should generate summary statistics for called API operations."]
    #[serde(rename = "IncludeAPIs", default, skip_serializing_if = "Option::is_none")]
    pub include_ap_is: Option<bool>,
    #[doc = "The retention policy."]
    #[serde(rename = "RetentionPolicy", default, skip_serializing_if = "Option::is_none")]
    pub retention_policy: Option<RetentionPolicy>,
}
impl Metrics {
    pub fn new(version: String, enabled: bool) -> Self {
        Self {
            version,
            enabled,
            include_ap_is: None,
            retention_policy: None,
        }
    }
}
#[doc = "The retention policy."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    #[doc = "Indicates whether a retention policy is enabled for the File service. If false, metrics data is retained, and the user is responsible for deleting it."]
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[doc = "Indicates the number of days that metrics data should be retained. All data older than this value will be deleted. Metrics data is deleted on a best-effort basis after the retention period expires."]
    #[serde(rename = "Days", default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}
impl RetentionPolicy {
    pub fn new(enabled: bool) -> Self {
        Self { enabled, days: None }
    }
}
pub type ShareEnabledProtocols = String;
#[doc = "The list of file ranges"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareFileRangeList {
    #[serde(rename = "Ranges", default, skip_serializing_if = "Vec::is_empty")]
    pub ranges: Vec<FileRange>,
    #[serde(rename = "ClearRanges", default, skip_serializing_if = "Vec::is_empty")]
    pub clear_ranges: Vec<ClearRange>,
}
impl ShareFileRangeList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A listed Azure Storage share item."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareItemInternal {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Snapshot", default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[serde(rename = "Deleted", default, skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Properties of a share."]
    #[serde(rename = "Properties")]
    pub properties: SharePropertiesInternal,
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}
impl ShareItemInternal {
    pub fn new(name: String, properties: SharePropertiesInternal) -> Self {
        Self {
            name,
            snapshot: None,
            deleted: None,
            version: None,
            properties,
            metadata: None,
        }
    }
}
#[doc = "A permission (a security descriptor) at the share level."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharePermission {
    #[doc = "The permission in the Security Descriptor Definition Language (SDDL)."]
    pub permission: String,
}
impl SharePermission {
    pub fn new(permission: String) -> Self {
        Self { permission }
    }
}
#[doc = "Properties of a share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SharePropertiesInternal {
    #[serde(rename = "Last-Modified")]
    pub last_modified: String,
    #[serde(rename = "Etag")]
    pub etag: String,
    #[serde(rename = "Quota")]
    pub quota: i64,
    #[serde(rename = "ProvisionedIops", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<i64>,
    #[serde(rename = "ProvisionedIngressMBps", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_ingress_m_bps: Option<i64>,
    #[serde(rename = "ProvisionedEgressMBps", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_egress_m_bps: Option<i64>,
    #[serde(rename = "ProvisionedBandwidthMiBps", default, skip_serializing_if = "Option::is_none")]
    pub provisioned_bandwidth_mi_bps: Option<i64>,
    #[serde(rename = "NextAllowedQuotaDowngradeTime", default, skip_serializing_if = "Option::is_none")]
    pub next_allowed_quota_downgrade_time: Option<String>,
    #[serde(rename = "DeletedTime", default, skip_serializing_if = "Option::is_none")]
    pub deleted_time: Option<String>,
    #[serde(rename = "RemainingRetentionDays", default, skip_serializing_if = "Option::is_none")]
    pub remaining_retention_days: Option<i64>,
    #[serde(rename = "AccessTier", default, skip_serializing_if = "Option::is_none")]
    pub access_tier: Option<String>,
    #[serde(rename = "AccessTierChangeTime", default, skip_serializing_if = "Option::is_none")]
    pub access_tier_change_time: Option<String>,
    #[serde(rename = "AccessTierTransitionState", default, skip_serializing_if = "Option::is_none")]
    pub access_tier_transition_state: Option<String>,
    #[doc = "The current lease status of the share."]
    #[serde(rename = "LeaseStatus", default, skip_serializing_if = "Option::is_none")]
    pub lease_status: Option<LeaseStatus>,
    #[doc = "Lease state of the share."]
    #[serde(rename = "LeaseState", default, skip_serializing_if = "Option::is_none")]
    pub lease_state: Option<LeaseState>,
    #[doc = "When a share is leased, specifies whether the lease is of infinite or fixed duration."]
    #[serde(rename = "LeaseDuration", default, skip_serializing_if = "Option::is_none")]
    pub lease_duration: Option<LeaseDuration>,
    #[serde(rename = "EnabledProtocols", default, skip_serializing_if = "Option::is_none")]
    pub enabled_protocols: Option<ShareEnabledProtocols>,
    #[serde(rename = "RootSquash", default, skip_serializing_if = "Option::is_none")]
    pub root_squash: Option<ShareRootSquash>,
}
impl SharePropertiesInternal {
    pub fn new(last_modified: String, etag: String, quota: i64) -> Self {
        Self {
            last_modified,
            etag,
            quota,
            provisioned_iops: None,
            provisioned_ingress_m_bps: None,
            provisioned_egress_m_bps: None,
            provisioned_bandwidth_mi_bps: None,
            next_allowed_quota_downgrade_time: None,
            deleted_time: None,
            remaining_retention_days: None,
            access_tier: None,
            access_tier_change_time: None,
            access_tier_transition_state: None,
            lease_status: None,
            lease_state: None,
            lease_duration: None,
            enabled_protocols: None,
            root_squash: None,
        }
    }
}
#[doc = "Protocol settings"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareProtocolSettings {
    #[doc = "Settings for SMB protocol."]
    #[serde(rename = "Smb", default, skip_serializing_if = "Option::is_none")]
    pub smb: Option<ShareSmbSettings>,
}
impl ShareProtocolSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ShareRootSquash {
    NoRootSquash,
    RootSquash,
    AllSquash,
}
#[doc = "Settings for SMB protocol."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ShareSmbSettings {
    #[doc = "Settings for SMB multichannel"]
    #[serde(rename = "Multichannel", default, skip_serializing_if = "Option::is_none")]
    pub multichannel: Option<SmbMultichannel>,
}
impl ShareSmbSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Stats for the share."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShareStats {
    #[doc = "The approximate size of the data stored in bytes. Note that this value may not include all recently created or recently resized files."]
    #[serde(rename = "ShareUsageBytes")]
    pub share_usage_bytes: i64,
}
impl ShareStats {
    pub fn new(share_usage_bytes: i64) -> Self {
        Self { share_usage_bytes }
    }
}
#[doc = "Signed identifier."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignedIdentifier {
    #[doc = "A unique id."]
    #[serde(rename = "Id")]
    pub id: String,
    #[doc = "An Access policy."]
    #[serde(rename = "AccessPolicy", default, skip_serializing_if = "Option::is_none")]
    pub access_policy: Option<AccessPolicy>,
}
impl SignedIdentifier {
    pub fn new(id: String) -> Self {
        Self { id, access_policy: None }
    }
}
pub type SignedIdentifiers = Vec<SignedIdentifier>;
#[doc = "Settings for SMB multichannel"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SmbMultichannel {
    #[doc = "If SMB multichannel is enabled."]
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
impl SmbMultichannel {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageError {
    #[serde(rename = "Message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl azure_core::Continuable for StorageError {
    fn continuation(&self) -> Option<String> {
        None
    }
}
impl StorageError {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Storage service properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct StorageServiceProperties {
    #[doc = "Storage Analytics metrics for file service."]
    #[serde(rename = "HourMetrics", default, skip_serializing_if = "Option::is_none")]
    pub hour_metrics: Option<Metrics>,
    #[doc = "Storage Analytics metrics for file service."]
    #[serde(rename = "MinuteMetrics", default, skip_serializing_if = "Option::is_none")]
    pub minute_metrics: Option<Metrics>,
    #[doc = "The set of CORS rules."]
    #[serde(rename = "Cors", default, skip_serializing_if = "Vec::is_empty")]
    pub cors: Vec<CorsRule>,
    #[doc = "Protocol settings"]
    #[serde(rename = "Protocol", default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ShareProtocolSettings>,
}
impl StorageServiceProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
