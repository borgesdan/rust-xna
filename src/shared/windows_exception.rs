use windows::core::Error;
use crate::shared::{Exception, ExceptionConverter};

impl From<Error> for Exception {
    fn from(value: Error) -> Self {
        let message = value.message();

        Exception::new(message.as_str(), None)
    }
}

impl<T> ExceptionConverter<T> for Result<T, Error> {
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception> {
        if self.is_ok() {
            return Ok(self.unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let exception = Exception::new(message, Some(inner));

        Err(exception)
    }

    fn unwrap_ref_or_exception(&self, message: &str) -> Result<&T, Exception> {
        if self.is_ok() {
            return Ok(self.as_ref().unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let exception = Exception::new(message, Some(inner));

        Err(exception)
    }

    fn unwrap_mut_or_exception(&mut self, message: &str) -> Result<&mut T, Exception> {
        if self.is_ok() {
            return Ok(self.as_mut().unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let inner = Exception::from(error.clone());
        let exception = Exception::new(message, Some(inner));

        Err(exception)
    }

    fn unwrap_or_throw(self, exception: Exception) -> Result<T, Exception> {
        if self.is_ok() {
            return Ok(self.unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let mut ex = exception;
        ex.inner = Some(Box::new(Exception::from(error.clone())));

        Err(ex)
    }

    fn unwrap_ref_or_throw(&self, exception: Exception) -> Result<&T, Exception> {
        if self.is_ok() {
            return Ok(self.as_ref().unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let mut ex = exception;
        ex.inner = Some(Box::new(Exception::from(error.clone())));

        Err(ex)
    }

    fn unwrap_mut_or_throw(&mut self, exception: Exception) -> Result<&mut T, Exception> {
        if self.is_ok() {
            return Ok(self.as_mut().unwrap());
        }

        let error = self.as_ref().err().unwrap();
        let mut ex = exception;
        ex.inner = Some(Box::new(Exception::from(error.clone())));

        Err(ex)
    }
}
