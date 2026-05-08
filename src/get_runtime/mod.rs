use std::sync::Arc;

use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramRegistry, ResolveResourceError, ResourceId};

pub type Runtime = Arc<tokio::runtime::Runtime>;

pub const RUNTIME_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor Runtime");

pub const RUNTIME_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(RUNTIME_RESOURCE_ID),
    resource_password: None
};

pub fn get_runtime<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
) -> Result<Result<Shared<'a, Runtime>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve::<Shared<Runtime>>(vec![RUNTIME_ACCESS_BUILDER])
}