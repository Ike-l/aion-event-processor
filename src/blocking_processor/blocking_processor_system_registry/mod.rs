use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};
use aion_processor::prelude::Shared;

pub type BlockingProcessorSystemRegistry = HashSet<ResourceId>;

pub const BLOCKING_PROCESSOR_SYSTEM_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor BlockingProcessorSystemRegistry");

pub const BLOCKING_PROCESSOR_SYSTEM_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(BLOCKING_PROCESSOR_SYSTEM_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_blocking_processor_system_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>,
) -> Result<Result<Result<Shared<'a, BlockingProcessorSystemRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    let mut access_builder = BLOCKING_PROCESSOR_SYSTEM_REGISTRY_ACCESS_BUILDER.clone();
    access_builder.program_id = program_id.clone();
    program_registry.resolve_with_insert::<Shared<BlockingProcessorSystemRegistry>>(
        vec![access_builder], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(BlockingProcessorSystemRegistry::default()))), 
            resource_id: Some(BLOCKING_PROCESSOR_SYSTEM_REGISTRY_RESOURCE_ID), 
            program_id,
            ..Default::default()
        }
    // is only ever None if resource_id or resource is None
    ).unwrap()
}