//! Core library for `EchoFiber`.
//!
//! SOR parsing and export models will be added in later milestones.

/// Returns the version of the core library.
#[must_use]
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn reports_the_package_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }
}
