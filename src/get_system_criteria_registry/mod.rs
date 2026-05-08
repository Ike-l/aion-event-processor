use std::{collections::HashMap, sync::Arc};

use aion_processor::prelude::Shared;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};

use crate::prelude::SystemCriteria;

pub mod system_criteria;

pub type SystemCriteriaRegistry = HashMap<ResourceId, SystemCriteria>;

pub const SYSTEM_CRITERIA_REGISTRY_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor SystemCriteriaRegistry");

pub const SYSTEM_CRITERIA_REGISTRY_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(SYSTEM_CRITERIA_REGISTRY_RESOURCE_ID),
    resource_password: None
};

pub fn get_system_criteria_registry<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>,
) -> Result<Result<Result<Shared<'a, SystemCriteriaRegistry>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    let mut access_builder = SYSTEM_CRITERIA_REGISTRY_ACCESS_BUILDER.clone();
    access_builder.program_id = program_id.clone();
    
    program_registry.resolve_with_insert::<Shared<SystemCriteriaRegistry>>(
        vec![access_builder], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(SystemCriteriaRegistry::default()))), 
            resource_id: Some(SYSTEM_CRITERIA_REGISTRY_RESOURCE_ID), 
            program_id,
            ..Default::default()
        }
    ).unwrap()
}