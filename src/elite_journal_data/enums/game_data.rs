use serde::Deserialize;

#[derive(Deserialize)]
pub enum GameMode {
    Solo,
    Group,
    Open,
}