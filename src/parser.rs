use serde::Deserialize;
use serde_json::{Result, Value};
use std::io::prelude::*;
use crate::elite_events::events::*;

//endregion


#[derive(Deserialize)]
#[serde(tag = "event")]
pub enum EliteEvent{
    LoadGame(LoadGame),
    Shutdown(Shutdown),
    Music(Music),
    Loadout(Loadout),
    SuitLoadout(SuitLoadout),
    ShipLocker(ShipLocker),
    Backpack(Backpack),
    FSSSignalDiscovered(FSSSignalDiscovered),
    FSSDiscoveryScan(FSSDiscoveryScan),
    DockingRequested(DockingRequested),
    DockingGranted(DockingGranted),
    Docked(Docked),
    Undocked(Undocked),
    RefuelAll(RefuelAll),
    Shipyard(Shipyard),
    StoredShips(StoredShips),
    ShipyardTransfer(ShipyardTransfer),
    Embark(Embark),
    Disembark(Disembark),
    FSDTarget(FSDTarget),
    StartJump(StartJump),
    FSDJump(FSDJump),
    SuperCruiseEntry(SuperCruiseEntry),
    SupercruiseExit(SupercruiseExit),
    SupercruiseDestinationDrop(SupercruiseDestinationDrop),
    ReceiveText(ReceiveText),
    Friends(Friends),
    WingLeave(WingLeave),
    CommunityGoal(CommunityGoal),
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
                let v: Value = serde_json::from_str(line).unwrap();
                println!("Failed to parse event (Type {})! : {}", v["event"], e);
            }
        }
    }
    println!("Parsed {} lines", events.len());
    Ok(events)
}
