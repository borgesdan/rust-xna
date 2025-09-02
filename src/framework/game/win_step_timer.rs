use windows::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
use crate::exception;
use crate::framework::game::StepTimer;
use crate::shared::{ExceptionConverter, XnaResult};

#[cfg(target_os = "windows")]
impl StepTimer {
    pub(crate) fn win_query_performance_counter(value: &mut i64) -> XnaResult<()> {
        unsafe {
            QueryPerformanceCounter(value)
                .unwrap_or_throw(exception!("QueryPerformanceCounter failed.", None))
        }
    }

    fn win_query_perfomance_frequency(value: &mut i64) -> XnaResult<()> {
        unsafe {
            QueryPerformanceFrequency(value)
                .unwrap_or_throw(exception!("QueryPerfomanceFrequency failed.", None))
        }
    }

    pub(crate) fn win_apply_initial_values(&mut self) -> XnaResult<()>{
        unsafe {
            Self::win_query_performance_counter(&mut self.frequency)?;
            Self::win_query_perfomance_frequency(&mut self.last_time)
        }
    }
}