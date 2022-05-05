#![doc = "generated by AutoRust"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "The properties of the Azure File volume. Azure File shares are mounted as volumes."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AzureFileVolume {
    #[doc = "The name of the Azure File share to be mounted as a volume."]
    #[serde(rename = "shareName")]
    pub share_name: String,
    #[doc = "The flag indicating whether the Azure File shared mounted as a volume is read-only."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[doc = "The name of the storage account that contains the Azure File share."]
    #[serde(rename = "storageAccountName")]
    pub storage_account_name: String,
    #[doc = "The storage account access key used to access the Azure File share."]
    #[serde(rename = "storageAccountKey", default, skip_serializing_if = "Option::is_none")]
    pub storage_account_key: Option<String>,
}
impl AzureFileVolume {
    pub fn new(share_name: String, storage_account_name: String) -> Self {
        Self {
            share_name,
            read_only: None,
            storage_account_name,
            storage_account_key: None,
        }
    }
}
#[doc = "A container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Container {
    #[doc = "The user-provided name of the container instance."]
    pub name: String,
    #[doc = "The container instance properties."]
    pub properties: ContainerProperties,
}
impl Container {
    pub fn new(name: String, properties: ContainerProperties) -> Self {
        Self { name, properties }
    }
}
#[doc = "The container execution command, for liveness or readiness probe"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExec {
    #[doc = "The commands to execute within the container."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
}
impl ContainerExec {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container exec request."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExecRequest {
    #[doc = "The command to be executed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[doc = "The size of the terminal."]
    #[serde(rename = "terminalSize", default, skip_serializing_if = "Option::is_none")]
    pub terminal_size: Option<container_exec_request::TerminalSize>,
}
impl ContainerExecRequest {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_exec_request {
    use super::*;
    #[doc = "The size of the terminal."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct TerminalSize {
        #[doc = "The row size of the terminal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rows: Option<i64>,
        #[doc = "The column size of the terminal"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cols: Option<i64>,
    }
    impl TerminalSize {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The information for the container exec command."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerExecResponse {
    #[doc = "The uri for the exec websocket."]
    #[serde(rename = "webSocketUri", default, skip_serializing_if = "Option::is_none")]
    pub web_socket_uri: Option<String>,
    #[doc = "The password to start the exec command."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ContainerExecResponse {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroup {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<container_group::Properties>,
}
impl ContainerGroup {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod container_group {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Properties {
        #[doc = "The provisioning state of the container group. This only appears in the response."]
        #[serde(rename = "provisioningState", default, skip_serializing_if = "Option::is_none")]
        pub provisioning_state: Option<String>,
        #[doc = "The containers within the container group."]
        pub containers: Vec<Container>,
        #[doc = "The image registry credentials by which the container group is created from."]
        #[serde(rename = "imageRegistryCredentials", default, skip_serializing_if = "Vec::is_empty")]
        pub image_registry_credentials: Vec<ImageRegistryCredential>,
        #[doc = "Restart policy for all containers within the container group. \n- `Always` Always restart\n- `OnFailure` Restart on failure\n- `Never` Never restart\n"]
        #[serde(rename = "restartPolicy", default, skip_serializing_if = "Option::is_none")]
        pub restart_policy: Option<properties::RestartPolicy>,
        #[doc = "IP address for the container group."]
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<IpAddress>,
        #[doc = "The operating system type required by the containers in the container group."]
        #[serde(rename = "osType")]
        pub os_type: properties::OsType,
        #[doc = "The list of volumes that can be mounted by containers in this container group."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub volumes: Vec<Volume>,
        #[doc = "The instance view of the container group. Only valid in response."]
        #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
        pub instance_view: Option<properties::InstanceView>,
        #[doc = "Container group diagnostic information."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub diagnostics: Option<ContainerGroupDiagnostics>,
    }
    impl Properties {
        pub fn new(containers: Vec<Container>, os_type: properties::OsType) -> Self {
            Self {
                provisioning_state: None,
                containers,
                image_registry_credentials: Vec::new(),
                restart_policy: None,
                ip_address: None,
                os_type,
                volumes: Vec::new(),
                instance_view: None,
                diagnostics: None,
            }
        }
    }
    pub mod properties {
        use super::*;
        #[doc = "Restart policy for all containers within the container group. \n- `Always` Always restart\n- `OnFailure` Restart on failure\n- `Never` Never restart\n"]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum RestartPolicy {
            Always,
            OnFailure,
            Never,
        }
        #[doc = "The operating system type required by the containers in the container group."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum OsType {
            Windows,
            Linux,
        }
        #[doc = "The instance view of the container group. Only valid in response."]
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
        pub struct InstanceView {
            #[doc = "The events of this container group."]
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            pub events: Vec<Event>,
            #[doc = "The state of the container group. Only valid in response."]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub state: Option<String>,
        }
        impl InstanceView {
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}
#[doc = "Container group diagnostic information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupDiagnostics {
    #[doc = "Container group log analytics information."]
    #[serde(rename = "logAnalytics", default, skip_serializing_if = "Option::is_none")]
    pub log_analytics: Option<LogAnalytics>,
}
impl ContainerGroupDiagnostics {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container group list response that contains the container group properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerGroupListResult {
    #[doc = "The list of container groups."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<ContainerGroup>,
    #[doc = "The URI to fetch the next page of container groups."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl azure_core::Continuable for ContainerGroupListResult {
    fn continuation(&self) -> Option<String> {
        self.next_link.clone()
    }
}
impl ContainerGroupListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container Http Get settings, for liveness or readiness probe"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerHttpGet {
    #[doc = "The path to probe."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The port number to probe."]
    pub port: i32,
    #[doc = "The scheme."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<container_http_get::Scheme>,
}
impl ContainerHttpGet {
    pub fn new(port: i32) -> Self {
        Self {
            path: None,
            port,
            scheme: None,
        }
    }
}
pub mod container_http_get {
    use super::*;
    #[doc = "The scheme."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Scheme {
        #[serde(rename = "http")]
        Http,
        #[serde(rename = "https")]
        Https,
    }
}
#[doc = "The port exposed on the container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPort {
    #[doc = "The protocol associated with the port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<container_port::Protocol>,
    #[doc = "The port number exposed within the container group."]
    pub port: i32,
}
impl ContainerPort {
    pub fn new(port: i32) -> Self {
        Self { protocol: None, port }
    }
}
pub mod container_port {
    use super::*;
    #[doc = "The protocol associated with the port."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[doc = "The container probe, for liveness or readiness"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerProbe {
    #[doc = "The container execution command, for liveness or readiness probe"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exec: Option<ContainerExec>,
    #[doc = "The container Http Get settings, for liveness or readiness probe"]
    #[serde(rename = "httpGet", default, skip_serializing_if = "Option::is_none")]
    pub http_get: Option<ContainerHttpGet>,
    #[doc = "The initial delay seconds."]
    #[serde(rename = "initialDelaySeconds", default, skip_serializing_if = "Option::is_none")]
    pub initial_delay_seconds: Option<i32>,
    #[doc = "The period seconds."]
    #[serde(rename = "periodSeconds", default, skip_serializing_if = "Option::is_none")]
    pub period_seconds: Option<i32>,
    #[doc = "The failure threshold."]
    #[serde(rename = "failureThreshold", default, skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[doc = "The success threshold."]
    #[serde(rename = "successThreshold", default, skip_serializing_if = "Option::is_none")]
    pub success_threshold: Option<i32>,
    #[doc = "The timeout seconds."]
    #[serde(rename = "timeoutSeconds", default, skip_serializing_if = "Option::is_none")]
    pub timeout_seconds: Option<i32>,
}
impl ContainerProbe {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The container instance properties."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerProperties {
    #[doc = "The name of the image used to create the container instance."]
    pub image: String,
    #[doc = "The commands to execute within the container instance in exec form."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[doc = "The exposed ports on the container instance."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<ContainerPort>,
    #[doc = "The environment variables to set in the container instance."]
    #[serde(rename = "environmentVariables", default, skip_serializing_if = "Vec::is_empty")]
    pub environment_variables: Vec<EnvironmentVariable>,
    #[doc = "The instance view of the container instance. Only valid in response."]
    #[serde(rename = "instanceView", default, skip_serializing_if = "Option::is_none")]
    pub instance_view: Option<container_properties::InstanceView>,
    #[doc = "The resource requirements."]
    pub resources: ResourceRequirements,
    #[doc = "The volume mounts available to the container instance."]
    #[serde(rename = "volumeMounts", default, skip_serializing_if = "Vec::is_empty")]
    pub volume_mounts: Vec<VolumeMount>,
    #[doc = "The container probe, for liveness or readiness"]
    #[serde(rename = "livenessProbe", default, skip_serializing_if = "Option::is_none")]
    pub liveness_probe: Option<ContainerProbe>,
    #[doc = "The container probe, for liveness or readiness"]
    #[serde(rename = "readinessProbe", default, skip_serializing_if = "Option::is_none")]
    pub readiness_probe: Option<ContainerProbe>,
}
impl ContainerProperties {
    pub fn new(image: String, resources: ResourceRequirements) -> Self {
        Self {
            image,
            command: Vec::new(),
            ports: Vec::new(),
            environment_variables: Vec::new(),
            instance_view: None,
            resources,
            volume_mounts: Vec::new(),
            liveness_probe: None,
            readiness_probe: None,
        }
    }
}
pub mod container_properties {
    use super::*;
    #[doc = "The instance view of the container instance. Only valid in response."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct InstanceView {
        #[doc = "The number of times that the container instance has been restarted."]
        #[serde(rename = "restartCount", default, skip_serializing_if = "Option::is_none")]
        pub restart_count: Option<i64>,
        #[doc = "The container instance state."]
        #[serde(rename = "currentState", default, skip_serializing_if = "Option::is_none")]
        pub current_state: Option<ContainerState>,
        #[doc = "The container instance state."]
        #[serde(rename = "previousState", default, skip_serializing_if = "Option::is_none")]
        pub previous_state: Option<ContainerState>,
        #[doc = "The events of the container instance."]
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub events: Vec<Event>,
    }
    impl InstanceView {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The container instance state."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ContainerState {
    #[doc = "The state of the container instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The date-time when the container instance state started."]
    #[serde(rename = "startTime", default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The container instance exit codes correspond to those from the `docker run` command."]
    #[serde(rename = "exitCode", default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    #[doc = "The date-time when the container instance state finished."]
    #[serde(rename = "finishTime", default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    #[doc = "The human-readable status of the container instance state."]
    #[serde(rename = "detailStatus", default, skip_serializing_if = "Option::is_none")]
    pub detail_status: Option<String>,
}
impl ContainerState {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The empty directory volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct EmptyDirVolume {}
impl EmptyDirVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The environment variable to set within the container instance."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnvironmentVariable {
    #[doc = "The name of the environment variable."]
    pub name: String,
    #[doc = "The value of the environment variable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The value of the secure environment variable."]
    #[serde(rename = "secureValue", default, skip_serializing_if = "Option::is_none")]
    pub secure_value: Option<String>,
}
impl EnvironmentVariable {
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            secure_value: None,
        }
    }
}
#[doc = "A container group or container instance event."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Event {
    #[doc = "The count of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The date-time of the earliest logged event."]
    #[serde(rename = "firstTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub first_timestamp: Option<String>,
    #[doc = "The date-time of the latest logged event."]
    #[serde(rename = "lastTimestamp", default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<String>,
    #[doc = "The event name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The event message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The event type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl Event {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "Represents a volume that is populated with the contents of a git repository"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GitRepoVolume {
    #[doc = "Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<String>,
    #[doc = "Repository URL"]
    pub repository: String,
    #[doc = "Commit hash for the specified revision."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
