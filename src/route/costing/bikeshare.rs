use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BikeshareCostingOptions {
    /// TODO: implement builder with all option. If you want to contribute this, a PR is appreciated
    #[serde(skip)]
    _placeholder: (),
}
