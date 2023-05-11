use macroquad::prelude::get_time;

pub struct Timer {
    pub start_time: f64,
    pub life_time:f64,
    pub first_time: bool,
}

impl Timer {
    pub fn new(time: f64, intial: bool) -> Timer {
        Timer{ start_time: get_time(), life_time: time, first_time: intial }
    }

    pub fn is_timer_done(&self) -> bool {
        let current_time = get_time();
        let time_elapsed = current_time - self.start_time;
        if time_elapsed == self.life_time {
            true
        }
        else {
            false
        }
    }
}