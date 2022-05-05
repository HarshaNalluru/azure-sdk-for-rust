#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<String>,
    #[doc = "Url to get the next page of reservations"]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl AppliedReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservations {
    #[doc = "Identifier of the applied reservations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of resource. \"Microsoft.Capacity/AppliedReservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<AppliedReservationsProperties>,
}
impl AppliedReservations {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct AppliedReservationsProperties {
    #[serde(rename = "reservationOrderIds", default, skip_serializing_if = "Option::is_none")]
    pub reservation_order_ids: Option<AppliedReservationList>,
}
impl AppliedReservationsProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Type of the Applied Scope."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum AppliedScopeType {
    Single,
    Shared,
}
pub type AppliedScopes = Vec<String>;
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Catalog {
    #[doc = "The type of resource the SKU applies to."]
    #[serde(rename = "resourceType", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The name of SKU"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Available reservation terms for this resource"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub terms: Vec<ReservationTerm>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<String>,
    #[serde(rename = "skuProperties", default, skip_serializing_if = "Vec::is_empty")]
    pub sku_properties: Vec<SkuProperty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<SkuRestriction>,
}
impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Error {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ExtendedErrorInfo>,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorResponseCode {
    NotSpecified,
    InternalServerError,
    ServerTimeout,
    AuthorizationFailed,
    BadRequest,
    ClientCertificateThumbprintNotSet,
    InvalidRequestContent,
    OperationFailed,
    HttpMethodNotSupported,
    InvalidRequestUri,
    MissingTenantId,
    InvalidTenantId,
    InvalidReservationOrderId,
    InvalidReservationId,
    ReservationIdNotInReservationOrder,
    ReservationOrderNotFound,
    InvalidSubscriptionId,
    InvalidAccessToken,
    InvalidLocationId,
    UnauthenticatedRequestsThrottled,
    InvalidHealthCheckType,
    Forbidden,
    BillingScopeIdCannotBeChanged,
    AppliedScopesNotAssociatedWithCommerceAccount,
    PatchValuesSameAsExisting,
    RoleAssignmentCreationFailed,
    ReservationOrderCreationFailed,
    ReservationOrderNotEnabled,
    CapacityUpdateScopesFailed,
    UnsupportedReservationTerm,
    ReservationOrderIdAlreadyExists,
    RiskCheckFailed,
    CreateQuoteFailed,
    ActivateQuoteFailed,
    NonsupportedAccountId,
    PaymentInstrumentNotFound,
    MissingAppliedScopesForSingle,
    NoValidReservationsToReRate,
    #[serde(rename = "ReRateOnlyAllowedForEA")]
    ReRateOnlyAllowedForEa,
    OperationCannotBePerformedInCurrentState,
    InvalidSingleAppliedScopesCount,
    InvalidFulfillmentRequestParameters,
    NotSupportedCountry,
    InvalidRefundQuantity,
    PurchaseError,
    BillingCustomerInputError,
    BillingPaymentInstrumentSoftError,
    BillingPaymentInstrumentHardError,
    BillingTransientError,
    BillingError,
    FulfillmentConfigurationError,
    FulfillmentOutOfStockError,
    FulfillmentTransientError,
    FulfillmentError,
    CalculatePriceFailed,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedErrorInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<ErrorResponseCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedErrorInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ExtendedStatusInfo {
    #[serde(rename = "statusCode", default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<ReservationStatusCode>,
    #[doc = "The message giving detailed information about the status code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ExtendedStatusInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Allows reservation discount to be applied across skus within the same Autofit group. Not all skus support instance size flexibility."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum InstanceFlexibility {
    On,
    Off,
    NotSupported,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeProperties {
    #[doc = "Format of the resource id should be /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<String>,
}
impl MergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct MergeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<MergeProperties>,
}
impl MergeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationDisplay {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl OperationDisplay {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<OperationResponse>,
    #[doc = "Url to get the next page of items."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for OperationList {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl OperationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<OperationDisplay>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
impl OperationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Patch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<PatchProperties>,
}
impl Patch {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PatchProperties {
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Allows reservation discount to be applied across skus within the same Autofit group. Not all skus support instance size flexibility."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Name of the Reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl PatchProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationResponse>,
    #[doc = "Url to get the next page of reservations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationList {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl ReservationList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationMergeProperties {
    #[doc = "Reservation Resource Id Created due to the merge. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "mergeDestination", default, skip_serializing_if = "Option::is_none")]
    pub merge_destination: Option<String>,
    #[doc = "Resource Ids of the Source Reservation's merged to form this Reservation. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "mergeSources", default, skip_serializing_if = "Vec::is_empty")]
    pub merge_sources: Vec<String>,
}
impl ReservationMergeProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ReservationOrderResponse>,
    #[doc = "Url to get the next page of reservationOrders."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ReservationOrderList {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl ReservationOrderList {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderProperties {
    #[doc = "Friendly name for user to easily identified the reservation."]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "This is the DateTime when the reservation was initially requested for purchase."]
    #[serde(rename = "requestDateTime", default, skip_serializing_if = "Option::is_none")]
    pub request_date_time: Option<String>,
    #[doc = "This is the DateTime when the reservation was created."]
    #[serde(rename = "createdDateTime", default, skip_serializing_if = "Option::is_none")]
    pub created_date_time: Option<String>,
    #[doc = "This is the date when the Reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "Total Quantity of the SKUs purchased in the Reservation."]
    #[serde(rename = "originalQuantity", default, skip_serializing_if = "Option::is_none")]
    pub original_quantity: Option<i32>,
    #[doc = "Represent the term of Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<ReservationTerm>,
    #[doc = "Current state of the reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reservations: Vec<ReservationResponse>,
}
impl ReservationOrderProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationOrderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationOrderProperties>,
    #[doc = "Type of resource. \"Microsoft.Capacity/reservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReservationOrderResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationProperties {
    #[doc = "The type of the resource that is being reserved."]
    #[serde(rename = "reservedResourceType", default, skip_serializing_if = "Option::is_none")]
    pub reserved_resource_type: Option<ReservedResourceType>,
    #[doc = "Allows reservation discount to be applied across skus within the same Autofit group. Not all skus support instance size flexibility."]
    #[serde(rename = "instanceFlexibility", default, skip_serializing_if = "Option::is_none")]
    pub instance_flexibility: Option<InstanceFlexibility>,
    #[doc = "Friendly name for user to easily identify the reservation"]
    #[serde(rename = "displayName", default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "List of the subscriptions that the benefit will be applied. Do not specify if AppliedScopeType is Shared."]
    #[serde(rename = "appliedScopes", default, skip_serializing_if = "Option::is_none")]
    pub applied_scopes: Option<AppliedScopes>,
    #[doc = "Type of the Applied Scope."]
    #[serde(rename = "appliedScopeType", default, skip_serializing_if = "Option::is_none")]
    pub applied_scope_type: Option<AppliedScopeType>,
    #[doc = "Quantity of the SKUs that are part of the Reservation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[doc = "Current state of the reservation."]
    #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<String>,
    #[doc = "DateTime of the Reservation starting when this version is effective from."]
    #[serde(rename = "effectiveDateTime", default, skip_serializing_if = "Option::is_none")]
    pub effective_date_time: Option<String>,
    #[doc = "DateTime of the last time the Reservation was updated."]
    #[serde(rename = "lastUpdatedDateTime", default, skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<String>,
    #[doc = "This is the date when the Reservation will expire."]
    #[serde(rename = "expiryDate", default, skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[doc = "Description of the SKU in english."]
    #[serde(rename = "skuDescription", default, skip_serializing_if = "Option::is_none")]
    pub sku_description: Option<String>,
    #[serde(rename = "extendedStatusInfo", default, skip_serializing_if = "Option::is_none")]
    pub extended_status_info: Option<ExtendedStatusInfo>,
    #[serde(rename = "splitProperties", default, skip_serializing_if = "Option::is_none")]
    pub split_properties: Option<ReservationSplitProperties>,
    #[serde(rename = "mergeProperties", default, skip_serializing_if = "Option::is_none")]
    pub merge_properties: Option<ReservationMergeProperties>,
}
impl ReservationProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationResponse {
    #[doc = "The Azure Region where the reserved resource lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<i64>,
    #[doc = "Identifier of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the reservation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sku: Option<SkuName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<ReservationProperties>,
    #[doc = "Type of resource. \"Microsoft.Capacity/reservationOrders/reservations\""]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl ReservationResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ReservationSplitProperties {
    #[doc = "List of destination Resource Id that are created due to split. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "splitDestinations", default, skip_serializing_if = "Vec::is_empty")]
    pub split_destinations: Vec<String>,
    #[doc = "Resource Id of the Reservation from which this is split. Format of the resource Id is /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "splitSource", default, skip_serializing_if = "Option::is_none")]
    pub split_source: Option<String>,
}
impl ReservationSplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationStatusCode {
    None,
    Pending,
    Active,
    PurchaseError,
    PaymentInstrumentError,
    Split,
    Merged,
    Expired,
    Succeeded,
}
#[doc = "Represent the term of Reservation."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservationTerm {
    #[serde(rename = "P1Y")]
    P1y,
    #[serde(rename = "P3Y")]
    P3y,
}
#[doc = "The type of the resource that is being reserved."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ReservedResourceType {
    VirtualMachines,
    SqlDatabases,
    SuseLinux,
    CosmosDb,
    RedHat,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuName {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl SkuName {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuProperty {
    #[doc = "An invariant to describe the feature."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "An invariant if the feature is measured by quantity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl SkuProperty {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SkuRestriction {
    #[doc = "The type of restrictions."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The value of restrictions. If the restriction type is set to location. This would be different locations where the SKU is restricted."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<String>,
    #[doc = "The reason for restriction."]
    #[serde(rename = "reasonCode", default, skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}
impl SkuRestriction {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitProperties {
    #[doc = "List of the quantities in the new reservations to create."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quantities: Vec<i64>,
    #[doc = "Resource id of the reservation to be split. Format of the resource id should be /providers/Microsoft.Capacity/reservationOrders/{reservationOrderId}/reservations/{reservationId}"]
    #[serde(rename = "reservationId", default, skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
}
impl SplitProperties {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SplitRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<SplitProperties>,
}
impl SplitRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
