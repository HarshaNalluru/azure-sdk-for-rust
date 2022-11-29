#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "no-default-tag")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-01_15_0")]
pub mod package_2022_01_15_0;
#[cfg(all(feature = "package-2022-01_15_0", not(feature = "no-default-tag")))]
pub use package_2022_01_15_0::*;
#[cfg(feature = "package-2021-06_14_0")]
pub mod package_2021_06_14_0;
#[cfg(all(feature = "package-2021-06_14_0", not(feature = "no-default-tag")))]
pub use package_2021_06_14_0::*;
#[cfg(feature = "package-2020-09_12_0")]
pub mod package_2020_09_12_0;
#[cfg(all(feature = "package-2020-09_12_0", not(feature = "no-default-tag")))]
pub use package_2020_09_12_0::*;
#[cfg(feature = "package-2020-03_11_0")]
pub mod package_2020_03_11_0;
#[cfg(all(feature = "package-2020-03_11_0", not(feature = "no-default-tag")))]
pub use package_2020_03_11_0::*;
