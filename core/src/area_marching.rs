use crate::{Domain, FastMarching};

impl FastMarching for Domain {
    fn trial_area(&self, elapsed_time: f32) -> f32 {
        _ = elapsed_time;
        // need to add this, treat time as web distance and it works exactlyu
        // the same
        todo!()
    }
}
