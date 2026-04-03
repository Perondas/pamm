use crate::io::serialization::versioning::versioned::Versioned;
use crate::io::serialization::versioning::versioned_types::versioned_optionals_config::VersionedOptionalsConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type OptionalName = String;
pub type PackName = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionalsConfig {
    pub enabled_optionals: HashMap<OptionalName, Vec<PackName>>,
}

impl Versioned for OptionalsConfig {
    type WrapperType = VersionedOptionalsConfig;
}
