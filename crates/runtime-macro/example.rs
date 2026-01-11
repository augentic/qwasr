
use wasi_http::{WasiHttp, HttpDefault};
use wasi_blobstore::{WasiBlobstore, BlobstoreDefault};

warp::runtime!({
    main: true,
    hosts: {
        WasiHttp: HttpDefault,
        WasiBlobstore: BlobstoreDefault,
    }
});

