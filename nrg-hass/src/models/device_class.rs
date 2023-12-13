use serde::Serialize;

/// https://www.home-assistant.io/integrations/sensor/#device-class
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceClass {
    // Apparent power in VA.
    ApparentPower,
    /// Air Quality Index (unitless).
    Aqi,
    /// Atmospheric pressure in cbar, bar, hPa, inHg, kPa, mbar, Pa or psi
    AtmosphericPressure,
    /// Percentage of battery that is left in %
    Battery,
    /// Carbon Dioxide in CO2 (Smoke) in ppm
    CarbonDioxide,
    /// Carbon Monoxide in CO (Gas CNG/LPG) in ppm
    CarbonMonoxide,
    /// Current in A, mA
    Current,
    /// Data rate in bit/s, kbit/s, Mbit/s, Gbit/s, B/s, kB/s, MB/s, GB/s, KiB/s, MiB/s or GiB/s
    DataRate,
    /// Data size in bit, kbit, Mbit, Gbit, B, kB, MB, GB, TB, PB, EB, ZB, YB, KiB, MiB, GiB, TiB, PiB, EiB, ZiB or YiB
    DataSize,
    /// Date string (ISO 8601)
    Date,
    /// Generic distance in km, m, cm, mm, mi, yd, or in
    Distance,
    /// Duration in d, h, min, or s
    Duration,
    /// Energy in Wh, kWh, MWh, MJ, or GJ
    Energy,
    /// Stored energy in Wh, kWh, MWh, MJ, or GJ
    EnergyStorage,
    /// Has a limited set of (non-numeric) states
    Enum,
    /// Frequency in Hz, kHz, MHz, or GHz
    Frequency,
    /// Gasvolume in m³, ft³ or CCF
    Gas,
    /// Percentage of humidity in the air in %
    Humidity,
    /// The current light level in lx
    Illuminance,
    /// Irradiance in W/m² or BTU/(h⋅ft²)
    Irradiance,
    /// Percentage of water in a substance in %
    Moisture,
    /// The monetary value (ISO 4217)
    Monetary,
    /// Concentration of Nitrogen Dioxide in µg/m³
    NitrogenDioxide,
    /// Concentration of Nitrogen Monoxide in µg/m³
    NitrogenMonoxide,
    /// Concentration of Nitrous Oxide in µg/m³
    NitrousOxide,
    /// Concentration of Ozone in µg/m³
    Ozone,
    /// Concentration of particulate matter less than 1 micrometer in µg/m³
    Pm1,
    /// Concentration of particulate matter less than 2.5 micrometers in µg/m³
    Pm25,
    /// Concentration of particulate matter less than 10 micrometers in µg/m³
    Pm10,
    /// Power factor (unitless), unit may be None or %
    PowerFactor,
    /// Power in W or kW
    Power,
    /// Accumulated precipitation in cm, in or mm
    Precipitation,
    /// Precipitation intensity in in/d, in/h, mm/d or mm/h
    PrecipitationIntensity,
    /// Pressure in Pa, kPa, hPa, bar, cbar, mbar, mmHg, inHg or psi
    Pressure,
    /// Reactive power in var
    ReactivePower,
    /// Signal strength in dB or dBm
    SignalStrength,
    /// Sound pressure in dB or dBA
    SoundPressure,
    /// Generic speed in ft/s, in/d, in/h, km/h, kn, m/s, mph or mm/d
    Speed,
    /// Concentration of sulphur dioxide in µg/m³
    SulphurDioxide,
    /// Temperature in °C, °F or K
    Temperature,
    /// Datetime object or timestamp string (ISO 8601)
    Timestamp,
    /// Concentration of volatile organic compounds in µg/m³
    VolatileOrganicCompounds,
    /// Ratio of volatile organic compounds in ppm or ppb
    VolatileOrganicCompoundsParts,
    /// Voltage in V, mV
    Voltage,
    /// Generic volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    Volume,
    /// Generic stored volume in L, mL, gal, fl. oz., m³, ft³, or CCF
    VolumeStorage,
    /// Water consumption in L, gal, m³, ft³, or CCF
    Water,
    /// Generic mass in kg, g, mg, µg, oz, lb, or st
    Weight,
    /// Wind speed in ft/s, km/h, kn, m/s, or mph
    WindSpeed,
}
