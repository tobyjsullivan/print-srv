#[derive(Copy, Clone, Debug)]
pub enum PrinterState {
    Idle = 0x03,
    Processing = 0x04,
    Stopped = 0x05,
}

#[derive(Copy, Clone, Debug)]
pub struct PrinterStateReason {
    pub keyword: PrinterStateReasonKeyword,
    pub severity: Option<PrinterStateReasonSeverity>,
}

impl From<PrinterStateReason> for String {
    fn from(reason: PrinterStateReason) -> Self {
        let severity = match reason.severity {
            Some(sev) => format!("-{}", Into::<String>::into(sev)),
            None => String::from(""),
        };
        format!("{}{}", Into::<String>::into(reason.keyword), severity)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PrinterStateReasonSeverity {
    Report,
    Warning,
    Error,
}

impl From<PrinterStateReasonSeverity> for String {
    fn from(sev: PrinterStateReasonSeverity) -> Self {
        match sev {
            PrinterStateReasonSeverity::Report => String::from("report"),
            PrinterStateReasonSeverity::Warning => String::from("warning"),
            PrinterStateReasonSeverity::Error => String::from("error"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PrinterStateReasonKeyword {
    None,
    Other,
    ConnectingToDevice,
    CoverOpen,
    DeveloperEmpty,
    DeveloperLow,
    DoorOpen,
    FuserOverTemp,
    FuserUnderTemp,
    InputTrayMissing,
    InterlockOpen,
    InterpreterResourceUnavailable,
    MarkerSupplyEmpty,
    MarkerSupplyLow,
    MarkerWasteAlmostFull,
    MarkerWasteFull,
    MediaEmpty,
    MediaJam,
    MediaLow,
    MediaNeeded,
    MovingToPaused,
    OpcLifeOver,
    OpcNearEol,
    OutputAreaAlmostFull,
    OutputAreaFull,
    OutputTrayMissing,
    Paused,
    Shutdown,
    SpoolAreaFull,
    StoppedPartly,
    Stopping,
    TimedOut,
    TonerEmpty,
    TonerLow,
}

impl From<PrinterStateReasonKeyword> for String {
    fn from(kw: PrinterStateReasonKeyword) -> Self {
        match kw {
            PrinterStateReasonKeyword::None => String::from("none"),
            PrinterStateReasonKeyword::Other => String::from("other"),
            PrinterStateReasonKeyword::ConnectingToDevice => String::from("connecting-to-device"),
            PrinterStateReasonKeyword::CoverOpen => String::from("cover-open"),
            PrinterStateReasonKeyword::DeveloperEmpty => String::from("developer-empty"),
            PrinterStateReasonKeyword::DeveloperLow => String::from("developer-low"),
            PrinterStateReasonKeyword::DoorOpen => String::from("door-open"),
            PrinterStateReasonKeyword::FuserOverTemp => String::from("fuser-over-temp"),
            PrinterStateReasonKeyword::FuserUnderTemp => String::from("fuser-under-temp"),
            PrinterStateReasonKeyword::InputTrayMissing => String::from("input-tray-missing"),
            PrinterStateReasonKeyword::InterlockOpen => String::from("interlock-open"),
            PrinterStateReasonKeyword::InterpreterResourceUnavailable => String::from("interpreter-resource-unavailable"),
            PrinterStateReasonKeyword::MarkerSupplyEmpty => String::from("marker-supply-empty"),
            PrinterStateReasonKeyword::MarkerSupplyLow => String::from("marker-supply-low"),
            PrinterStateReasonKeyword::MarkerWasteAlmostFull => String::from("marker-waste-almost-full"),
            PrinterStateReasonKeyword::MarkerWasteFull => String::from("marker-waste-full"),
            PrinterStateReasonKeyword::MediaEmpty => String::from("media-empty"),
            PrinterStateReasonKeyword::MediaJam => String::from("media-jam"),
            PrinterStateReasonKeyword::MediaLow => String::from("media-low"),
            PrinterStateReasonKeyword::MediaNeeded => String::from("media-needed"),
            PrinterStateReasonKeyword::MovingToPaused => String::from("moving-to-paused"),
            PrinterStateReasonKeyword::OpcLifeOver => String::from("opc-life-over"),
            PrinterStateReasonKeyword::OpcNearEol => String::from("opc-near-eol"),
            PrinterStateReasonKeyword::OutputAreaAlmostFull => String::from("output-area-almost-full"),
            PrinterStateReasonKeyword::OutputAreaFull => String::from("output-area-full"),
            PrinterStateReasonKeyword::OutputTrayMissing => String::from("output-tray-missing"),
            PrinterStateReasonKeyword::Paused => String::from("paused"),
            PrinterStateReasonKeyword::Shutdown => String::from("shutdown"),
            PrinterStateReasonKeyword::SpoolAreaFull => String::from("spool-area-full"),
            PrinterStateReasonKeyword::StoppedPartly => String::from("stopped-partly"),
            PrinterStateReasonKeyword::Stopping => String::from("stopping"),
            PrinterStateReasonKeyword::TimedOut => String::from("timed-out"),
            PrinterStateReasonKeyword::TonerEmpty => String::from("toner-empty"),
            PrinterStateReasonKeyword::TonerLow => String::from("toner-low"),
        }
    }
}