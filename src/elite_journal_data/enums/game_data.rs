use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum GameMode {
    Solo,
    Group,
    Open,
    MainGame,
}