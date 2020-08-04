pub mod std;

#[cfg(feature = "fast_fmt")]
pub mod fast_fmt;

#[cfg(not(feature = "fast_fmt"))]
pub use self::std::*;

#[cfg(feature = "fast_fmt")]
pub use self::fast_fmt::*;
