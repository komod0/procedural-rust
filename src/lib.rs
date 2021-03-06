#![allow(unused_imports)]

use color_eyre::eyre::{self, WrapErr};
use tracing::{event, info, instrument, span, warn, Level};

/// The string literal `"hello, world!"`
/// ```
/// use procedural_maps::hello;
/// assert_eq!(hello(), "hello, world!");
/// ```
pub fn hello() -> &'static str {
    "hello, world!"
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn hello_test() {
        assert_eq!(hello(), "hello, world!");
    }
}
