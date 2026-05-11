pub mod get_mut_system_metadata;
pub mod get_system_metadata;
pub mod get_links;
pub mod get_threadpool;
pub mod get_runtime;
pub mod get_system_criteria_registry;
pub mod get_system_event_registry;
pub mod get_mut_active_system_registry;

#[cfg(feature = "blocking-processor")]
pub mod blocking_processor;

#[cfg(feature = "non-blocking-processor")]
pub mod non_blocking_processor;

pub mod prelude {
    pub use super::{
        get_mut_system_metadata::{
            get_mut_system_metadata
        },
        get_system_metadata::{
            get_system_metadata,
        },
        get_links::{
            LINKS_ACCESS_BUILDER,
            LINKS_RESOURCE_ID,
            Links,
            get_links,
        },
        get_threadpool::{
            THREAD_POOL_ACCESS_BUILDER,
            THREAD_POOL_RESOURCE_ID,
            ThreadPool,
            get_threadpool
        },
        get_runtime::{
            RUNTIME_ACCESS_BUILDER,
            RUNTIME_RESOURCE_ID,
            Runtime,
            get_runtime
        },
        get_system_criteria_registry::{
            SYSTEM_CRITERIA_REGISTRY_ACCESS_BUILDER,
            SYSTEM_CRITERIA_REGISTRY_RESOURCE_ID,
            SystemCriteriaRegistry,
            get_system_criteria_registry,
            system_criteria::{
                SystemCriteria
            }
        },
        get_system_event_registry::{
            SYSTEM_EVENT_REGISTRY_ACCESS_BUILDER,
            SYSTEM_EVENT_REGISTRY_RESOURCE_ID,
            SystemEventRegistry,
            get_system_event_registry
        },
        get_mut_active_system_registry::{
            get_mut_active_system_registry
        }
    };

    #[cfg(feature = "blocking-processor")]
    pub use super::{
        blocking_processor::{
           BlockingProcessor,
           blocking_processor_system_registry::{
                BLOCKING_PROCESSOR_SYSTEM_REGISTRY_ACCESS_BUILDER,
                BLOCKING_PROCESSOR_SYSTEM_REGISTRY_RESOURCE_ID,
                BlockingProcessorSystemRegistry,
                get_blocking_processor_system_registry
           }
        }
    };

    #[cfg(feature = "non-blocking-processor")]
    pub use super::{
        non_blocking_processor::{
            NonBlockingProcessor
        }
    };
}