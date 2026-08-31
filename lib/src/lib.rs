#[cfg(not(any(feature = "client", feature = "server")))]
compile_error!(
    "pamm_lib needs at least one of the `client` or `server` features enabled. \
     There are no default features, so building or testing the crate on its own \
     needs them named explicitly, e.g. `--features client,server`."
);

pub mod handle;
pub mod io;
pub mod models;
pub mod util;
