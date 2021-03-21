pub use clock::Clock;
mod clock {
    use std::fmt::{Display, Formatter, Result};
    #[derive(Debug, Eq, PartialEq)]
    pub struct Clock {
        minutes: i32,
    }

    const MINUTES_PER_HOUR: i32 = 60;
    const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

    impl Clock {
        pub fn new(hours: i32, minutes: i32) -> Self {
            Clock {
                minutes: (hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY),
            }
        }

        pub fn add_minutes(&self, minutes: i32) -> Self {
            Clock::new(0, self.minutes + minutes)
        }
    }

    impl Display for Clock {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "{:02}:{:02}",
                self.minutes.div_euclid(MINUTES_PER_HOUR),
                self.minutes.rem_euclid(MINUTES_PER_HOUR)
            )
        }
    }
}
