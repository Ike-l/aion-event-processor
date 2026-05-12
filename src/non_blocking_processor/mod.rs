use std::sync::Arc;

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_processor::prelude::{Processor, Unique};
use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use aion_system::prelude::StoredSystem;

use crate::prelude::{get_mut_join_handle_buffer, parse_result};

// should be called *after* BlockingProcessor because it might acquire conflicting accesses which are held for the entire duration.

// split into Start/Finish if systems want to know if a background system is active? (since even if it has finished it will still be active until it is joined at the start of this execute)

pub struct NonBlockingProcessor;

impl EventSystem for NonBlockingProcessor {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let mut finished_sync_join_handles = Vec::new();
        let mut finished_async_join_handles = Vec::new();

        if let Ok(Ok(Ok(join_handle_buffer))) = get_mut_join_handle_buffer(program_registry) {
            let buffer = join_handle_buffer.as_mut();
            let finished_sync = buffer.0.extract_if(.., |sync_join_handle| {
                sync_join_handle.is_finished()
            });

            let finished_async = buffer.1.extract_if(.., |async_join_handle| {
                async_join_handle.is_finished()
            });

            finished_sync_join_handles.extend(finished_sync);
            finished_async_join_handles.extend(finished_async);
        }

        for join_handle in finished_sync_join_handles {
            let result = join_handle.join();
            
            match result {
                Ok((program_id, system_metadata, stored_system_kind, result)) => {
                    // TODO consume system metadata and remove clones
                    let prompted_access = AccessBuilder {
                        program_id: Some(program_id),
                        program_password: system_metadata.system_program_password().clone(),
                        user_details: system_metadata.user_details().clone(),
                        resource_id: Some(system_metadata.system_resource_id().clone()),
                        resource_access: None,
                        resource_password: system_metadata.system_resource_password().clone(),
                    };
        
                   match program_registry.resolve::<Unique<StoredSystem>>(vec![prompted_access]) {
                        Ok(Ok(mut stored_system)) => {
                            let system = stored_system.as_mut();
        
                            system.put_system(stored_system_kind);
                        },
                        // system will now vanish into the aether
                        _ => ()
                    }
                    match result {
                        Ok(result) => {
                            event_buffer.extend(parse_result(result, program_registry, (program_id, system_metadata.system_resource_id().clone())).into_iter());
                        },
                        Err(error) => todo!(),
                    }
                },
                Err(_) => todo!(),
            }
        }




        // START
        // for each program
        // for system metadata in non blocking system registry
        // if system.test passes
        // systems.insert(program, resource, system metadata)

        // map systems into queue input

        // get runtime

        // for each system in system queue
        // put into active system registry

        let join_handles = Processor::process_non_blocking(
            system_queue, 
            program_registry, 
            runtime
        );

        // store join handles

        event_buffer
    }
}