impl GitRepoVolume {
    pub fn new(repository: String) -> Self {
        Self {
            directory: None,
            repository,
            revision: None,
        }
    }
}
#[doc = "Image registry credential."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageRegistryCredential {
    #[doc = "The Docker image registry server without a protocol such as \"http\" and \"https\"."]
    pub server: String,
    #[doc = "The username for the private registry."]
    pub username: String,
    #[doc = "The password for the private registry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}
impl ImageRegistryCredential {
    pub fn new(server: String, username: String) -> Self {
        Self {
            server,
            username,
            password: None,
        }
    }
}
#[doc = "IP address for the container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpAddress {
    #[doc = "The list of ports exposed on the container group."]
    pub ports: Vec<Port>,
    #[doc = "Specifies if the IP is exposed to the public internet."]
    #[serde(rename = "type")]
    pub type_: ip_address::Type,
    #[doc = "The IP exposed to the public internet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The Dns name label for the IP."]
    #[serde(rename = "dnsNameLabel", default, skip_serializing_if = "Option::is_none")]
    pub dns_name_label: Option<String>,
    #[doc = "The FQDN for the IP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fqdn: Option<String>,
}
impl IpAddress {
    pub fn new(ports: Vec<Port>, type_: ip_address::Type) -> Self {
        Self {
            ports,
            type_,
            ip: None,
            dns_name_label: None,
            fqdn: None,
        }
    }
}
pub mod ip_address {
    use super::*;
    #[doc = "Specifies if the IP is exposed to the public internet."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        Public,
    }
}
#[doc = "Container group log analytics information."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogAnalytics {
    #[doc = "The workspace id for log analytics"]
    #[serde(rename = "workspaceId")]
    pub workspace_id: String,
    #[doc = "The workspace key for log analytics"]
    #[serde(rename = "workspaceKey")]
    pub workspace_key: String,
}
impl LogAnalytics {
    pub fn new(workspace_id: String, workspace_key: String) -> Self {
        Self {
            workspace_id,
            workspace_key,
        }
    }
}
#[doc = "The logs."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Logs {
    #[doc = "The content of the log."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
