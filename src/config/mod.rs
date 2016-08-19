//! Config for specific platform's package.

pub mod core;
pub mod meta;
pub mod arch;

pub use self::core::*;
pub use self::meta::*;
pub use self::arch::*;
