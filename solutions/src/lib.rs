//! Solutions to the Advent of Code puzzles.

/// Reads the entire stdin into a [`String`].
pub fn read_stdin() -> String {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("Should have been able to read stdin to end");

    input
}

/// Sets up the tracing subscriber for logging.
pub fn setup_tracing() {
    let max_level = if cfg!(debug_assertions) {
        ::tracing::Level::TRACE
    } else {
        ::tracing::Level::DEBUG
    };

    use ::std::io::IsTerminal;
    let subscriber = ::tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(max_level)
        .with_ansi(::std::io::stderr().is_terminal())
        .with_writer(::std::io::stderr)
        .with_target(false)
        .finish();

    ::tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

pub mod grid;

pub mod num {
    //! Utilities for working with numbers.

    pub mod u64 {
        /// Returns the number of digits in a number.
        pub fn digits(n: u64) -> u32 {
            n.checked_ilog10().map(|d| d + 1).unwrap_or(1)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_digits() {
                assert_eq!(digits(0), 1);
                assert_eq!(digits(1), 1);
                assert_eq!(digits(9), 1);
                assert_eq!(digits(10), 2);
                assert_eq!(digits(99), 2);
                assert_eq!(digits(100), 3);
                assert_eq!(digits(999), 3);
                assert_eq!(digits(1000), 4);
                assert_eq!(digits(18446744073709551615), 20); // u64::MAX
            }
        }
    }

    pub mod usize {
        /// Returns the number of digits in a number.
        pub fn digits(n: usize) -> u32 {
            n.checked_ilog10().map(|d| d + 1).unwrap_or(1)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_digits() {
                assert_eq!(digits(0), 1);
                assert_eq!(digits(1), 1);
                assert_eq!(digits(9), 1);
                assert_eq!(digits(10), 2);
                assert_eq!(digits(99), 2);
                assert_eq!(digits(100), 3);
                assert_eq!(digits(999), 3);
                assert_eq!(digits(1000), 4);
            }
        }
    }
}
