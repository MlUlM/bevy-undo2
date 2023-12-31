use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;
use bevy::prelude::{Event, Resource};


#[derive(Event, Clone)]
pub(crate) struct RequestCommitReservationsFromSchedulerEvent;


#[derive(Event, Clone)]
pub(crate) struct RequestCommitReservationsEvent;


#[derive(Event, Clone)]
pub(crate) struct UndoReserveEvent<E: Event + Clone> {
    pub inner: E,
    pub reserve_no: usize,
}


#[derive(Resource, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(transparent)]
pub(crate) struct ReserveCounter(usize);


impl Deref for ReserveCounter {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl ReserveCounter {
    #[inline(always)]
    pub fn increment(&mut self) {
        self.0 += 1;
    }


    #[inline(always)]
    pub fn reset(&mut self) {
        self.0 = 0;
    }
}


#[derive(Resource)]
pub(crate) struct UndoReservedArea<E: Event + Clone>(pub(crate) Vec<UndoReserveEvent<E>>);


impl<E: Event + Clone> UndoReservedArea<E> {
    #[inline]
    pub fn push(&mut self, event: UndoReserveEvent<E>) {
        self.0.push(event);
    }


    #[inline]
    pub fn pop_front(&mut self) -> Option<UndoReserveEvent<E>> {
        self.0.pop()
        // if self.0.is_empty(){
        //     None
        // }else{
        //     Some(self.0.remove(0))
        // }
    }
}


impl<E: Event + Clone> Default for UndoReservedArea<E> {
    #[inline(always)]
    fn default() -> Self {
        Self(Vec::new())
    }
}




