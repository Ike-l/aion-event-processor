use std::{collections::{HashMap, HashSet}, sync::Arc};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_processor::prelude::{ActivatableSystemQueue, Processor, SystemQueue, Unique};
use aion_program::prelude::{AccessBuilder, ProgramRegistry};
use aion_system::prelude::StoredSystem;

use crate::prelude::{get_mut_active_system_registry, get_mut_join_handle_buffer, get_non_blocking_processor_system_registry, get_runtime, get_system_criteria_registry, get_system_metadata, parse_result};

pub mod non_blocking_processor_system_registry;
// should be called *after* BlockingProcessor because it might acquire conflicting accesses which are held for the entire duration.

// split into Start/Finish if systems want to know if a background system is active? (since even if it has finished it will still be active until it is joined at the start of this execute)

pub struct NonBlockingProcessor;

impl EventSystem for NonBlockingProcessor {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let mut finished_sync_join_handles = Vec::new();
        let mut finished_async_join_handles = Vec::new();

        if let Ok(Ok(Ok(mut join_handle_buffer))) = get_mut_join_handle_buffer(program_registry) {
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
                        program_id: Some(program_id.clone()),
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
                            event_buffer.extend(parse_result(result, program_registry, (program_id.clone(), system_metadata.system_resource_id().clone())).into_iter());
                        },
                        Err(_error) => { /*TODO LOG ERROR */ },
                    }

                    if let Ok(Ok(Ok(mut active_system_registry))) = get_mut_active_system_registry(program_registry, Some(program_id)) {
                        active_system_registry.as_mut().remove(system_metadata.system_resource_id());
                    }
                },
                Err(_) => todo!(),
            }
        }


        let mut systems = HashMap::new();
        for program_id in program_registry.program_ids() {
            if let Ok(Ok(Ok(blocking_processor_system_registry))) = get_non_blocking_processor_system_registry(program_registry, Some(program_id.clone())) {
                for system_metadata_resource_id in blocking_processor_system_registry.as_ref().iter() {
                    let Ok(Ok(system_metadata)) = get_system_metadata(program_registry, Some(program_id.clone()), system_metadata_resource_id.clone()) else { continue };
                    if system_metadata.as_ref().requires_main_thread() {
                        // TODO LOG CANNOT PROCESS THIS SYSTEM
                        continue;
                    }
                    
                    let resource_id = system_metadata.as_ref().system_resource_id();
                    if let Ok(Ok(Ok(system_conditionals))) = get_system_criteria_registry(program_registry, Some(program_id.clone())) {
                        if let Some(system_conditional) = system_conditionals.as_ref().get(resource_id) {
                            if !system_conditional.test(current_events) {
                                continue;
                            }
                        }
                    }



                    systems.insert((program_id, resource_id.clone()), system_metadata);
                }
            }
        }

        let system_queue_input = systems.iter().map(|((program_id, resource_id), system_metadata)| {
            ((*program_id, resource_id), system_metadata.as_ref())
        });

        let system_queue = SystemQueue::new(system_queue_input);
        
        let activatable_system_queue = ActivatableSystemQueue::new(system_queue, program_registry);

        let runtime = if let Ok(Ok(runtime)) = get_runtime(program_registry) {
            Some(runtime)
        } else { None };

        let runtime = runtime.as_ref().map(|runtime| runtime.as_ref()).unwrap();

        let mut activated_systems = HashSet::new();
        for ((program_id, system_id), _) in activatable_system_queue.get_systems() {
            if let Ok(Ok(Ok(mut active_system_registry))) = get_mut_active_system_registry(program_registry, Some((*program_id).clone())) {
                activated_systems.insert((*system_id).clone());
                active_system_registry.as_mut().insert((*system_id).clone());
            }
        }

        let join_handles = Processor::process_non_blocking(
            activatable_system_queue, 
            program_registry, 
            runtime
        );

        if let Ok(Ok(Ok(mut join_handle_buffer))) = get_mut_join_handle_buffer(program_registry) {
            let buffer = join_handle_buffer.as_mut();
            buffer.0.extend(join_handles.0);
            buffer.1.extend(join_handles.1);
        }

        event_buffer
    }
}