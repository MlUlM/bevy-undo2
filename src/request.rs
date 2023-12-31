use bevy::ecs::system::SystemParam;
use bevy::prelude::{Event, EventWriter, Res};
use crate::counter::UndoCounter;

#[derive(Event, Default, PartialEq, Debug, Copy, Clone, )]
pub(crate) struct RequestUndoEvent(pub UndoCounter);


#[derive(SystemParam)]
pub struct UndoRequester<'w> {
    ew: EventWriter<'w, RequestUndoEvent>,
    counter: Res<'w, UndoCounter>
}


impl<'w> UndoRequester<'w> {
    /// request undo-operation.
    /// This will send　the most recent event registered via [`UndoScheduler`](crate::undo_event::UndoScheduler).
    #[inline(always)]
    pub fn undo(&mut self) {
        self.ew.send(RequestUndoEvent(*self.counter));
    }
}