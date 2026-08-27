use serde::Deserialize;

#[derive(Deserialize)]
pub struct CargoItem{
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Name_Localized")]
    pub name_localized: Option<String>,
    #[serde(rename = "Count")]
    pub count: u64,
    #[serde(rename = "Stolen")]
    pub stolen: Option<u64>,
}

#[derive(Deserialize)]
pub struct Module{
    #[serde(rename = "Slot")]
    slot: String,
    #[serde(rename = "Item")]
    item: String,
    #[serde(rename = "On")]
    on: bool,
    #[serde(rename = "Priority")]
    priority: u64,
    #[serde(rename = "Health")]
    health: f64
}

#[derive(Deserialize)]
pub struct FuelCapacity{
    #[serde(rename = "Main")]
    main: f64,
    #[serde(rename = "Reserve")]
    reserve: f64,
}