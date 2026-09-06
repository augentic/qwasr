//! The privileged runtime half the loader drives, erased of the deployment's
//! backend type.

use futures::FutureExt as _;
use futures::future::BoxFuture;
use omnia_core::{AdmitError, GuestId, WeakRuntime};

use crate::error::LoadError;

/// The registry record and the admission seam — erased of the deployment's
/// backend type so [`Plugins`](crate::Plugins) can live in the runtime's
/// extensions.
pub trait Admission: Send + Sync + 'static {
    /// The registration state of `id`, with any recorded digest.
    fn registration(&self, id: &GuestId) -> Result<Registration, LoadError>;

    /// Admit raw wasm bytes as the late guest `id`.
    fn admit(&self, id: GuestId, bytes: Vec<u8>) -> BoxFuture<'static, Result<(), AdmitError>>;
}

// Weak: a strong handle would cycle through the extension.
impl<B: Clone + Send + Sync + 'static> Admission for WeakRuntime<B> {
    fn registration(&self, id: &GuestId) -> Result<Registration, LoadError> {
        let runtime = self
            .upgrade()
            .ok_or_else(|| LoadError::Internal("the runtime has shut down".to_owned()))?;
        Ok(runtime.registry().get(id).map_or(Registration::Absent, |guest| {
            Registration::Active(guest.digest().map(str::to_owned))
        }))
    }

    fn admit(&self, id: GuestId, bytes: Vec<u8>) -> BoxFuture<'static, Result<(), AdmitError>> {
        let weak = self.clone();
        async move {
            let Some(runtime) = weak.upgrade() else {
                return Err(AdmitError::Internal("the runtime has shut down".to_owned()));
            };
            runtime.admit(id, bytes).await
        }
        .boxed()
    }
}

pub enum Registration {
    Absent,
    Active(Option<String>),
}

impl Registration {
    /// The recorded digest, when active with one.
    pub fn digest(self) -> Option<String> {
        match self {
            Self::Active(digest) => digest,
            Self::Absent => None,
        }
    }
}
