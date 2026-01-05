//! # Crate Publish
//!
//! 'crate_publish' is a collection of utilities to make performing certain calculations
//! more convenient.

pub use self::helpers::{
    add_world,
    arithmatic_operations::{self, add, add_one},
};

pub mod helpers {

    pub mod arithmatic_operations {
        use std::collections::binary_heap::Iter;

        /// Adds one to the number given
        ///
        /// # Examples
        ///
        /// ```
        /// let arg = 5;
        /// let result = crates_publish::helpers::arithmatic_operations::add_one(arg);
        ///
        /// assert_eq!(6, result);
        /// ```

        pub fn add_one(num: i32) -> i32 {
            num + 1
        }

        /// Add function takes a Iter<i32> type and returns the sum of the collection
        pub fn add(nums: Iter<i32>) -> i32 {
            nums.sum()
        }
    }
    /// Adds the string "world" to `s`.
    pub fn add_world(s: &mut String) {
        s.push_str("world");
    }
}
