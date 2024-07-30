//! Settings related to auto scaling groups.
use bottlerocket_model_derive::model;
use bottlerocket_settings_sdk::{GenerateResult, SettingsModel};
use std::convert::Infallible;

/// NvidiaRuntimeSettingsV1 contains the settings for a container runtime settings for Nvidia gpu.
#[model(impl_default = true)]
pub struct NvidiaContainerRuntimeSettingsV1 {
    visible_devices_as_volume_mounts: bool,
    visible_devices_envvar_when_unprivileged: bool,
}

type Result<T> = std::result::Result<T, Infallible>;

impl SettingsModel for NvidiaContainerRuntimeSettingsV1 {
    type PartialKind = Self;
    type ErrorKind = Infallible;

    fn get_version() -> &'static str {
        "v1"
    }

    fn set(_current_value: Option<Self>, _target: Self) -> Result<()> {
        // Set anything that can be parsed as NvidiaContainerRuntimeSettingsV1.
        Ok(())
    }

    fn generate(
        existing_partial: Option<Self::PartialKind>,
        _dependent_settings: Option<serde_json::Value>,
    ) -> Result<GenerateResult<Self::PartialKind, Self>> {
        Ok(GenerateResult::Complete(
            existing_partial.unwrap_or_default(),
        ))
    }

    fn validate(_value: Self, _validated_settings: Option<serde_json::Value>) -> Result<()> {
        // NvidiaContainerRuntimeSettingsV1 is validated during deserialization.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_nvidia_container_runtime() {
        assert_eq!(
            NvidiaContainerRuntimeSettingsV1::generate(None, None).unwrap(),
            GenerateResult::Complete(NvidiaContainerRuntimeSettingsV1 {
                visible_devices_as_volume_mounts: None,
                visible_devices_envvar_when_unprivileged: None,
            })
        )
    }

    #[test]
    fn test_serde_nvidia_container_runtime() {
        let test_json = r#"{"visible-devices-as-volume-mounts":true,"visible-devices-envvar-when-unprivileged":true}"#;

        let nvidia_runtime: NvidiaContainerRuntimeSettingsV1 =
            serde_json::from_str(test_json).unwrap();
        assert_eq!(
            nvidia_runtime,
            NvidiaContainerRuntimeSettingsV1 {
                visible_devices_as_volume_mounts: Some(true),
                visible_devices_envvar_when_unprivileged: Some(true),
            }
        );

        let results = serde_json::to_string(&nvidia_runtime).unwrap();
        assert_eq!(results, test_json);
    }
}
