use windows::Win32::System::Performance::{QueryPerformanceFrequency};
use crate::framework::game::StepTimer;
use crate::shared::{XnaResult};

impl StepTimer {
    pub fn new() -> XnaResult<Self> {
        let mut step_timer = StepTimer {
            is_fixed_time_step: false,
            target_elapsed_ticks: Self::TICKS_PER_SECOND / 60,
            ..Default::default()
        };

        if cfg!(target_os = "windows") {
            step_timer.win_apply_initial_values()?;
        }

        Ok(step_timer)
    }

    pub fn reset_elapsed_time(&mut self) -> XnaResult<()> {
        if cfg!(target_os = "windows") {
            Self::win_query_performance_counter(&mut self.last_time)?;
        }

        self.left_over_ticks = 0;
        self.frames_per_second = 0;
        self.frames_this_second = 0;
        self.second_counter = 0;

        Ok(())
    }

    pub fn tick<FUpdate>(&mut self,update: &mut FUpdate) -> XnaResult<()> where FUpdate: FnMut(&StepTimer) -> XnaResult<()>{
        let mut current_time : i64 = 0;

        if cfg!(target_os = "windows") {
            Self::win_query_performance_counter(&mut current_time)?;
        }

        let mut time_delta:u64 = (current_time - self.last_time) as u64;

        self.last_time = current_time;
        self.second_counter = self.second_counter + time_delta;

        if(time_delta > self.max_delta){
            time_delta = self.max_delta
        }

        time_delta = time_delta * Self::TICKS_PER_SECOND;
        time_delta = time_delta / self.frequency as u64;

        let last_frame_count = self.frame_count;

        if(self.is_fixed_time_step){
            let diff = (time_delta - self.target_elapsed_ticks) as i64;

            if diff.abs() < Self::TICKS_PER_SECOND as i64 / 4000 {
                time_delta = self.target_elapsed_ticks;
            }

            self.left_over_ticks = self.left_over_ticks + time_delta;

            loop {
                if self.left_over_ticks >= self.target_elapsed_ticks {
                    self.elapsed_ticks = self.target_elapsed_ticks;
                    self.total_ticks = self.total_ticks + self.target_elapsed_ticks;
                    self.left_over_ticks = self.left_over_ticks - self.target_elapsed_ticks;
                    self.frame_count = self.frame_count + 1;

                    update(self)?;
                } else {
                    break;
                }
            }
        } else {
            self.elapsed_ticks = time_delta;
            self.total_ticks = self.total_ticks + time_delta;
            self.left_over_ticks = 0;
            self.frame_count = self.frame_count + 1;

            update(self)?;
        }

        if self.frame_count != last_frame_count {
            self.frames_this_second = self.frames_this_second + 1;
        }

        if self.second_counter >= self.frequency as u64 {
            self.frames_per_second = self.frames_this_second;
            self.frames_this_second = 0;
            self.second_counter = self.second_counter % self.frequency as u64;
        }

        Ok(())
    }

    pub fn get_elapsed_ticks(&self) -> u64 {
        self.elapsed_ticks
    }

    pub fn get_elapsed_seconds(&self) -> f64 {
        Self::ticks_to_seconds(self.elapsed_ticks)
    }

    pub fn get_total_ticks(&self) -> u64 {
        self.total_ticks
    }

    pub fn get_total_seconds(&self) -> f64 {
        Self::ticks_to_seconds(self.total_ticks)
    }

    pub fn get_frame_count(&self) -> u32 {
        self.frames_per_second
    }

    pub fn get_frame_per_second(&self) -> u32 {
        return self.frames_per_second;
    }

    pub fn set_target_elapsed_seconds(&mut self, target_elapsed: f64) {
        self.target_elapsed_ticks = Self::seconds_to_ticks(target_elapsed);
    }

    fn ticks_to_seconds(ticks: u64) -> f64 {
        (ticks / Self::TICKS_PER_SECOND) as f64
    }

    fn seconds_to_ticks(seconds: f64) -> u64 {
        (seconds * Self::TICKS_PER_SECOND as f64) as u64
    }

    pub const TICKS_PER_SECOND: u64 = 10000000;
}

