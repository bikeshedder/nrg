//! The registers are defined in the
//! "P30 Charging Station Modbus TCP Programmers Guide V 1.04"
//! https://www.keba.com/download/x/dea7ae6b84/kecontactp30modbustcp_pgen.pdf
#![allow(dead_code)]

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use strum::{AsRefStr, Display};

use crate::modbus::{Register, Type};

// Readable
pub const CHARGING_STATE: Register<ChargingState> = Register::new("charging_state", 1000);
pub const CABLE_STATE: Register<u32> = Register::new("cable_state", 1004);
pub const ERROR_CODE: Register<u32> = Register::new("cable_state", 1006);
pub const CHARGING_CURRENT_PHASE_1: Register<u32> = Register::new("charging_current_phase_1", 1008);
pub const CHARGING_CURRENT_PHASE_2: Register<u32> = Register::new("charging_current_phase_2", 1010);
pub const CHARGING_CURRENT_PHASE_3: Register<u32> = Register::new("charging_current_phase_3", 1012);
pub const SERIAL_NUMBER: Register<u32> = Register::new("serial_number", 1014);
pub const PRODUCT_TYPE_AND_FEATURES: Register<u32> =
    Register::new("product_type_and_features", 1016);
pub const FIRMWARE_VERSION: Register<u32> = Register::new("firmware_version", 1018);
pub const ACTIVE_POWER: Register<u32> = Register::new("active_power", 1020);
pub const TOTAL_ENERGY: Register<u32> = Register::new("total_energy", 1036);
pub const VOLTAGE_PHASE_1: Register<u32> = Register::new("voltage_phase_1", 1040);
pub const VOLTAGE_PHASE_2: Register<u32> = Register::new("voltage_phase_2", 1042);
pub const VOLTAGE_PHASE_3: Register<u32> = Register::new("voltage_phase_3", 1044);
pub const POWER_FACTOR: Register<u32> = Register::new("power_factor", 1046);
pub const MAX_CHARGING_CURRENT: Register<u32> = Register::new("max_charging_current", 1100);
pub const MAX_SUPPORTED_CURRENT: Register<u32> = Register::new("max_supported_current", 1110);
pub const RFID_CARD: Register<u32> = Register::new("rfid_card", 1500);
pub const CHARGED_ENERGY: Register<u32> = Register::new("charged_eneryg", 1502);
pub const PHASE_SWITCHING_SOURCE: Register<u32> = Register::new("phase_switching_source", 1550);
pub const PHASE_SWITCHING_STATE: Register<u32> = Register::new("phase_switching_state", 1552);
pub const FAILSAFE_CURRENT_SETTING: Register<u32> = Register::new("failsafe_current_setting", 1600);
pub const FAILSAFE_TIMEOUT_SETTING: Register<u32> = Register::new("failsafe_timeout_setting", 1602);

// Writable
pub const SET_CHARGING_CURRENT: Register<u16> = Register::new("set_charging_current", 5004);
pub const SET_ENERGY: Register<u16> = Register::new("set_energy", 5010);
pub const UNLOCK_PLUG: Register<u16> = Register::new("unlock_plug", 5012);
pub const ENABLE_CHARGING_STATION: Register<u16> = Register::new("enable_charging_station", 5014);
pub const SET_PHASE_SWITCH_TOGGLE: Register<u16> = Register::new("set_phase_switch_toggle", 5050);
pub const TRIGGER_PHASE_SWITCH: Register<u16> = Register::new("trigger_phase_switch", 5052);
pub const FAILSAFE_CURRENT: Register<u16> = Register::new("failsafe_current", 5016);
pub const FAILSAFE_TIMEOUT: Register<u16> = Register::new("failsafe_timeout", 5018);
pub const FAILSAFE_PERSIST: Register<u16> = Register::new("failsafe_persist", 5020);

/// This register contains the state of the charging station.
#[derive(Copy, Clone, FromPrimitive, Display, AsRefStr)]
#[repr(u32)]
pub enum ChargingState {
    /// 0: Start-up of the charging station
    StartUp = 0,
    /// The charging station is not ready for charging. The charging
    /// station is not connected to an electric vehicle, it is locked
    /// by the authorization function or another mechanism.
    NotReady = 1,
    /// The charging station is ready for charging and waits for a
    /// reaction from the electric vehicle
    Ready = 2,
    /// A charging process is active.
    Active = 3,
    /// An error has occurred.
    Error = 4,
    /// The charging process is temporarily interrupted because the
    /// temperature is too high or the wallbox is in suspended mode.
    Suspended = 5,
}

impl Type for ChargingState {
    const LEN: u16 = 2;
    fn decode(data: &[u16]) -> Option<Self> {
        u32::decode(data).and_then(ChargingState::from_u32)
    }
    fn encode(&self) -> Box<[u16]> {
        (*self as u32).encode()
    }
}
