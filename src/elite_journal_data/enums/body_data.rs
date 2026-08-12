use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub enum BodyType {
    Star,
    Planet,
    Station,
}

#[derive(Deserialize)]
pub enum StarClass {
    O,
    A,
    B,
    D,
    DA,
    G,
    F,
    K,
    L,
    T,
    TTS,
    Ae,
    Y,
    W,
    N,
    M,
    DB,
    DC,
    DO,
    DQ,
    DX,
    DAV,
    DBV,
    DCV,
    WO,
    WC,
    WNC,
    WN,
    AeBe,
    H,
    SupermassiveBlackHole,
    X,
    RoguePlanet,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub enum AtmosphereType {
    Argon,
    ArgonRich,
    Methane,
    MethaneRich,
    SilicateVapour,
    CarbonDioxide,
    CarbonDioxideRich,
    Nitrogen,
    SulphurDioxide,
    Helium,
    Neon,
    NeonRich,
    Oxygen,
    Ammonia,
    AmmoniaRich,
    AmmoniaOxygen,
    Water,
    WaterRich,
    #[default]
    None,
}

impl fmt::Display for AtmosphereType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Eq, PartialEq)]
pub enum PlanetClass {
    #[serde(rename = "High metal content body")]
    HMC,
    #[serde(rename = "Metal rich body")]
    MetalRich,
    #[serde(rename = "Rocky body")]
    Rocky,
    #[serde(rename = "Icy body")]
    Icy,
    #[serde(rename = "Rocky ice body")]
    RockyIce,
    #[serde(rename = "Sudarsky class I gas giant")]
    GasGiantClass1,
    #[serde(rename = "Sudarsky class II gas giant")]
    GasGiantClass2,
    #[serde(rename = "Sudarsky class III gas giant")]
    GasGiantClass3,
    #[serde(rename = "Sudarsky class IV gas giant")]
    GasGiantClass4,
    #[serde(rename = "Sudarsky class V gas giant")]
    GasGiantClass5,
    #[serde(rename = "Water giant")]
    WaterGiant,
    #[serde(rename = "Gas giant with water based life")]
    GasGiantWBL,
    #[serde(rename = "Gas giant with ammonia based life")]
    GasGiantABL,
    #[serde(rename = "Helium rich gas giant")]
    GasGiantHeliumRich,
    #[serde(rename = "Helium gas giant")]
    HeliumGasGiant,
    #[serde(rename = "Water world")]
    WaterWorld,
    #[serde(rename = "Ammonia world")]
    AmmoniaWorld,
    #[serde(rename = "Earthlike body")]
    ELW,

}

#[derive(Deserialize, Default, Eq, PartialEq, Clone)]
pub enum Volcanism {
    #[serde(rename = "silicate vapour geysers volcanism")]
    SilicateVapour,
    #[serde(rename = "minor silicate vapour geysers volcanism")]
    MinorSilicateVapour,
    #[serde(rename = "major silicate vapour geysers volcanism")]
    MajorSilicateVapour,
    #[serde(rename = "water geysers volcanism")]
    WaterGeysers,
    #[serde(rename = "major water geysers volcanism")]
    MajorWaterGeysers,
    #[serde(rename = "minor water geysers volcanism")]
    MinorWaterGeysers,
    #[serde(rename = "water magma volcanism")]
    WaterMagma,
    #[serde(rename = "major water magma volcanism")]
    MajorWaterMagma,
    #[serde(rename = "minor water magma volcanism")]
    MinorWaterMagma,
    #[serde(rename = "rocky magma volcanism")]
    RockyMagma,
    #[serde(rename = "major rocky magma volcanism")]
    MajorRockyMagma,
    #[serde(rename = "minor rocky magma volcanism")]
    MinorRockyMagma,
    #[serde(rename = "carbon dioxide geysers volcanism")]
    CarbonDioxide,
    #[serde(rename = "carbon dioxide geysers volcanism")]
    MajorCarbonDioxide,
    #[serde(rename = "minor carbon dioxide geysers volcanism")]
    MinorCarbonDioxide,
    #[serde(rename = "methane geysers volcanism")]
    Methane,
    #[serde(rename = "minor methane geysers volcanism")]
    MinorMethane,
    #[serde(rename = "Major methane geysers volcanism")]
    MajorMethane,
    #[serde(rename="methane magma volcanism")]
    MethaneMagma,
    #[serde(rename="minor methane magma volcanism")]
    MinorMethaneMagma,
    #[serde(rename="major methane magma volcanism")]
    MajorMethaneMagma,
    #[serde(rename = "nitrogen geysers volcanism")]
    NitrogenGeysers,
    #[serde(rename = "minor nitrogen geysers volcanism")]
    MinorNitrogenGeysers,
    #[serde(rename = "major nitrogen geysers volcanism")]
    MajorNitrogenGeysers,
    #[serde(rename = "nitrogen magma volcanism")]
    NitrogenMagma,
    #[serde(rename = "minor nitrogen magma volcanism")]
    MinorNitrogenMagma,
    #[serde(rename = "major nitrogen magma volcanism")]
    MajorNitrogenMagma,
    #[serde(rename = "ammonia geysers volcanism")]
    Ammonia,
    #[serde(rename = "minor ammonia geysers volcanism")]
    MinorAmmonia,
    #[serde(rename = "major ammonia geysers volcanism")]
    MajorAmmonia,
    #[serde(rename = "ammonia magma volcanism")]
    AmmoniaMagma,
    #[serde(rename = "minor ammonia magma volcanism")]
    MinorAmmoniaMagma,
    #[serde(rename = "major ammonia magma volcanism")]
    MajorAmmoniaMagma,
    #[serde(rename = "metallic magma volcanism")]
    MetallicMagma,
    #[serde(rename = "major metallic magma volcanism")]
    MajorMetallicMagma,
    #[serde(rename = "minor metallic magma volcanism")]
    MinorMetallicMagma,
    Helium,
    Iron,
    #[serde(rename = "")]
    #[default]
    None,
}

