use std::sync::Arc;

use aion_processor::prelude::{Shared, SystemId};
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};
use execution_graph::prelude::Link;

pub type Links = Vec<Link<SystemId>>;

pub const LINKS_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor Links");

pub const LINKS_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(LINKS_RESOURCE_ID),
    resource_password: None
};

pub fn get_links<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
    program_id: Option<ProgramId>
) -> Result<Result<Result<Shared<'a, Links>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    let mut access_builder = LINKS_ACCESS_BUILDER.clone();
    access_builder.program_id = program_id.clone();
    program_registry.resolve_with_insert::<Shared<Links>>(
        vec![access_builder], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(Links::default()))), 
            resource_id: Some(LINKS_RESOURCE_ID),
            program_id, 
            ..Default::default()
        }
    ).unwrap()
}