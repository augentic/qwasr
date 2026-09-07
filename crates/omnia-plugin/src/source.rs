//! The acquisition seam: where one load's bytes come from and the two
//! per-kind acquirer slots [`Plugins`](crate::Plugins) fills.

use futures::future::BoxFuture;

use crate::error::LoadError;

/// Where one load's component bytes come from, resolved against the
/// deployment's declared [`Location`](omnia_core::Location)s — the host mirror
/// of the `omnia:plugins/loader` `location` variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// A package registry; `None` selects the acquirer's default.
    Registry(Option<String>),
    /// A location-relative component path.
    Path(String),
}

/// Path acquisition policy — the path slot of [`Plugins`](crate::Plugins).
pub trait PathSource: Send + Sync + 'static {
    /// Produce the raw component bytes at the location-relative `path`,
    /// split by remedy: [`LoadError::Refused`] for a path no location
    /// serves, never for a read failure a retry might clear
    /// ([`LoadError::Unavailable`]).
    fn acquire<'a>(&'a self, path: &'a str) -> BoxFuture<'a, Result<Vec<u8>, LoadError>>;
}

/// Registry acquisition policy — the registry slot of
/// [`Plugins`](crate::Plugins).
pub trait RegistrySource: Send + Sync + 'static {
    /// Produce the raw component bytes for `package` from `registry`
    /// (`None` selects the acquirer's default endpoint), split by remedy:
    /// [`LoadError::Refused`] for an authoritative "no", never for a
    /// source failure a retry might clear ([`LoadError::Unavailable`]).
    fn acquire<'a>(
        &'a self, package: &'a str, registry: Option<&'a str>,
    ) -> BoxFuture<'a, Result<Vec<u8>, LoadError>>;
}