impl Logs {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "An operation for Azure Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[doc = "The name of the operation."]
    pub name: String,
    #[doc = "The display information of the operation."]
    pub display: operation::Display,
    #[doc = "The intended executor of the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<operation::Origin>,
}
impl Operation {
    pub fn new(name: String, display: operation::Display) -> Self {
        Self {
            name,
            display,
            origin: None,
        }
    }
}
pub mod operation {
    use super::*;
    #[doc = "The display information of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Display {
        #[doc = "The name of the provider of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[doc = "The name of the resource type of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[doc = "The friendly name of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
        #[doc = "The description of the operation."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
    }
    impl Display {
        pub fn new() -> Self {
            Self::default()
        }
    }
    #[doc = "The intended executor of the operation."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Origin {
        User,
        System,
    }
}
#[doc = "The operation list response that contains all operations for Azure Container Instance service."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationListResult {
    #[doc = "The list of operations."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[doc = "The URI to fetch the next page of operations."]
    #[serde(rename = "nextLink", default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
impl OperationListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The port exposed on the container group."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    #[doc = "The protocol associated with the port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<port::Protocol>,
    #[doc = "The port number."]
    pub port: i32,
}
impl Port {
    pub fn new(port: i32) -> Self {
        Self { protocol: None, port }
    }
}
pub mod port {
    use super::*;
    #[doc = "The protocol associated with the port."]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Protocol {
        #[serde(rename = "TCP")]
        Tcp,
        #[serde(rename = "UDP")]
        Udp,
    }
}
#[doc = "The Resource model definition."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Resource {
    #[doc = "The resource id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The resource name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The resource type."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "The resource location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "The resource tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
