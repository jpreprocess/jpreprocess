use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
/// サ変
pub enum SaIrregular {
    /// スル
    Alone,
    /// －スル
    ConjugationSuru,
    /// －ズル
    ConjugationZuru,
}
