use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use indoc::writedoc;
use proc_macro2::{Ident, Span};
use svd_parser::{Cluster, Register, RegisterProperties};

use crate::register::_Register;
use crate::utils::{array_names, build_children, build_ident, merge_defaults};

#[derive(Clone)]
pub(super) struct _Cluster<'a> {
    pub(super) name: Ident,
    description: &'a Option<String>,
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
    ) -> Vec<_Cluster<'a>> {
        let _cluster = _Cluster::build(cluster, clusters, registers, defaults);
        match cluster {
            Cluster::Single(_) => vec![_Cluster {
                name: build_ident(cluster.name.to_lowercase()),
                .._cluster
            }],
            Cluster::Array(_, dim) => {
                let mut clusters = Vec::new();
                let mut offset = _cluster.offset;
                for name in array_names(&cluster.name, dim) {
                    clusters.push(_Cluster {
                        name: build_ident(name),
                        description: _cluster.description,
                        offset,
                        clusters: _cluster.clusters.clone(),
                        registers: _cluster.registers.clone(),
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
    ) -> _Cluster<'a> {
        let defaults = merge_defaults(cluster.default_register_properties(clusters), defaults);
        let (clusters, registers) =
            build_children(&cluster.children, clusters, registers, defaults);
        _Cluster {
            name: Ident::new("_", Span::call_site()), //placeholder
            description: &cluster.description,
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
        let mut children = Vec::with_capacity(self.clusters.len() + self.registers.len());
        for cluster in &self.clusters {
            children.push(&cluster.name);
        }
        for register in &self.registers {
            children.push(&register.name);
        }
        for child in children {
            writedoc!(
                f,
                "
                mod {module};
                pub use {module}::*;
                ",
                module = child
            )?;
        }
        Ok(())
    }
}

trait DerivedCluster<'a> {
    fn default_register_properties(
        &'a self,
        others: &HashMap<String, &'a Cluster>,
    ) -> RegisterProperties;
}

impl<'a> DerivedCluster<'a> for Cluster {
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
}
