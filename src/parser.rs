use serde::Deserialize;
use serde_json::{Result, Value};
use std::io::prelude::*;
use crate::elite_events::events::*;

//endregion
#[derive(Deserialize)]
#[serde(tag = "event")]
pub enum EliteEvent{
    Fileheader(FileHeader),
    LoadGame(LoadGame),
    Location(Location),
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
    Backpack(Backpack),
    FSSSignalDiscovered(FSSSignalDiscovered),
    FSSDiscoveryScan(FSSDiscoveryScan),
    Scan(Scan),
    FSSBodySignals(FSSBodySignals),
    FSSAllBodiesFound(FSSAllBodiesFound),
    SAASignalsFound(SAASignalsFound),
    SAAScanComplete(SAAScanComplete),
    ScanBaryCentre(ScanBaryCentre),
    DockingRequested(DockingRequested),
    DockingGranted(DockingGranted),
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
    Shipyard(Shipyard),
    StoredShips(StoredShips),
    ShipyardTransfer(ShipyardTransfer),
    ShipyardSwap(ShipyardSwap),
    Embark(Embark),
    Disembark(Disembark),
    LaunchFighter(LaunchFighter),
    DockSRV(DockSRV),
    FSDTarget(FSDTarget),
    StartJump(StartJump),
    FSDJump(FSDJump),
    SupercruiseEntry(SupercruiseEntry),
    SupercruiseExit(SupercruiseExit),
    SupercruiseDestinationDrop(SupercruiseDestinationDrop),
    ReceiveText(ReceiveText),
    Friends(Friends),
    WingInvite(WingInvite),
    WingAdd(WingAdd),
    WingLeave(WingLeave),
    SquadronStartup(SquadronStartup),
    CommunityGoal(CommunityGoal),
    Rank(Rank),
    Progress(Progress),
    Reputation(Reputation),
    Powerplay(Powerplay),
    Missions(Missions),
    EngineerProgress(EngineerProgress),
    Cargo(Cargo),
    CodexEntry(CodexEntry),
    ScanOrganic(ScanOrganic),

}
pub fn parse_logstring(log: String) -> Result<Vec<EliteEvent>> {
    let mut events: Vec<EliteEvent> = Vec::new();
    let loglines = log.lines();
    for line in loglines {
        let event:Result<EliteEvent>= serde_json::from_str(line);
        match event
        {
            Ok(event) => {
                events.push(event);
            }
            Err(e) => {
                let v: Value = serde_json::from_str(line)?;
                println!("Failed to parse event (Type {})! - {}", v["event"], e.to_string());
            }
        }
    }
    //println!("Parsed {} lines", events.len());
    Ok(events)
}
