use std::collections::HashMap;

use proc_macro2::Ident;
use quote::format_ident;
use svd_parser::{Cluster, DimElement, Register, RegisterCluster, RegisterProperties};

use crate::cluster::_Cluster;
use crate::register::_Register;

pub(super) fn build_ident(name: String) -> Ident {
    let name = format_ident!("{}", name);
    if !accept_as_ident(&name) {
        format_ident!("_{}", name)
    } else {
        name
    }
}

fn accept_as_ident(ident: &Ident) -> bool {
    match ident.to_string().as_str() {
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
    if dim.dim > 0 {
        dim.dim_index
            .as_ref()
            .expect("Indexes are expected to be specified")
            .iter()
            .map(|index| name.replace("%s", index).to_lowercase())
            .collect()
    } else {
        vec![name.to_lowercase()]
    }
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
) -> (Vec<_Cluster<'a>>, Vec<_Register<'a>>) {
    let mut child_clusters = Vec::new();
    let mut child_registers = Vec::new();
    for child in children {
        match child {
            RegisterCluster::Cluster(cluster) => {
                child_clusters.extend(_Cluster::build_all(
                    cluster, &clusters, &registers, defaults,
                ));
            }
            RegisterCluster::Register(register) => {
                child_registers.extend(_Register::build_all(register, &registers, defaults));
            }
        }
    }
    (child_clusters, child_registers)
}
