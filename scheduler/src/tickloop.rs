use std::ops::ControlFlow;
use std::thread;
use std::time::{Duration, Instant};

const MAX_BEHIND: Duration = Duration::from_secs(2);

/// Calls tick() until it breaks; avgs at 20!
pub fn run_loop(mut tick: impl FnMut() -> ControlFlow<()>) { // TODO
    let tick_time = Duration::from_millis(1000 / crate::TPS);
    let mut next = Instant::now();

    while tick().is_continue() {
        next += tick_time;

        match next.checked_duration_since(Instant::now()) {
            Some(wait) => thread::sleep(wait),

            None if next.elapsed() > MAX_BEHIND => { // past this delay the loop stops catching up and resets its clock
                eprintln!("Can't keep up! Running {:?} behind, skipping ticks", next.elapsed()); // watchdog

                next = Instant::now();
            }

            None => {}
        }
    }
}