use serde::Deserialize;
use serde_json::{Result, Value};
use crate::elite_journal_data::events::commander::*;
use crate::elite_journal_data::events::fsd::*;
use crate::elite_journal_data::events::fss::*;
use crate::elite_journal_data::events::game_state::*;
use crate::elite_journal_data::events::inventory::*;
use crate::elite_journal_data::events::missions::*;
use crate::elite_journal_data::events::planetary::*;
use crate::elite_journal_data::events::scans::*;
use crate::elite_journal_data::events::ship::*;
use crate::elite_journal_data::events::shipyard::*;
use crate::elite_journal_data::events::social::*;
use crate::elite_journal_data::events::station::*;

//endregion
#[derive(Deserialize)]
#[serde(tag = "event")]
#[allow(dead_code)]
pub enum EliteEvent {
    Fileheader(FileHeader),
    LoadGame(LoadGame),
    GameModeChange(GameModeChange),
    Location(Location),
    CarrierLocation(CarrierLocation),
    CarrierStats(CarrierStats),
    Commander(Commander),
    Statistics(Statistics),
    Shutdown(Shutdown),
    Music(Music),
    NavRoute(NavRoute),
    NavRouteClear(NavRouteClear),
    Loadout(Loadout),
    SuitLoadout(SuitLoadout),
    ShipLocker(ShipLocker),
    Materials(Materials),
    MaterialCollected(MaterialCollected),
    MaterialDiscovered(MaterialDiscovered),
    MaterialTrade(MaterialTrade),

    //Inventory
    Backpack(Backpack),
    ModuleInfo(ModuleInfo),

    //Scans
    FSSSignalDiscovered(FSSSignalDiscovered),
    FSSDiscoveryScan(FSSDiscoveryScan),
    NavBeaconScan(NavBeaconScan),
    Scan(Scan),
    FSSBodySignals(FSSBodySignals),
    FSSAllBodiesFound(FSSAllBodiesFound),
    SAASignalsFound(SAASignalsFound),
    SAAScanComplete(SAAScanComplete),
    ScanBaryCentre(ScanBaryCentre),

    //Ships Events
    DockingRequested(DockingRequested),
    DockingGranted(DockingGranted),
    ApproachSettlement(ApproachSettlement),
    Docked(Docked),
    Undocked(Undocked),
    Liftoff(Liftoff),
    Touchdown(Touchdown),
    ApproachBody(ApproachBody),
    LeaveBody(LeaveBody),
    RefuelAll(RefuelAll),
    FuelScoop(FuelScoop),
    JetConeBoost(JetConeBoost),
    ReservoirReplenished(ReservoirReplenished),
    ShipTargeted(ShipTargeted),
    ShieldState(ShieldState),
    UnderAttack(UnderAttack),
    Scanned(Scanned),


    //Station related
    Market(Market),
    Shipyard(Shipyard),
    Outfitting(Outfitting),
    StoredShips(StoredShips),
    StoredModules(StoredModules),
    ShipyardTransfer(ShipyardTransfer),
    ShipyardSwap(ShipyardSwap),
    Repair(Repair),
    RepairAll(RepairAll),
    MarketBuy(MarketBuy),
    Embark(Embark),
    Disembark(Disembark),
    LaunchDrone(LaunchDrone),
    RepairDrone(RepairDrone),
    LaunchFighter(LaunchFighter),
    LaunchSRV(LaunchSRV),
    DockSRV(DockSRV),
    FSDTarget(FSDTarget),
    StartJump(StartJump),
    FSDJump(FSDJump),
    SupercruiseEntry(SupercruiseEntry),
    SupercruiseExit(SupercruiseExit),
    SupercruiseDestinationDrop(SupercruiseDestinationDrop),

    //Social
    SendText(SendText),
    ReceiveText(ReceiveText),
    Friends(Friends),
    WingInvite(WingInvite),
    WingJoin(WingJoin),
    WingAdd(WingAdd),
    WingLeave(WingLeave),
    SquadronStartup(SquadronStartup),


    CommunityGoal(CommunityGoal),
    CommunityGoalReward(CommunityGoalReward),
    Rank(Rank),
    Progress(Progress),
    Reputation(Reputation),
    Powerplay(Powerplay),
    Missions(Missions),
    MissionAccepted(MissionAccepted),
    EngineerProgress(EngineerProgress),
    EngineerCraft(EngineerCraft),
    Cargo(Cargo),
    CollectCargo(CollectCargo),
    EjectCargo(EjectCargo),

    ColonisationConstructionDepot(ColonisationConstructionDepot),
    ColonisationContribution(ColonisationContribution),

    CodexEntry(CodexEntry),
    ScanOrganic(ScanOrganic),

    DatalinkScan(DatalinkScan),

    PayFines(PayFines),
}
pub fn parse_logstring(log: String) -> Result<Vec<EliteEvent>> {
    let mut events: Vec<EliteEvent> = Vec::new();
    let loglines = log.lines();
    for line in loglines {
        let event: Result<EliteEvent> = serde_json::from_str(line);
        match event {
            Ok(event) => {
                events.push(event);
            }
            Err(e) => {
                let v: Value = serde_json::from_str(line)?;
                println!(
                    "Failed to parse event (Type {})! - {}",
                    v["event"],
                    e.to_string()
                );
            }
        }
    }
    //println!("Parsed {} lines", events.len());
    Ok(events)
}
