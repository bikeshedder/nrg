use serde::Serialize;

// https://github.com/home-assistant/core/blob/master/homeassistant/const.py#L384
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum UnitOfMeasurement {
    // Apparent power units
    #[serde(rename = "VA")]
    VoltAmpere,

    // Power units
    #[serde(rename = "W")]
    Watt,
    #[serde(rename = "kW")]
    KiloWatt,
    #[serde(rename = "BTU/h")]
    BtuPerHour,

    // Energy units
    #[serde(rename = "kWh")]
    KiloWattHours,
    #[serde(rename = "MWh")]
    MegaWattHours,
    #[serde(rename = "Wh")]
    WattHours,

    // Electric_current units
    #[serde(rename = "mA")]
    MilliAmpere,
    #[serde(rename = "A")]
    Ampere,

    // Electric_potential units
    #[serde(rename = "mV")]
    MilliVolt,
    #[serde(rename = "V")]
    Volt,

    // Degree units
    #[serde(rename = "°")]
    Degree,

    // Currency units
    #[serde(rename = "€")]
    Euro,
    #[serde(rename = "$")]
    Dollar,
    #[serde(rename = "¢")]
    Cent,

    // Temperature units
    #[serde(rename = "°C")]
    TempCelsius,
    #[serde(rename = "°F")]
    TempFahrenheit,
    #[serde(rename = "K")]
    TempKelvin,

    // Time units
    #[serde(rename = "μs")]
    Microseconds,
    #[serde(rename = "ms")]
    Milliseconds,
    #[serde(rename = "s")]
    Seconds,
    #[serde(rename = "min")]
    Minutes,
    #[serde(rename = "h")]
    Hours,
    #[serde(rename = "d")]
    Days,
    #[serde(rename = "w")]
    Weeks,
    #[serde(rename = "m")]
    Months,
    #[serde(rename = "y")]
    Years,

    // Length units
    #[serde(rename = "mm")]
    Millimeters,
    #[serde(rename = "cm")]
    Centimeters,
    #[serde(rename = "m")]
    Meters,
    #[serde(rename = "km")]
    Kilometers,
    #[serde(rename = "in")]
    Inches,
    #[serde(rename = "ft")]
    Feet,
    #[serde(rename = "yd")]
    Yard,
    #[serde(rename = "mi")]
    Miles,

    // Frequency units
    #[serde(rename = "Hz")]
    Hertz,
    #[serde(rename = "kHz")]
    Kilohertz,
    #[serde(rename = "MHz")]
    Megahertz,
    #[serde(rename = "GHz")]
    Gigahertz,

    // Pressure units
    #[serde(rename = "Pa")]
    Pa,
    #[serde(rename = "hPa")]
    Hpa,
    #[serde(rename = "kPa")]
    Kpa,
    #[serde(rename = "bar")]
    Bar,
    #[serde(rename = "cbar")]
    Cbar,
    #[serde(rename = "mbar")]
    Mbar,
    #[serde(rename = "mmHg")]
    Mmhg,
    #[serde(rename = "inHg")]
    Inhg,
    #[serde(rename = "psi")]
    Psi,

    // Sound pressure units
    #[serde(rename = "dB")]
    Decibel,
    #[serde(rename = "dBA")]
    WeightedDecibelA,

    // Volume units
    #[serde(rename = "ft³")]
    CubicFeet,
    #[serde(rename = "CCF")]
    CentumCubicFeet,
    #[serde(rename = "m³")]
    CubicMeters,
    #[serde(rename = "L")]
    Liters,
    #[serde(rename = "mL")]
    Milliliters,
    #[serde(rename = "gal")]
    Gallons,
    #[serde(rename = "fl. oz.")]
    FluidOunces,

    // Volume Flow Rate units
    #[serde(rename = "m³/h")]
    CubicMetersPerHour,
    #[serde(rename = "ft³/m")]
    CubicFeetPerMinute,

    // Area units
    #[serde(rename = "m²")]
    SquareMeters,

    // Mass units
    #[serde(rename = "g")]
    Grams,
    #[serde(rename = "kg")]
    Kilograms,
    #[serde(rename = "mg")]
    Milligrams,
    #[serde(rename = "µg")]
    Micrograms,
    #[serde(rename = "oz")]
    Ounces,
    #[serde(rename = "lb")]
    Pounds,
    #[serde(rename = "st")]
    Stones,

    // Conductivity units
    #[serde(rename = "µS/cm")]
    Conductivity,

    // Light units
    #[serde(rename = "lx")]
    Lux,

    // UV Index units
    #[serde(rename = "UV index")]
    UvIndex,

    // Percentage units
    #[serde(rename = "%")]
    Percentage,

    // Rotational speed units
    #[serde(rename = "rpm")]
    RevolutionsPerMinute,

    // Irradiance units
    #[serde(rename = "W/m²")]
    WattsPerSquareMeter,
    #[serde(rename = "BTU/(h⋅ft²)")]
    BtusPerHourSquareFoot,

    // Volumetric flux
    #[serde(rename = "in/d")]
    InchesPerDay,
    #[serde(rename = "in/h")]
    InchesPerHour,
    #[serde(rename = "mm/d")]
    MillimetersPerDay,
    #[serde(rename = "mm/h")]
    MillimetersPerHour,

    // Concentration units
    #[serde(rename = "µg/m³")]
    MicrogramsPreCubicMeter,
    #[serde(rename = "mg/m³")]
    MilligramsPerCubicMeter,
    #[serde(rename = "μg/ft³")]
    MicrogramsPerCubicFoot,
    #[serde(rename = "p/m³")]
    PartsPerCubicMeter,
    #[serde(rename = "ppm")]
    PartsPerMillion,
    #[serde(rename = "ppb")]
    PartsPerBillion,

    // Speed units
    #[serde(rename = "ft/s")]
    FeetPerSecond,
    #[serde(rename = "m/s")]
    MetersPerSecond,
    #[serde(rename = "km/h")]
    KilometersPerHour,
    #[serde(rename = "kn")]
    Knots,
    #[serde(rename = "mph")]
    MilesPerHour,

    // Signal_strength units
    //#[serde(rename = "dB")]
    //Decibel,
    #[serde(rename = "dBm")]
    DecibelsMilliwatt,

    // Data units
    #[serde(rename = "bit")]
    Bits,
    #[serde(rename = "kbit")]
    Kilobits,
    #[serde(rename = "Mbit")]
    Megabits,
    #[serde(rename = "Gbit")]
    Gigabits,
    #[serde(rename = "B")]
    Bytes,
    #[serde(rename = "kB")]
    Kilobytes,
    #[serde(rename = "MB")]
    Megabytes,
    #[serde(rename = "GB")]
    Gigabytes,
    #[serde(rename = "TB")]
    Terabytes,
    #[serde(rename = "PB")]
    Petabytes,
    #[serde(rename = "EB")]
    Exabytes,
    #[serde(rename = "ZB")]
    Zettabytes,
    #[serde(rename = "YB")]
    Yottabytes,
    #[serde(rename = "KiB")]
    Kibibytes,
    #[serde(rename = "MiB")]
    Mebibytes,
    #[serde(rename = "GiB")]
    Gibibytes,
    #[serde(rename = "TiB")]
    Tebibytes,
    #[serde(rename = "PiB")]
    Pebibytes,
    #[serde(rename = "EiB")]
    Exbibytes,
    #[serde(rename = "ZiB")]
    Zebibytes,
    #[serde(rename = "YiB")]
    Yobibytes,

    // Data rate units
    #[serde(rename = "bit/s")]
    BitsPerSecond,
    #[serde(rename = "kbit/s")]
    KiloBitsPerSecond,
    #[serde(rename = "Mbit/s")]
    MegaBitsPerSecond,
    #[serde(rename = "Gbit/s")]
    GigaBitsPerSecond,
    #[serde(rename = "B/s")]
    BytesPerSecond,
    #[serde(rename = "kB/s")]
    KilobytesPerSecond,
    #[serde(rename = "MB/s")]
    MegabytesPerSecond,
    #[serde(rename = "GB/s")]
    GigabytesPerSecond,
    #[serde(rename = "KiB/s")]
    KibibytesPerSecond,
    #[serde(rename = "MiB/s")]
    MebibytesPerSecond,
    #[serde(rename = "GiB/s")]
    GibibytesPerSecond,
}
