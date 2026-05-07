pub mod get_mut_system_metadata;

#[cfg(feature = "blocking-processor")]
pub mod blocking_processor;

#[cfg(feature = "non-blocking-processor")]
pub mod non_blocking_processor;

pub mod prelude {
    pub use super::{
        get_mut_system_metadata::{
            get_mut_system_metadata
        }
    };

    #[cfg(feature = "blocking-processor")]
    pub use super::{
        blocking_processor::{

        }
    };

    #[cfg(feature = "non-blocking-processor")]
    pub use super::{
        non_blocking_processor::{

        }
    };
}