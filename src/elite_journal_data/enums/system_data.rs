use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Default)]
pub enum Economy {
    #[serde(rename = "$economy_Agri;")]
    Agricultural,
    #[serde(rename = "$economy_Carrier;")]
    Carrier,
    #[serde(rename = "$economy_Industrial;")]
    Industrial,
    #[serde(rename = "$economy_Military;")]
    Military,
    #[serde(rename = "$economy_Extraction;")]
    Extraction,
    #[serde(rename = "$economy_HighTech;")]
    HighTech,
    #[serde(rename = "$economy_Terraforming;")]
    Terraforming,
    #[serde(rename = "$economy_Tourism;")]
    Tourism,
    #[serde(rename = "$economy_Refinery;")]
    Refinery,
    #[serde(rename = "$economy_Colony;")]
    Colony,
    #[default]
    #[serde(rename = "$economy_None;")]
    None,
}

//region - System Factions -
///System Security states, Anarchy is lowest
#[derive(Deserialize, Default)]
pub enum SystemSecurity {
    #[default]
    #[serde(rename = "$GAlAXY_MAP_INFO_state_anarchy;")]
    Anarchy,
    #[serde(rename = "$SYSTEM_SECURITY_low;")]
    Low,
    #[serde(rename = "$SYSTEM_SECURITY_medium;")]
    Medium,
    #[serde(rename = "$SYSTEM_SECURITY_high;")]
    High,
}

///All states a faction (and thus system at large) can be in, will be represented with an `Option<FactionState>` field, since there can be no active state.
#[derive(Deserialize, Default)]
pub enum FactionState {
    #[default]
    None,
    Boom,
    Bust,
    CivilUnrest,
    Famine,
    Lockdown,
    Outbreak,
    War,
    CivilWar,
    Election,
    Retreat,
    Expansion,
    Blight,
    Drought,
    InfrastructureFailure,
    Terrorism,
    NaturalDisaster,
    PublicHoliday,
    CivilLiberty,
    Investment,

}

///Possible system governments
#[derive(Deserialize, Default)]
pub enum Government {
    #[serde(alias = "$government_Anarchy;")]
    Anarchy,
    Communist,
    #[serde(alias = "$government_Confederacy;")]
    Confederacy,
    #[serde(alias = "$government_Cooperative;")]
    Cooperative,
    #[serde(alias = "$government_Corporate;")]
    Corporate,
    #[serde(alias = "$government_Democracy;")]
    Democracy,
    Dictatorship,
    Feudal,
    #[serde(alias = "$government_Patronage;")]
    Patronage,
    PrisonColony,
    #[serde(alias = "$government_Theocracy;")]
    Theocracy,
    #[serde(alias = "$government_Engineer;")]
    Engineer,
    #[serde(alias = "$government_Megaconstruction;")]
    MegaConstruction,
    #[default]
    #[serde(alias = "$government_None;")]
    None,
}

///Current Powerplay State a system can be in, only covers the state the ruling faction is part of, so will never be "Exploiting" or "Undermining"
#[derive(Deserialize, Default)]
pub enum PowerplayState {
    #[default]
    Unoccupied,
    Stronghold,
    Exploited,
    Fortified,
}

/// Enum of all Powerplay factions, Spaces and dashes removed. Use <enum value>.to_string() for the full name.
#[derive(Deserialize, Default)]
pub enum PowerplayPower {
    #[serde(rename = "A. Lavigny-Duval")]
    ALavignyDuval,
    #[serde(rename = "Aisling Duval")]
    AislingDuval,
    #[serde(rename = "Denton Patreus")]
    DentonPatreus,
    #[serde(rename = "Edmund Mahon")]
    EdmundMahon,
    #[serde(rename = "Zemina Torval")]
    ZeminaTorval,
    #[serde(rename = "Li Yong-Rui")]
    LiYongRui,
    #[serde(rename = "Jerome Archer")]
    JeromeArcher,
    #[serde(rename = "Yuri Grom")]
    YuriGrom,
    #[serde(rename = "Felicia Winters")]
    FeliciaWinters,
    #[serde(rename = "Nakato Kaine")]
    NakatoKaine,
    #[serde(rename = "Pranav Antal")]
    PranavAntal,
    #[default]
    None, //TODO Fill out
}

impl fmt::Display for PowerplayPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerplayPower::ALavignyDuval => write!(f, "A. Lavigny-Duval"),
            PowerplayPower::AislingDuval => write!(f, "Aisling Duval"),
            PowerplayPower::DentonPatreus => write!(f, "Denton Patreus"),
            PowerplayPower::ZeminaTorval => write!(f, "Zemina Torval"),
            _ => write!(f, "Undefined"),
        }
    }
}