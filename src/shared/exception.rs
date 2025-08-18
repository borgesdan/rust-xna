use crate::shared::{Exception, ExceptionConverter, SilentExceptionConverter};

impl Exception {
    pub fn new(message: &str, inner: Option<Exception>) -> Self {
        Exception {
            message: message.to_string(),
            inner: if inner.is_some() { Some(Box::new(inner.unwrap())) } else { None },
            ..Default::default()
        }
    }

    pub fn throw(message: &str, inner: Option<Exception>, file: &str, line: u32, column: u32) -> Self {
        Exception {
            message: message.to_string(),
            inner: if inner.is_some() { Some(Box::new(inner.unwrap())) } else { None },
            file: Some(file.to_string()),
            line: Some(line),
            column: Some(column),
        }
    }
}

impl<T> ExceptionConverter<T> for Option<T> {
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception> {
        if self.is_some() {
            return Ok(self.unwrap())
        }

        Err(Exception::new(message, None))
    }

    fn unwrap_ref_or_exception(&self, message: &str) -> Result<&T, Exception> {
        if self.is_some() {
            return Ok(self.as_ref().unwrap());
        }

        Err(Exception::new(message, None))
    }

    fn unwrap_mut_or_exception(&mut self, message: &str) -> Result<&mut T, Exception> {
        if self.is_some() {
            return Ok(self.as_mut().unwrap())
        }

        Err(Exception::new(message, None))
    }

    fn unwrap_or_throw(self, exception: Exception) -> Result<T, Exception> {
        if self.is_some() {
            return Ok(self.unwrap())
        }

        Err(exception)
    }

    fn unwrap_ref_or_throw(&self, exception: Exception) -> Result<&T, Exception> {
        if self.is_some() {
            return Ok(self.as_ref().unwrap());
        }

        Err(exception)
    }

    fn unwrap_mut_or_throw(&mut self, exception: Exception) -> Result<&mut T, Exception> {
        if self.is_some() {
            return Ok(self.as_mut().unwrap())
        }

        Err(exception)
    }
}

impl<T> SilentExceptionConverter<T> for Option<T> {
    fn unwrap_or_default_exception(self) -> Result<T, Exception> {
        if self.is_some() {
            return Ok(self.unwrap())
        }

        Err(Exception::new("Invalid unwrap() operation.", None))
    }

    fn unwrap_ref_or_default_exception(&self) -> Result<&T, Exception> {
        if self.is_some() {
            return Ok(self.as_ref().unwrap());
        }

        Err(Exception::new("Invalid .as_ref().unwrap() operation.", None))
    }

    fn unwrap_mut_or_default_exception(&mut self) -> Result<&mut T, Exception> {
        if self.is_some() {
            return Ok(self.as_mut().unwrap())
        }

        Err(Exception::new("Invalid .as_mut().unwrap() operation", None))
    }
}