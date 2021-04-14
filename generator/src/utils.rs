use std::collections::HashMap;

use svd_parser::{Cluster, DimElement, Register, RegisterCluster, RegisterProperties};

use crate::cluster::_Cluster;
use crate::overrides::{ClusterOverrides, RegisterOverrides};
use crate::register::_Register;
use std::ops::Range;

pub(super) fn build_ident(name: &String) -> String {
    let name = name.to_lowercase();
    if !accept_as_ident(&name) {
        format!("_{}", name)
    } else {
        name
    }
}

/// Extracted from [syn](https://crates.io/crates/syn) crate
fn accept_as_ident(ident: &String) -> bool {
    match ident.as_str() {
        "_" |
        // Based on https://doc.rust-lang.org/grammar.html#keywords
        // and https://github.com/rust-lang/rfcs/blob/master/text/2421-unreservations-2018.md
        // and https://github.com/rust-lang/rfcs/blob/master/text/2420-unreserve-proc.md
        "abstract" | "as" | "become" | "box" | "break" | "const" | "continue" |
        "crate" | "do" | "else" | "enum" | "extern" | "false" | "final" | "fn" |
        "for" | "if" | "impl" | "in" | "let" | "loop" | "macro" | "match" |
        "mod" | "move" | "mut" | "override" | "priv" | "pub" | "ref" |
        "return" | "Self" | "self" | "static" | "struct" | "super" | "trait" |
        "true" | "type" | "typeof" | "unsafe" | "unsized" | "use" | "virtual" |
        "where" | "while" | "yield" => false,
        _ => true,
    }
}

pub(super) fn indent(string: String, indent_level: usize) -> String {
    let indent = " ".repeat(indent_level * 4);
    string
        .split("\n")
        .map(|line| {
            if line.is_empty() || line.eq("\r") {
                String::from(line)
            } else {
                format!("{}{}", &indent, line)
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub(super) fn array_names(name: &String, dim: &DimElement) -> Vec<String> {
    dim.dim_index
        .clone()
        .unwrap_or_else(|| {
            Range {
                start: 0,
                end: dim.dim,
            }
            .map(|index| index.to_string())
            .collect()
        })
        .iter()
        .map(|index| name.replace("[%s]", index).to_lowercase())
        .collect()
}

pub(super) fn merge_defaults(
    current: RegisterProperties,
    parent: RegisterProperties,
) -> RegisterProperties {
    let mut properties = RegisterProperties::default();
    properties.size = current.size.or(parent.size);
    properties.reset_mask = current.reset_mask.or(parent.reset_mask);
    properties.reset_value = current.reset_value.or(parent.reset_value);
    properties.access = current.access.or(parent.access);
    properties
}

pub(super) fn build_children<'a>(
    children: &'a Vec<RegisterCluster>,
    clusters: &HashMap<String, &'a Cluster>,
    registers: &HashMap<String, &'a Register>,
    defaults: RegisterProperties,
    cluster_overrides: Option<&'a HashMap<String, ClusterOverrides>>,
    register_overrides: Option<&'a HashMap<String, RegisterOverrides>>,
) -> (Vec<_Cluster<'a>>, Vec<_Register<'a>>) {
    let mut child_clusters = Vec::new();
    let mut child_registers = Vec::new();
    for child in children {
        match child {
            RegisterCluster::Cluster(cluster) => {
                child_clusters.extend(_Cluster::build_all(
                    cluster,
                    &clusters,
                    &registers,
                    defaults,
                    cluster_overrides,
                    register_overrides,
                ));
            }
            RegisterCluster::Register(register) => {
                child_registers.extend(_Register::build_all(
                    register,
                    &registers,
                    defaults,
                    register_overrides,
                ));
            }
        }
    }
    (child_clusters, child_registers)
}

macro_rules! overrides {
    ($s: ident, $o: ident) => {
        $o.and_then(|overrides| {
            let derived = $s
                .derived_from
                .as_ref()
                .and_then(|name| overrides.get(name));
            overrides.get(&$s.name).or(derived)
        })
    };
}

macro_rules! features_cfg {
    ($i:ident) => {
        $i.features.map(|features| {
            let predicates = features
                .iter()
                .map(|feature| format!("feature = \"{}\"", feature))
                .collect::<Vec<String>>()
                .join(", ");
            format!("#[cfg(any({predicates}))]\n", predicates = predicates)
        })
    };
}

macro_rules! write_children {
    ($s: ident, $f: ident) => {
        let mut children = Vec::with_capacity($s.clusters.len() + $s.registers.len());
        for cluster in &$s.clusters {
            children.push((&cluster.name, features_cfg!(cluster)));
        }
        for register in &$s.registers {
            children.push((&register.name, features_cfg!(register)));
        }
        for child in children {
            let (module, cfg) = child;
            if let Some(features_cfg) = &cfg {
                write!($f, "{}", features_cfg)?;
            }
            write!($f, "mod {module};\n", module = module)?;
            if let Some(features_cfg) = &cfg {
                write!($f, "{}", features_cfg)?;
            }
            write!($f, "pub use {module}::*;\n", module = module)?;
        }
    };
}
