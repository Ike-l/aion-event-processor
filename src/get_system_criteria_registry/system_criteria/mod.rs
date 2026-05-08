use aion_event::prelude::EventBuffer;

pub struct SystemCriteria {
    activator: Box<dyn Fn(&EventBuffer) -> bool + Send + Sync>
}

impl SystemCriteria {
    pub fn new(criteria: impl Fn(&EventBuffer) -> bool + Send + Sync + 'static) -> Self {
        Self {
            activator: Box::new(criteria)
        }
    }

    pub fn test(&self, events: &EventBuffer) -> bool {
        (self.activator)(events)
    }
}