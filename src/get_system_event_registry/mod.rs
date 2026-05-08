use std::{collections::{HashMap, HashSet}, sync::Arc};

use aion_event::prelude::Event;
use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

pub type SystemEventRegistry = HashMap<ResourceId, HashSet<Event>>;

pub const SYSTEM_EVENT_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor SystemEventRegistry");

pub const SYSTEM_EVENT_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(SYSTEM_EVENT_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_system_event_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>,
) -> Result<Result<Result<Shared<'a, SystemEventRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    let mut access_builder = SYSTEM_EVENT_REGISTRY_ACCESS_BUILDER.clone();
    access_builder.program_id = program_id.clone();
    
    program_registry.resolve_with_insert::<Shared<SystemEventRegistry>>(
        vec![access_builder], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(SystemEventRegistry::default()))), 
            resource_id: Some(SYSTEM_EVENT_REGISTRY_RESOURCE_ID), 
            program_id,
            ..Default::default()
        }
    ).unwrap()
}