use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use crate::exception;
use crate::shared::{Exception, ExceptionConverter, Ptr, XnaResult};

impl<T> Ptr<T> {
    pub fn new(value: T) -> Ptr<T> {
        Self::internal_new(Some(value))
    }

    fn internal_new(value: Option<T>) -> Ptr<T> {
        let mut ptr = Self {
            pointer: None,
        };

        ptr.set(value);

        ptr
    }

    pub fn null() -> Ptr<T> {
        Self::internal_new(None)
    }

    pub fn is_null(&self) -> bool {
        self.pointer.is_none()
    }

    pub fn set_null(&mut self) {
        self.pointer = None
    }

    pub fn set(&mut self, value: Option<T>) {
        if value.is_none() {
            self.set_null();
            return;
        }

        if self.pointer.is_some() {
            let mut ptr = self.pointer.as_mut().unwrap();
            let v = value.unwrap();
            *ptr.borrow_mut() = v;
        } else {
            self.pointer = Some(Rc::new(RefCell::new(value.unwrap())));
        }
    }

    pub fn get(&self) -> XnaResult<Ref<T>> {
        let ptr = self.pointer
            .unwrap_ref_or_throw(exception!("The pointer is null", None))?;

        Ok(ptr.borrow())
    }

    pub fn try_get(&self, exception: Exception) -> XnaResult<Ref<T>> {
        let ptr = self.pointer
            .unwrap_ref_or_throw(exception)?;

        Ok(ptr.borrow())
    }

    pub fn get_mut(&mut self) -> XnaResult<RefMut<T>> {
        let ptr = self.pointer
            .unwrap_mut_or_throw(exception!("The pointer is null", None))?;

        Ok(ptr.borrow_mut())
    }

    pub fn try_get_mut(&mut self, exception: Exception) -> XnaResult<RefMut<T>> {
        let ptr = self.pointer
            .unwrap_mut_or_throw(exception)?;

        Ok(ptr.borrow_mut())
    }
}