impl Volcanism{
    pub(crate) fn is_water(&self) -> bool{
        matches!(self,
            Volcanism::WaterGeysers
            | Volcanism::MinorWaterGeysers
            | Volcanism::MajorWaterGeysers
            | Volcanism::WaterMagma
            | Volcanism::MinorWaterMagma
            | Volcanism::MajorWaterMagma )
    }
    pub(crate) fn is_carbon_dioxide(&self) -> bool{
        matches!(self,
            Volcanism::CarbonDioxide
            | Volcanism::MinorCarbonDioxide
            | Volcanism::MajorCarbonDioxide)
    }
    pub fn is_silicate_vapours(&self) -> bool{
        matches!(self,
            Volcanism::SilicateVapour
            | Volcanism::MinorSilicateVapour
            | Volcanism::MajorSilicateVapour)
    }

    pub fn is_iron(&self) -> bool{
        matches!(self,
            Volcanism::MetallicMagma
            | Volcanism::MinorMetallicMagma
            | Volcanism::MajorMetallicMagma)
    }
    pub fn is_rocky(&self) -> bool{
        matches!(self,
            Volcanism::RockyMagma
            | Volcanism::MinorRockyMagma
            | Volcanism::MajorRockyMagma)
    }
    pub fn is_ammonia(&self) -> bool{
        matches!(self,
            Volcanism::Ammonia
            | Volcanism::MinorAmmonia
            | Volcanism::MajorAmmonia
            | Volcanism::AmmoniaMagma
            | Volcanism::MinorAmmoniaMagma
            | Volcanism::MajorAmmoniaMagma)
    }

    pub(crate) fn is_methane(&self) -> bool {
        matches!(self,
            Volcanism::Methane
            | Volcanism::MinorMethane
            | Volcanism::MajorMethane
            | Volcanism::MethaneMagma
            | Volcanism::MinorMethaneMagma
            | Volcanism::MajorMethaneMagma )
    }
    pub(crate) fn is_nitrogen(&self) -> bool {
        matches!(self,
            Volcanism::NitrogenGeysers
            | Volcanism::MinorNitrogenGeysers
            | Volcanism::MajorNitrogenGeysers
            | Volcanism::NitrogenMagma
            | Volcanism::MinorNitrogenMagma
            | Volcanism::MajorNitrogenMagma )
    }
    
}

#[derive(Debug, Deserialize)]
pub enum BodyParent {
    Ring(u64),
    Star(u64),
    Null(u64),
    Planet(u64),
}

#[derive(Deserialize, Eq, PartialEq, Clone)]
pub enum RawMaterial {
    //Grade 1
    #[serde(rename = "carbon")]
    Carbon,
    #[serde(rename = "iron")]
    Iron,
    #[serde(rename = "lead")]
    Lead,
    #[serde(rename = "nickel")]
    Nickel,
    #[serde(rename = "phosphorus")]
    Phosphorus,
    #[serde(rename = "rhenium")]
    Rhenium,
    #[serde(rename = "sulphur")]
    Sulphur,
    //Grade 2
    #[serde(rename = "arsenic")]
    Arsenic,
    #[serde(rename = "chromium")]
    Chromium,
    #[serde(rename = "germanium")]
    Germanium,
    #[serde(rename = "manganese")]
    Manganese,
    #[serde(rename = "vanadium")]
    Vanadium,
    #[serde(rename = "zinc")]
    Zinc,
    #[serde(rename = "zirconium")]
    Zirconium,
    //Grade 3
    #[serde(rename = "cadmium")]
    Cadmium,
    #[serde(rename = "mercury")]
    Mercury,
    #[serde(rename = "molybdenum")]
    Molybdenum,
    #[serde(rename = "niobium")]
    Niobium,
    #[serde(rename = "tin")]
    Tin,
    #[serde(rename = "tungsten")]
    Tungsten,
    #[serde(rename = "boron")]
    Boron,
    //Grade 4
    #[serde(rename = "antimony")]
    Antimony,
    #[serde(rename = "polonium")]
    Polonium,
    #[serde(rename = "ruthenium")]
    Ruthenium,
    #[serde(rename = "selenium")]
    Selenium,
    #[serde(rename = "technetium")]
    Technetium,
    #[serde(rename = "tellurium")]
    Tellurium,
    #[serde(rename = "yttrium")]
    Yttrium,
}