use serde::Deserialize;

#[derive(Deserialize)]
pub enum CommodityType {
    //Refinery/extraction
    #[serde(alias = "$aluminium_name;", alias = "aluminium")]
    Aluminium,
    #[serde(alias = "$copper_name;", alias = "copper")]
    Copper,
    #[serde(alias = "$steel_name;", alias = "steel")]
    Steel,

    //Medical
    #[serde(alias = "$combatstabilisers_name;", alias = "combatstabilisers")]
    Combatstabilisers,

    //
    #[serde(alias = "$advancedcatalysers_name;", alias = "advancedcatalysers")]
    AdvancedCatalysers,
    #[serde(alias = "$buildingfabricators_name;", alias = "buildingfabricators")]
    BuildingFabricators,

    #[serde(alias = "$ceramiccomposites_name;", alias = "ceramiccomposites")]
    CeramicComposites,
    #[serde(alias = "$computercomponents_name;", alias = "computercomponents")]
    ComputerComponents,

    //Millitary
    #[serde(alias = "$battleweapons_name;", alias = "battleweapons")]
    BattleWeapons,

    //Colony

    //Thargoid
    ThargoidPod,
    MetaAlloys,
    ThargoidHeart,
    ThargoidTitanDriveComponent,


}