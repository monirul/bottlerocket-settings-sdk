//! Modeled types for creating Kubernetes settings.
mod de;

use self::de::deserialize_node_taints;
use bottlerocket_model_derive::model;
#[cfg(feature = "nvidia-device-plugin")]
use bottlerocket_modeled_types::K8sDevicePluginsSettings;
use bottlerocket_modeled_types::KubernetesEvictionKey;
use bottlerocket_modeled_types::KubernetesMemoryManagerPolicy;
use bottlerocket_modeled_types::KubernetesMemoryReservation;
use bottlerocket_modeled_types::NonNegativeInteger;
use bottlerocket_modeled_types::{
    KubernetesCPUManagerPolicyOption, KubernetesHostnameOverrideSource,
};
use std::collections::HashMap;
use std::net::IpAddr;

use bottlerocket_modeled_types::{
    CpuManagerPolicy, CredentialProvider, DNSDomain, Identifier, IntegerPercent, KernelCpuSetValue,
    KubernetesAuthenticationMode, KubernetesBootstrapToken, KubernetesCloudProvider,
    KubernetesClusterDnsIp, KubernetesClusterName, KubernetesDurationValue, KubernetesLabelKey,
    KubernetesLabelValue, KubernetesQuantityValue, KubernetesReservedResourceKey,
    KubernetesTaintValue, KubernetesThresholdValue, SingleLineString, TopologyManagerPolicy,
    TopologyManagerScope, Url, ValidBase64, ValidLinuxHostname,
};

// Kubernetes static pod manifest settings
#[model]
pub struct StaticPod {
    enabled: bool,
    manifest: ValidBase64,
}

// Kubernetes related settings. The dynamic settings are retrieved from
// IMDS via Sundog's child "Pluto".
#[model]
pub struct KubernetesSettingsV1 {
    // Settings that must be specified via user data or through API requests.  Not all settings are
    // useful for all modes. For example, in standalone mode the user does not need to specify any
    // cluster information, and the bootstrap token is only needed for TLS authentication mode.
    cluster_name: KubernetesClusterName,
    cluster_certificate: ValidBase64,
    api_server: Url,
    node_labels: HashMap<KubernetesLabelKey, KubernetesLabelValue>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_node_taints"
    )]
    node_taints: HashMap<KubernetesLabelKey, Vec<KubernetesTaintValue>>,
    static_pods: HashMap<Identifier, StaticPod>,
    authentication_mode: KubernetesAuthenticationMode,
    bootstrap_token: KubernetesBootstrapToken,
    standalone_mode: bool,
    eviction_hard: HashMap<KubernetesEvictionKey, KubernetesThresholdValue>,
    eviction_soft: HashMap<KubernetesEvictionKey, KubernetesThresholdValue>,
    eviction_soft_grace_period: HashMap<KubernetesEvictionKey, KubernetesDurationValue>,
    eviction_max_pod_grace_period: NonNegativeInteger,
    kube_reserved: HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue>,
    system_reserved: HashMap<KubernetesReservedResourceKey, KubernetesQuantityValue>,
    allowed_unsafe_sysctls: Vec<SingleLineString>,
    server_tls_bootstrap: bool,
    cloud_provider: KubernetesCloudProvider,
    registry_qps: i32,
    registry_burst: i32,
    event_qps: i32,
    event_burst: i32,
    kube_api_qps: i32,
    kube_api_burst: i32,
    container_log_max_size: KubernetesQuantityValue,
    container_log_max_files: i32,
    cpu_cfs_quota_enforced: bool,
    cpu_manager_policy: CpuManagerPolicy,
    cpu_manager_reconcile_period: KubernetesDurationValue,
    cpu_manager_policy_options: Vec<KubernetesCPUManagerPolicyOption>,
    topology_manager_scope: TopologyManagerScope,
    topology_manager_policy: TopologyManagerPolicy,
    pod_pids_limit: i64,
    image_gc_high_threshold_percent: IntegerPercent,
    image_gc_low_threshold_percent: IntegerPercent,
    provider_id: Url,
    log_level: u8,
    credential_providers: HashMap<Identifier, CredentialProvider>,
    server_certificate: ValidBase64,
    server_key: ValidBase64,
    shutdown_grace_period: KubernetesDurationValue,
    shutdown_grace_period_for_critical_pods: KubernetesDurationValue,
    memory_manager_reserved_memory: HashMap<Identifier, KubernetesMemoryReservation>,
    memory_manager_policy: KubernetesMemoryManagerPolicy,
    reserved_cpus: KernelCpuSetValue,

    // Settings where we generate a value based on the runtime environment.  The user can specify a
    // value to override the generated one, but typically would not.
    max_pods: u32,
    cluster_dns_ip: KubernetesClusterDnsIp,
    cluster_domain: DNSDomain,
    node_ip: IpAddr,
    pod_infra_container_image: SingleLineString,
    // Generated in `aws-k8s-1.26*` variants only
    hostname_override: ValidLinuxHostname,
    hostname_override_source: KubernetesHostnameOverrideSource,
    // Generated in `k8s-1.25+` variants only
    seccomp_default: bool,
    #[cfg(feature = "nvidia-device-plugin")]
    device_plugins: K8sDevicePluginsSettings,
}
