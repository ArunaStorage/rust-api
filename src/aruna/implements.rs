use crate::api::storage::models::v1::Version;

/// Implement partial ord for gRPC version
/// This allows us to compare versions with `<`, `>` etc.
impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.major.partial_cmp(&other.major) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => {
                return ord;
            }
        }
        match self.minor.partial_cmp(&other.minor) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => {
                return ord;
            }
        }
        self.patch.partial_cmp(&other.patch)
    }
}