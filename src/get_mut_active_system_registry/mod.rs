use std::{collections::HashSet, sync::Arc};

use aion_processor::prelude::Unique;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

pub type ActiveSystemRegistry = HashSet<ResourceId>;

pub const ACTIVE_SYSTEM_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor ActiveSystemRegistry");

pub const ACTIVE_SYSTEM_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(ACTIVE_SYSTEM_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_active_system_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>,
) -> Result<Result<Result<Unique<'a, ActiveSystemRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    let mut access_builder = ACTIVE_SYSTEM_REGISTRY_ACCESS_BUILDER.clone();
    access_builder.program_id = program_id.clone();
    
    program_registry.resolve_with_insert::<Unique<ActiveSystemRegistry>>(
        vec![access_builder], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(ActiveSystemRegistry::default()))), 
            resource_id: Some(ACTIVE_SYSTEM_REGISTRY_RESOURCE_ID), 
            program_id,
            ..Default::default()
        }
    ).unwrap()
}