macro_rules! impl_sql_via_json {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl rusqlite::types::ToSql for $ty {
                fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
                    let s = serde_json::to_string(self)
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
                    // serde_json renders a fieldless variant as a bare JSON string, e.g. "K_OrangeGiant"
                    Ok(rusqlite::types::ToSqlOutput::from(s.trim_matches('"').to_string()))
                }
            }

            impl rusqlite::types::FromSql for $ty {
                fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
                    let s = value.as_str()?;
                    serde_json::from_str(&format!("\"{s}\""))
                        .map_err(|e| rusqlite::types::FromSqlError::Other(e.into()))
                }
            }
        )+
    };
}

use crate::elite_journal_data::enums::body_data::*;
use crate::elite_journal_data::enums::system_data::*;
use crate::elite_journal_data::enums::signals::*;
use crate::elite_journal_data::enums::misc::*;
use crate::elite_journal_data::enums::station_data::*;
use crate::elite_journal_data::enums::vessels::*;
use crate::elite_journal_data::enums::cargo::*;
use crate::elite_journal_data::enums::game_data::*;
use crate::elite_journal_data::enums::exobiology::*;

impl_sql_via_json!(
    StarClass, LuminosityClass, AtmosphereType, PlanetClass, Volcanism, Geological, BodyType,
    Economy, SystemSecurity, FactionState, Government, PowerplayState, PowerplayPower,
    SignalType, SAASignalType,
    JumpType, FriendStatus, Allegiance, LegalStatus, PilotRank, ScanType, MaterialCategory,
    StationType, StationService, EngineerUnlockedStatus,
    ShipType, SRVType, SlotType, LimpetType,
    CommodityType, GameMode,
    Genus, Species, ExoBiologyVariant,
);