impl Resource {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource limits."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct ResourceLimits {
    #[doc = "The memory limit in GB of this container instance."]
    #[serde(rename = "memoryInGB", default, skip_serializing_if = "Option::is_none")]
    pub memory_in_gb: Option<f64>,
    #[doc = "The CPU limit of this container instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
}
impl ResourceLimits {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The resource requests."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequests {
    #[doc = "The memory request in GB of this container instance."]
    #[serde(rename = "memoryInGB")]
    pub memory_in_gb: f64,
    #[doc = "The CPU request of this container instance."]
    pub cpu: f64,
}
impl ResourceRequests {
    pub fn new(memory_in_gb: f64, cpu: f64) -> Self {
        Self { memory_in_gb, cpu }
    }
}
#[doc = "The resource requirements."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirements {
    #[doc = "The resource requests."]
    pub requests: ResourceRequests,
    #[doc = "The resource limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<ResourceLimits>,
}
impl ResourceRequirements {
    pub fn new(requests: ResourceRequests) -> Self {
        Self { requests, limits: None }
    }
}
#[doc = "The secret volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct SecretVolume {}
impl SecretVolume {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "A single usage result"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Usage {
    #[doc = "Unit of the usage result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[doc = "The current usage of the resource"]
    #[serde(rename = "currentValue", default, skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[doc = "The maximum permitted usage of the resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[doc = "The name object of the resource"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<usage::Name>,
}
impl Usage {
    pub fn new() -> Self {
        Self::default()
    }
}
pub mod usage {
    use super::*;
    #[doc = "The name object of the resource"]
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
    pub struct Name {
        #[doc = "The name of the resource"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
        #[doc = "The localized name of the resource"]
        #[serde(rename = "localizedValue", default, skip_serializing_if = "Option::is_none")]
        pub localized_value: Option<String>,
    }
    impl Name {
        pub fn new() -> Self {
            Self::default()
        }
    }
}
#[doc = "The response containing the usage data"]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct UsageListResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
impl UsageListResult {
    pub fn new() -> Self {
        Self::default()
    }
}
#[doc = "The properties of the volume."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    #[doc = "The name of the volume."]
    pub name: String,
    #[doc = "The properties of the Azure File volume. Azure File shares are mounted as volumes."]
    #[serde(rename = "azureFile", default, skip_serializing_if = "Option::is_none")]
    pub azure_file: Option<AzureFileVolume>,
    #[doc = "The empty directory volume."]
    #[serde(rename = "emptyDir", default, skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EmptyDirVolume>,
    #[doc = "The secret volume."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<SecretVolume>,
    #[doc = "Represents a volume that is populated with the contents of a git repository"]
    #[serde(rename = "gitRepo", default, skip_serializing_if = "Option::is_none")]
    pub git_repo: Option<GitRepoVolume>,
}
impl Volume {
    pub fn new(name: String) -> Self {
        Self {
            name,
            azure_file: None,
            empty_dir: None,
            secret: None,
            git_repo: None,
        }
    }
}
#[doc = "The properties of the volume mount."]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeMount {
    #[doc = "The name of the volume mount."]
    pub name: String,
    #[doc = "The path within the container where the volume should be mounted. Must not contain colon (:)."]
    #[serde(rename = "mountPath")]
    pub mount_path: String,
    #[doc = "The flag indicating whether the volume mount is read-only."]
    #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}
impl VolumeMount {
    pub fn new(name: String, mount_path: String) -> Self {
        Self {
            name,
            mount_path,
            read_only: None,
        }
    }
}
