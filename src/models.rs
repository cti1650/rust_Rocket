use serde::{Deserialize, Serialize};

/// モデルはmodels.rsに定義
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleJson {
    pub text: String,
}