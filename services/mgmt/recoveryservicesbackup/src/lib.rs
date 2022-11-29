#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-09-preview")]
pub mod package_2022_09_preview;
#[cfg(all(feature = "package-2022-09-preview", not(feature = "no-default-tag")))]
pub use package_2022_09_preview::*;
#[cfg(feature = "package-2022-06-01-preview")]
pub mod package_2022_06_01_preview;
#[cfg(all(feature = "package-2022-06-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_06_01_preview::*;
#[cfg(feature = "package-2022-04")]
pub mod package_2022_04;
#[cfg(all(feature = "package-2022-04", not(feature = "no-default-tag")))]
pub use package_2022_04::*;
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::*;
#[cfg(feature = "package-2022-02")]
pub mod package_2022_02;
#[cfg(all(feature = "package-2022-02", not(feature = "no-default-tag")))]
pub use package_2022_02::*;
