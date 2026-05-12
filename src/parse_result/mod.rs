use std::{collections::HashSet, sync::Arc};

use aion_event::prelude::{Event, EventBuffer};
use aion_processor::prelude::SystemId;
use aion_program::prelude::ProgramRegistry;
use aion_system::prelude::SystemResult;

use crate::prelude::get_system_event_registry;

// TODO: LOG RESULT

// for now the only way to parse results is by spawning an event
// so put the whole block into the if let
// otherwise could separate it out and only get when needed? (performance implication)
            

pub fn parse_result(
    result: Option<SystemResult>,
    program_registry: &Arc<ProgramRegistry>,
    (program_id, system_resource_id): SystemId
) -> HashSet<Event> {
    let mut event_buffer = HashSet::default();

    if let Ok(Ok(Ok(system_event_registry))) = get_system_event_registry(program_registry, Some(program_id)) {
        #[cfg(feature = "spawn-all-system-events")]
        {
            if let Some(events) = system_event_registry.as_ref().get(&system_resource_id) {
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
                                if let Some(events) = system_event_registry.as_ref().get(&system_resource_id) {
                                    event_buffer.extend(events.into_iter().cloned());
                                }
                            }
                        }
                    }
                },
            }
        }
    }

    event_buffer
}