use serde::{Deserialize, Serialize};

/// TODOのモデルはmodels.rsに定義
#[derive(Debug, Serialize, Deserialize)]
pub struct SampleJson {
    pub text: String,
}