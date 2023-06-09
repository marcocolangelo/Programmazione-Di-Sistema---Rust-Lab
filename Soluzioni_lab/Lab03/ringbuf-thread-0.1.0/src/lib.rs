use std::{vec, sync::Mutex};

// questo buffer è generico, quindi può essere usato per qualsiasi tipo di dato
// e logica procucer / consumer 
struct RBState<T> {
    pub buf: Vec<T>,
    pub read: usize,
    pub write: usize,
    pub full: bool,
}

impl<T> RBState<T> where 
T: Default + Copy {
    pub fn new(capacity: usize) -> Self {
        RBState {
            buf: vec![T::default(); capacity],
            read: 0,
            write: 0,
            full: false,
        }
    }
}

pub struct RingBuf<T> {
    state: Mutex<RBState<T>>,
}

impl<T> RingBuf<T>
where
    T: Default + Copy,
{
    pub fn new(capacity: usize) -> Self {
        RingBuf {
            state: Mutex::new(RBState::new(capacity)),
        }

    }

    pub fn write(&self, _element: T) -> Result<(), ()> {
        let mut state = self.state.lock().unwrap();

        if state.full {
            return Err(());
        } else {
            let widx = state.write;
            state.buf[widx] = _element;
            state.write = (state.write + 1) % state.buf.len();
            state.full = state.write == state.read;
            return Ok(());
        }
    }

    pub fn read(&self) -> Option<T> {
        let mut state = self.state.lock().unwrap();

        if state.read == state.write && !state.full {
            return None;
        } else {
            let element = state.buf[state.read].clone();
            state.read = (state.read + 1) % state.buf.len();
            state.full = false;
            return Some(element);
        }
    }
}
