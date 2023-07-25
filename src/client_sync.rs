use serde::{Serialize, Deserialize};
use derive_builder::Builder;

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, Builder)]
pub struct ClientSync {
    is_free_tier: bool,
}

