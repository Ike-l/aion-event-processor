use std::sync::Arc;

use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramRegistry, ResolveResourceError, ResourceId};

pub type ThreadPool = threadpool::ThreadPool;

pub const THREAD_POOL_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor ThreadPool");

pub const THREAD_POOL_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(THREAD_POOL_RESOURCE_ID),
    resource_password: None
};

pub fn get_threadpool<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
) -> Result<Result<Shared<'a, ThreadPool>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve::<Shared<ThreadPool>>(vec![THREAD_POOL_ACCESS_BUILDER])
}