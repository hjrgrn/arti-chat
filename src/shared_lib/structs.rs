use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TorSvc {
    // TODO: getters
    pub state_dir: String,
    pub cache_dir: String,
}

