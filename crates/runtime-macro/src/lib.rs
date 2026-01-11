mod expand;
mod runtime;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Generates the runtime infrastructure based on the configuration.
///
/// # Example
///
/// ```ignore
/// yetti::runtime!({
///     wasi_http: WasiHttp,
///     wasi_otel: DefaultOtel,
///     wasi_blobstore: MongoDb,
/// });
/// ```
#[proc_macro]
pub fn runtime(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as runtime::Config);
    match expand::expand(&parsed) {
        Ok(ts) => ts.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
