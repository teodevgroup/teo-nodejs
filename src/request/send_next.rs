use teo::prelude::Next;

#[derive(Clone, Copy)]
pub struct SendNext(* const dyn Next);

impl SendNext {

    pub fn new(next: &'static dyn Next) -> Self {
        Self(next as * const dyn Next)
    }

    pub fn next(&self) -> &'static dyn Next {
        unsafe {
            &*self.0 as &dyn Next
        }
    }
}

unsafe impl Send for SendNext {}
unsafe impl Sync for SendNext {}
