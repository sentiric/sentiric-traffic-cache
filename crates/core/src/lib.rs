//! The Core Crate
    //!
    //! This crate contains the shared business logic, data structures, and traits
    //! for the `sentiric-traffic-cache`. It is designed to be completely free of
    //! I/O operations and framework-specific details, making it highly testable
    //! and portable.

    pub fn add(left: usize, right: usize) -> usize {
        left + right
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let result = add(2, 2);
            assert_eq!(result, 4);
        }
    }