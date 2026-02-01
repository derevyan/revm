use super::KzgSettings;
use alloc::sync::Arc;

/// KZG Settings that allow us to specify a custom trusted setup.
/// or use hardcoded default settings.
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub enum EnvKzgSettings {
    /// Default mainnet trusted setup
    #[default]
    Default,
    /// Custom trusted setup.
    Custom(Arc<c_kzg::KzgSettings>),
}

impl EnvKzgSettings {
    /// Return set KZG settings.
    ///
    /// In will initialize the default settings if it is not already loaded.
    pub fn get(&self) -> &KzgSettings {
        match self {
            Self::Default => c_kzg::ethereum_kzg_settings(0),
            Self::Custom(settings) => settings,
        }
    }
}
