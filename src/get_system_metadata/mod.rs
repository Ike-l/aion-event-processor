use std::sync::Arc;

use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ResolveResourceError, ResourceId};
use aion_system::prelude::StoredSystemMetadata;

pub fn get_system_metadata<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>,
    system_metadata_resource_id: ResourceId,
) -> Result<Result<Shared<'a, StoredSystemMetadata>, ResolveResourceError>, AccessSubmissionError> {
    let access_builder = AccessBuilder {
        program_id,
        resource_id: Some(system_metadata_resource_id),
        ..Default::default()
    };

    program_registry.resolve::<Shared<StoredSystemMetadata>>(vec![access_builder])
}