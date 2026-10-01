use std::{collections::HashSet, sync::Arc};

use aion_program::prelude::{AccessBuilder, ProgramRegistry, ProgramRegistryResolveEitherError, ResourceId, Shared};
use tokio::runtime::Runtime;

pub type BlockingProcessorSystemRegistry = HashSet<ResourceId>;

pub trait GetBlockingProcessorSystemRegistry {
    fn get_blocking_processor_system_registry(
        &self,
        runtime: Option<&Runtime>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Shared<'_, BlockingProcessorSystemRegistry>, ProgramRegistryResolveEitherError>;
}

impl GetBlockingProcessorSystemRegistry for Arc<ProgramRegistry> {
    fn get_blocking_processor_system_registry(
        &self,
        runtime: Option<&Runtime>,
        access_builders: Vec<AccessBuilder>
    ) -> Result<Shared<'_, BlockingProcessorSystemRegistry>, ProgramRegistryResolveEitherError>
    {
        self.resolve_either::<Shared<BlockingProcessorSystemRegistry>>(runtime, None, access_builders)
    }
}

// pub fn get_blocking_processor_system_registry<'a>(
//     program_registry: &'a Arc<ProgramRegistry>,
//     program_id: Option<ProgramId>,
// ) -> Result<Shared<'a, BlockingProcessorSystemRegistry>, ProgramRegistryResolveWithIner> {
//     let mut access_builder = BLOCKING_PROCESSOR_SYSTEM_REGISTRY_ACCESS_BUILDER.clone();
//     access_builder.program_id = program_id.clone();
//     program_registry.resolve_with_insert::<Shared<BlockingProcessorSystemRegistry>>(
//         None,
//         vec![access_builder], 
//         ProgramRegistryResolveWithInsert { 
//             resource: Some(Box::new(|| Resource::new(BlockingProcessorSystemRegistry::default()))), 
//             resource_id: Some(BLOCKING_PROCESSOR_SYSTEM_REGISTRY_RESOURCE_ID), 
//             program_id,
//             ..Default::default()
//         }
//     )
// }