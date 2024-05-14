pub mod err;

#[cfg(feature = "pdf")]
pub mod pdf;

#[cfg(feature = "web")]
pub mod template;

#[cfg(feature = "web")]
pub mod resources;

#[cfg(feature = "web")]
pub mod docs;
