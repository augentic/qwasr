//! Installing the loader over a deployment's declared locations — the one
//! place the built-in acquirers are named concretely.

use std::sync::Arc;

use omnia_core::{Location, Runtime};

use crate::loader::Plugins;
use crate::path::PathMounts;
use crate::registry::RegistryClient;
use crate::source::{PathSource, RegistrySource};

impl Plugins {
    /// Install the loader capability over the deployment's declared
    /// locations ([`Runtime::plugin_locations`]): every path entry folds, in
    /// declaration order, into one [`PathMounts`] filling the path slot, the
    /// registry entry into a cacheless [`RegistryClient`] filling the
    /// registry slot. A deployment declaring no locations installs nothing,
    /// so a load refuses as loader misconfiguration.
    ///
    /// # Errors
    ///
    /// Returns an error if a path location cannot be opened or the capability
    /// is already installed.
    pub fn install_declared<B>(runtime: &Runtime<B>) -> anyhow::Result<()>
    where
        B: Clone + Send + Sync + 'static,
    {
        let locations = runtime.plugin_locations();
        if locations.is_empty() {
            return Ok(());
        }
        let paths: Vec<(&str, &std::path::Path)> = locations
            .iter()
            .filter_map(|location| match location {
                Location::Path { name, path } => Some((name.as_str(), path.as_path())),
                Location::Registry { .. } => None,
            })
            .collect();
        let path: Option<Arc<dyn PathSource>> =
            if paths.is_empty() { None } else { Some(Arc::new(PathMounts::new(paths)?)) };
        let registry: Option<Arc<dyn RegistrySource>> =
            locations.iter().find_map(|location| match location {
                Location::Registry { registry } => {
                    Some(Arc::new(RegistryClient::new(registry.as_str())) as Arc<dyn RegistrySource>)
                }
                Location::Path { .. } => None,
            });
        Self::install(runtime, registry, path)
    }
}
