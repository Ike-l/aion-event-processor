use std::{collections::{HashMap, HashSet}, sync::Arc};

use aion_event::prelude::{EventBuffer, EventHistory, EventSystem};
use aion_processor::prelude::{ActivatableSystemQueue, ProcessConfig, Processor, SystemQueue};
use aion_program::prelude::ProgramRegistry;

use crate::prelude::{get_blocking_processor_system_registry, get_links, get_mut_active_system_registry, get_runtime, get_system_criteria_registry, get_system_event_registry, get_system_metadata, get_threadpool};

pub mod blocking_processor_system_registry;

pub struct BlockingProcessor;

impl EventSystem for BlockingProcessor {
    fn execute(
        &self,
        program_registry: &Arc<ProgramRegistry>, 
        current_events: &EventBuffer,
        _event_history: &EventHistory,
    ) -> EventBuffer {
        let mut event_buffer = EventBuffer::default();

        let mut systems = HashMap::new();
        let mut main_thread_systems = HashSet::new();
        for program_id in program_registry.program_ids() {
            if let Ok(Ok(Ok(blocking_processor_system_registry))) = get_blocking_processor_system_registry(program_registry, Some(program_id.clone())) {
                for system_metadata_resource_id in blocking_processor_system_registry.as_ref().iter() {
                    let Ok(Ok(system_metadata)) = get_system_metadata(program_registry, Some(program_id.clone()), system_metadata_resource_id.clone()) else { continue };
                    
                    let resource_id = system_metadata.as_ref().system_resource_id();
                    if let Ok(Ok(Ok(system_conditionals))) = get_system_criteria_registry(program_registry, Some(program_id.clone())) {
                        if let Some(system_conditional) = system_conditionals.as_ref().get(resource_id) {
                            if !system_conditional.test(current_events) {
                                continue;
                            }
                        }
                    }

                    if system_metadata.as_ref().requires_main_thread() {
                        main_thread_systems.insert((program_id.clone(), resource_id.clone()));
                    }


                    systems.insert((program_id, resource_id.clone()), system_metadata);
                }
            }
        }

        let system_queue_input = systems.iter().map(|((program_id, resource_id), system_metadata)| {
            ((*program_id, resource_id), system_metadata.as_ref())
        });

        let system_queue = SystemQueue::new(system_queue_input);

        let mut registry_links = Vec::new();
        for program_id in program_registry.program_ids() {
            if let Ok(Ok(Ok(links))) = get_links(program_registry, Some(program_id.clone())) {
                registry_links.extend(links.as_ref().clone());
            }
        }

        let threadpool = if let Ok(Ok(threadpool)) = get_threadpool(program_registry) {
            Some(threadpool)
        } else { None };

        let runtime = if let Ok(Ok(runtime)) = get_runtime(program_registry) {
            Some(runtime)
        } else { None };

        let threadpool = threadpool.as_ref().map(|threadpool| threadpool.as_ref());
        let runtime = runtime.as_ref().map(|runtime| runtime.as_ref());

        let activatable_system_queue = ActivatableSystemQueue::new(system_queue, program_registry);
        
        for ((program_id, system_id), _) in activatable_system_queue.get_systems() {
            if let Ok(Ok(Ok(mut active_system_registry))) = get_mut_active_system_registry(program_registry, Some((*program_id).clone())) {
                active_system_registry.as_mut().insert((*system_id).clone());
            }
        }

        let results = Processor::process_blocking(
            activatable_system_queue, 
            registry_links,
            main_thread_systems, 
            program_registry, 
            ProcessConfig {
                threadpool,
                runtime
            }
        );

        for program_id in program_registry.program_ids() {
            if let Ok(Ok(Ok(mut active_system_registry))) = get_mut_active_system_registry(program_registry, Some(program_id.clone())) {
                todo!("only remove from active what was added before");
                // since NonBlocking can add to this registry
                // active_system_registry.as_mut().clear();
            }
        }

        // resource id is SystemMetadataResourceId
        for ((program_id, resource_id), result) in results {
            // TODO: LOG RESULT

            // for now the only way to parse results is by spawning an event
            // so put the whole block into the if let
            // otherwise could separate it out and only get when needed? (performance implication)
            if let Ok(Ok(Ok(system_event_registry))) = get_system_event_registry(program_registry, Some(program_id)) {
                #[cfg(feature = "spawn-all-system-events")]
                {
                    if let Some(events) = system_event_registry.as_ref().get(&resource_id) {
                        event_buffer.extend(events.into_iter().cloned());
                    }
                }
    
                if let Some(result) = result {
                    match result {
                        Ok(_message) => todo!("Log message"),
                        #[allow(unused_variables)]
                        Err(error) => {
                            #[cfg(not(feature = "spawn-all-system-events"))]
                            {
                                if let Ok(use_event) = error.downcast::<bool>() {
                                    if use_event {
                                        if let Some(events) = system_event_registry.as_ref().get(&resource_id) {
                                            event_buffer.extend(events.into_iter().cloned());
                                        }
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }

        event_buffer
    }
}