use std::cell::RefCell;
use std::rc::Rc;
use thiserror::Error;

#[cfg(feature = "windows")]
pub mod windows_exception;
pub mod exception;
pub mod ptr;
pub mod string_helper;

///Estrutura de erro padrão do Xna
#[derive(Error, Debug, Default, Eq, PartialEq, Clone)]
#[error("{message}")]
pub struct Exception {
    pub message: String,
    pub inner: Option<Box<Exception>>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub file: Option<String>,
}

/// Macro para criar o erro automaticamente com file/line/column
#[macro_export]
macro_rules! exception {
    ($msg:expr, $inner:expr) => {
        Exception::throw($msg, $inner, file!(), line!(), column!())
    };
}

///Um trait que facilita disparos de exceções, por exemplo, para um Option<T>.
pub trait ExceptionConverter<T> {
    fn unwrap_or_exception(self, message: &str) -> Result<T, Exception>;
    fn unwrap_ref_or_exception(&self, message: &str) -> Result<&T, Exception>;
    fn unwrap_mut_or_exception(&mut self, message: &str) -> Result<&mut T, Exception>;

    fn unwrap_or_throw(self, exception: Exception) -> Result<T, Exception>;
    fn unwrap_ref_or_throw(&self, exception: Exception) -> Result<&T, Exception>;
    fn unwrap_mut_or_throw(&mut self, exception: Exception) -> Result<&mut T, Exception>;
}

///Um trait que facilita disparos de exceções com mensagem padrão.
pub trait SilentExceptionConverter<T> {
    fn unwrap_or_default_exception(self) -> Result<T, Exception>;
    fn unwrap_ref_or_default_exception(&self) -> Result<&T, Exception>;
    fn unwrap_mut_or_default_exception(&mut self) -> Result<&mut T, Exception>;
}

///Representa um ponteiro (um Rc<RefCell>) e fornece funções de acesso e verificação de nulo (vazio).
#[derive(Default, Eq, PartialEq, Clone, Debug)]
pub struct Ptr<T> {
    pub pointer: Option<Rc<RefCell<T>>>,
}

///Tipo de Resultado padrão do Xna.
pub type XnaResult<T> = Result<T, Exception>;
