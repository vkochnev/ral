use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(super) struct DeviceOverrides {
    pub(super) name: Option<String>,
    pub(super) description: Option<String>,
    pub(super) peripherals: Option<HashMap<String, PeripheralOverrides>>,
}

#[derive(Deserialize)]
pub(super) struct PeripheralOverrides {
    pub(super) name: Option<String>,
    pub(super) description: Option<String>,
    pub(super) features: Option<Vec<String>>,
    pub(super) clusters: Option<HashMap<String, ClusterOverrides>>,
    pub(super) registers: Option<HashMap<String, RegisterOverrides>>,
}

#[derive(Deserialize)]
pub(super) struct ClusterOverrides {
    pub(super) name: Option<String>,
    pub(super) description: Option<String>,
    pub(super) features: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub(super) struct RegisterOverrides {
    pub(super) name: Option<String>,
    pub(super) description: Option<String>,
    pub(super) features: Option<Vec<String>>,
    pub(super) uses: Option<Vec<String>>,
    pub(super) fields: Option<HashMap<String, FieldOverrides>>,
}

#[derive(Deserialize)]
pub(super) struct FieldOverrides {
    pub(super) name: Option<String>,
    pub(super) description: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub(super) ty: Option<String>,
}
