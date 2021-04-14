use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use svd_parser::{Cluster, Register, RegisterProperties};

use crate::overrides::{ClusterOverrides, RegisterOverrides};
use crate::register::_Register;
use crate::utils::{array_names, build_children, build_ident, merge_defaults};

#[derive(Clone)]
pub(super) struct _Cluster<'a> {
    pub(super) name: String,
    description: Option<&'a String>,
    pub(super) features: Option<&'a Vec<String>>,
    offset: u64,
    pub(super) clusters: Vec<_Cluster<'a>>,
    pub(super) registers: Vec<_Register<'a>>,
}

impl<'a> _Cluster<'a> {
    pub(super) fn build_all(
        cluster: &'a Cluster,
        clusters: &HashMap<String, &'a Cluster>,
        registers: &HashMap<String, &'a Register>,
        defaults: RegisterProperties,
        cluster_overrides: Option<&'a HashMap<String, ClusterOverrides>>,
        register_overrides: Option<&'a HashMap<String, RegisterOverrides>>,
    ) -> Vec<_Cluster<'a>> {
        let _cluster = _Cluster::build(
            cluster,
            clusters,
            registers,
            defaults,
            cluster_overrides,
            register_overrides,
        );
        match cluster {
            Cluster::Single(_) => vec![_cluster],
            Cluster::Array(_, dim) => {
                let mut clusters = Vec::new();
                let mut offset = _cluster.offset;
                for name in array_names(&_cluster.name, dim) {
                    clusters.push(_Cluster {
                        name,
                        offset,
                        clusters: _cluster.clusters.clone(),
                        registers: _cluster.registers.clone(),
                        .._cluster
                    });
                    offset = offset + dim.dim_increment as u64;
                }
                clusters
            }
        }
    }

    pub(super) fn build(
        cluster: &'a Cluster,
        clusters: &HashMap<String, &'a Cluster>,
        registers: &HashMap<String, &'a Register>,
        defaults: RegisterProperties,
        cluster_overrides: Option<&'a HashMap<String, ClusterOverrides>>,
        register_overrides: Option<&'a HashMap<String, RegisterOverrides>>,
    ) -> _Cluster<'a> {
        let defaults = merge_defaults(cluster.default_register_properties(clusters), defaults);
        let (clusters, registers) = build_children(
            &cluster.children,
            clusters,
            registers,
            defaults,
            cluster_overrides,
            register_overrides,
        );
        let overrides = cluster.overrides(cluster_overrides);
        _Cluster {
            name: cluster.name(overrides),
            description: cluster.description(overrides),
            features: overrides.and_then(|overrides| overrides.features.as_ref()),
            offset: cluster.address_offset as u64,
            clusters,
            registers,
        }
    }
}

impl<'a> Display for _Cluster<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(description) = self.description {
            write!(
                f,
                "#![doc = \"{description}\"]\n\n",
                description = description
            )?;
        }
        write!(
            f,
            "const BASE_ADDRESS: usize = super::BASE_ADDRESS + {offset:#X};\n\n",
            offset = self.offset
        )?;
        write_children!(self, f);
        Ok(())
    }
}

trait DerivedCluster<'a> {
    fn name(&self, overrides: Option<&'a ClusterOverrides>) -> String;

    fn description(&'a self, overrides: Option<&'a ClusterOverrides>) -> Option<&'a String>;

    fn default_register_properties(
        &'a self,
        others: &HashMap<String, &'a Cluster>,
    ) -> RegisterProperties;

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, ClusterOverrides>>,
    ) -> Option<&'a ClusterOverrides>;
}

impl<'a> DerivedCluster<'a> for Cluster {
    fn name(&self, overrides: Option<&'a ClusterOverrides>) -> String {
        build_ident(
            overrides
                .and_then(|overrides| overrides.name.as_ref())
                .unwrap_or(&self.name),
        )
    }

    fn description(&'a self, overrides: Option<&'a ClusterOverrides>) -> Option<&'a String> {
        overrides
            .and_then(|overrides| overrides.description.as_ref())
            .or(self.description.as_ref())
    }

    fn default_register_properties(
        &'a self,
        others: &HashMap<String, &'a Cluster>,
    ) -> RegisterProperties {
        self.derived_from
            .as_ref()
            .and_then(|name| others.get(name))
            .map(|&c| c.default_register_properties(others))
            .map(|derived| merge_defaults(self.default_register_properties, derived))
            .unwrap_or(self.default_register_properties)
    }

    fn overrides(
        &'a self,
        overrides: Option<&'a HashMap<String, ClusterOverrides>>,
    ) -> Option<&'a ClusterOverrides> {
        overrides!(self, overrides)
    }
}
