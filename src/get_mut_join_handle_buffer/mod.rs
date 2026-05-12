use std::{sync::Arc, thread::JoinHandle};

use aion_processor::prelude::Unique;
use aion_program::prelude::{AccessBuilder, AccessSubmissionError, ProgramId, ProgramRegistry, ProgramRegistryReplaceResourceError, ProgramRegistryResolveWithInsert, ResolveResourceError, Resource, ResourceId};
use aion_system::prelude::{StoredSystemKind, SystemError, SystemResult};


pub type SyncJoinHandles = Vec<((ProgramId, ResourceId), JoinHandle<(StoredSystemKind, Result<Option<SystemResult>, SystemError>)>)>;
pub type AsyncJoinHandles = Vec<((ProgramId, ResourceId), tokio::task::JoinHandle<(StoredSystemKind, Result<Option<SystemResult>, SystemError>)>)>;

pub type JoinHandleBuffer = (SyncJoinHandles, AsyncJoinHandles);

pub const JOIN_HANDLE_BUFFER_RESOURCE_ID: ResourceId = ResourceId::StaticLabel("EventProcessor JoinHandleBuffer");

pub const JOIN_HANDLE_BUFFER_ACCESS_BUILDER: AccessBuilder = AccessBuilder {
    user_details: None,
    program_id: None,
    program_password: None,
    resource_access: None,
    resource_id: Some(JOIN_HANDLE_BUFFER_RESOURCE_ID),
    resource_password: None
};

pub fn get_mut_join_handle_buffer<'a>(
    program_registry: &'a Arc<ProgramRegistry>,
) -> Result<Result<Result<Unique<'a, JoinHandleBuffer>, ProgramRegistryReplaceResourceError>, ResolveResourceError>, AccessSubmissionError> {
    program_registry.resolve_with_insert::<Unique<JoinHandleBuffer>>(
        vec![JOIN_HANDLE_BUFFER_ACCESS_BUILDER], 
        ProgramRegistryResolveWithInsert { 
            resource: Some(Box::new(|| Resource::new(JoinHandleBuffer::default()))), 
            resource_id: Some(JOIN_HANDLE_BUFFER_RESOURCE_ID), 
            ..Default::default()
        }
    ).unwrap()
}