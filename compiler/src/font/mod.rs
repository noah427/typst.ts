pub use typst_ts_core::font::FontSlot;

#[cfg(feature = "system")]
pub mod system;

#[cfg(feature = "web-render")]
pub mod web;

pub(crate) mod info;
