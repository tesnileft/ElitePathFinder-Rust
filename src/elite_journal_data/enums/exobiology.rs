use serde::Deserialize;
use std::fmt;
use std::fmt::Display;

#[derive(Deserialize, Debug)]
pub enum Genus {
    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,
    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,
    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,
    #[serde(rename = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,
    #[serde(rename = "$Codex_Ent_Fonticulus_Genus_Name;;")]
    Fonticulua,
    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,
    #[serde(rename = "$Codex_Ent_Shrubs_Genus_Name;")]
    Frutexa,
    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,
    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,
    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,
    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,
    #[serde(rename = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,
    #[serde(rename = "$Codex_Ent_Sphere_Name;")]
    LuteolumAnemone,
    #[serde(rename = "$Codex_Ent_Fumerola_Name;")]
    Fumerola,
    #[serde(rename = "$Codex_Ent_Electricae_Name;")]
    Electricae,
    #[serde(rename = "$Codex_Ent_Clypoid_Name;")]
    Clypeus,
    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    CrystalineShards
}

impl Display for Genus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Species {
    //Aleoida
    Arcus,
    Coronamus,
    #[serde(rename = "$Codex_Ent_Aleoids_05_Name;")]
    Gravis,
    #[serde(rename = "$Codex_Ent_Aleoids_04_Name;")]
    Laminiae,
    Spica,
    //Bacterium
    Acies,
    #[serde(rename="$Codex_Ent_Bacterial_06_Name;")]
    Alcyoneum,
    #[serde(rename = "$Codex_Ent_Bacterial_01_Name;")]
    Aurasus,
    Bullaris,
    #[serde(rename="$Codex_Ent_Bacterial_12_Name;s")]
    Cerbrus,
    #[serde(rename="$Codex_Ent_Bacterial_08_Name;")]
    Informem,
    Nebulus,
    #[serde(rename="$Codex_Ent_Bacterial_11_Name;")]
    Omentum,
    Scopulum,
    #[serde(rename = "$Codex_Ent_Bacterial_07_Name;")]
    Tela,
    Verrata,
    #[serde(rename = "$Codex_Ent_Bacterial_05_Name;")]
    Vesicula,
    #[serde(rename = "$Codex_Ent_Bacterial_09_Name;")]
    Volu,
    //Cactoida
    Lapis,
    Peperatis,
    #[serde(rename = "$Codex_Ent_Cactoid_01_Name;")]
    Cortexum,
    Pullulanta,
    Vermis,
    //Concha
    Aureolas,
    Biconcavis,
    #[serde(rename="$Codex_Ent_Conchas_03_Name;")]
    Labiata,
    Renibus,
    //Electricae
    Pluma,
    Radialem,
    //Fonticulua
    Campestris,
    Digitos,
    Fluctus,
    Lapida,
    Segmentatus,
    Upupam,
    //Fumerola
    Aquatis,
    Carbosis,
    Extremus,
    #[serde(rename="$Codex_Ent_Fumerolas_03_Name;")]
    Nitris,
    //Fungoida
    #[serde(rename = "$Codex_Ent_Fungoids_03_Name;")]
    Bullarum,
    Gelata,
    #[serde(rename="$Codex_Ent_Fungoids_01_Name;")]
    Setisis,
    Stabitis,
    //Frutexa
    #[serde(rename = "$Codex_Ent_Shrubs_02_Name;")]
    Acus,
    Collum,
    Fera,
    #[serde(rename = "$Codex_Ent_Shrubs_01_Name;")]
    Flabellum,
    #[serde(rename = "$Codex_Ent_Shrubs_04_Name;")]
    Flammasis,
    #[serde(rename = "$Codex_Ent_Shrubs_03_Name;")]
    Metallicum,
    Sponsae,
    //Osseus
    Cornibus,
    Discus,
    #[serde(rename = "$Codex_Ent_Osseus_01_Name;")]
    Fractus,
    Pellebantus,
    Pumice,
    #[serde(rename = "$Codex_Ent_Osseus_03_Name;")]
    Spiralis,
    //Stratum
    #[serde(rename = "$Codex_Ent_Stratum_04_Name;")]
    Araneamus,
    #[serde(rename ="$Codex_Ent_Stratum_06_Name;")]
    Cucumisis,
    #[serde(rename ="$Codex_Ent_Stratum_01_Name;")]
    Excutitus,
    Frigus,
    Laminamus,
    Limaxus,
    #[serde(rename = "$Codex_Ent_Stratum_02_Name;")]
    Paleas,
    #[serde(rename = "$Codex_Ent_Stratum_07_Name;")]
    Tectonicas,
    //Tussock
    Albata,
    #[serde(rename = "$Codex_Ent_Tussocks_15_Name;")]
    Capillum,
    #[serde(rename = "$Codex_Ent_Tussocks_11_Name;")]
    Caputus,
    #[serde(rename = "$Codex_Ent_Tussocks_05_Name;")]
    Catena,
    #[serde(rename = "$Codex_Ent_Tussocks_04_Name;")]
    Cultro,
    Divisa,
    Ignis,
    Pennata,
    Pennatis,
    #[serde(rename = "$Codex_Ent_Tussocks_09_Name;")]
    Propagito,
    Serrati,
    Stigmasis,
    #[serde(rename = "$Codex_Ent_Tussocks_12_Name;")]
    Triticum,
    Ventusa,
    Virgam,

    //Recepta
    Conditivus,
    Deltahedronix,
    Umbrux,
    //Tubus
    #[serde(rename="$Codex_Ent_Tubus_03_Name;")]
    Cavas,
    #[serde(rename="$Codex_Ent_Tubus_05_Name;")]
    Compagibus,
    Conifer,
    #[serde(rename="$Codex_Ent_Tubus_04_Name;")]
    Rosarium,
    Sororibus,
    //Anemone
    #[serde(rename="$Codex_Ent_SphereEFGH_02_Name;")]
    Prasinum,
    //Clypeus
    #[serde(rename = "$Codex_Ent_Clypeus_02_Name;")]
    Margaritus,
}

impl Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Default, Debug)]
pub enum ExoBiologyVariant {
    Emerald,
    Gold,
    Maroon,
    Cobalt,
    Green,
    Yellow,
    Orange,
    Red,
    Magenta,
    Lime,
    Peach,
    Mulberry,
    Cyan,
    White,
    Blue,
    Aquamarine,
    Turquoise,
    Grey,
    Teal,
    Sage,
    Russet,
    Mauve,
    Amber,
    Ochre,
    Indigo,
    Amethyst,
    #[default]
    Unknown,
    Ocher,
}

impl Display for ExoBiologyVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}