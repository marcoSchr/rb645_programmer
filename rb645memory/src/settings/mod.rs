pub mod default_settings;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Settings {
    pub squelch: Squelch,
    pub time_out_timer: TimeOutTimer,
    pub vox: VoxLevel,
    pub vox_delay_time: VoxDelayTimes,
    pub scan_mode: ScanMode,
    pub voice_annunciation: VoiceAnnunciation,
    pub side_key_1_long: SideKeyFunction,
    pub side_key_2_long: SideKeyFunction,
    pub battery_save: BatterySave,
    pub beep_tone: BeepTone,
    pub _unknown5: u8,
    pub _unknown6: u8,
    pub _unknown7: u8,
    pub _unknown8: u8,
    pub _unknown9: u8,
    pub _unknown10: u8,
    pub data: Vec<u8>,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Squelch {
    Level0,
    Level1,
    Level2,
    #[default]
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Unknown(u8),
}

impl From<u8> for Squelch {
    fn from(value: u8) -> Self {
        match value {
            0 => Squelch::Level0,
            1 => Squelch::Level1,
            2 => Squelch::Level2,
            3 => Squelch::Level3,
            4 => Squelch::Level4,
            5 => Squelch::Level5,
            6 => Squelch::Level6,
            7 => Squelch::Level7,
            8 => Squelch::Level8,
            9 => Squelch::Level9,
            x => Squelch::Unknown(x),
        }
    }
}

impl From<Squelch> for u8 {
    fn from(value: Squelch) -> Self {
        match value {
            Squelch::Level0 => 0,
            Squelch::Level1 => 1,
            Squelch::Level2 => 2,
            Squelch::Level3 => 3,
            Squelch::Level4 => 4,
            Squelch::Level5 => 5,
            Squelch::Level6 => 6,
            Squelch::Level7 => 7,
            Squelch::Level8 => 8,
            Squelch::Level9 => 9,
            Squelch::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum TimeOutTimer {
    #[default]
    Off,
    ThirtySeconds,
    OneMinute,
    OneAndAHalfMinutes,
    TwoMinutes,
    TwoAndAHalfMinutes,
    ThreeMinutes,
    Unknown(u8),
}

impl From<u8> for TimeOutTimer {
    fn from(value: u8) -> Self {
        match value {
            0 => TimeOutTimer::Off,
            1 => TimeOutTimer::ThirtySeconds,
            2 => TimeOutTimer::OneMinute,
            3 => TimeOutTimer::OneAndAHalfMinutes,
            4 => TimeOutTimer::TwoMinutes,
            5 => TimeOutTimer::TwoAndAHalfMinutes,
            6 => TimeOutTimer::ThreeMinutes,
            x => TimeOutTimer::Unknown(x),
        }
    }
}

impl From<TimeOutTimer> for u8 {
    fn from(value: TimeOutTimer) -> Self {
        match value {
            TimeOutTimer::Off => 0,
            TimeOutTimer::ThirtySeconds => 1,
            TimeOutTimer::OneMinute => 2,
            TimeOutTimer::OneAndAHalfMinutes => 3,
            TimeOutTimer::TwoMinutes => 4,
            TimeOutTimer::TwoAndAHalfMinutes => 5,
            TimeOutTimer::ThreeMinutes => 6,
            TimeOutTimer::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum VoxLevel {
    #[default]
    Off,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
    Unknown(u8),
}

impl From<u8> for VoxLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => VoxLevel::Off,
            1 => VoxLevel::Level1,
            2 => VoxLevel::Level2,
            3 => VoxLevel::Level3,
            4 => VoxLevel::Level4,
            5 => VoxLevel::Level5,
            6 => VoxLevel::Level6,
            7 => VoxLevel::Level7,
            8 => VoxLevel::Level8,
            9 => VoxLevel::Level9,
            x => VoxLevel::Unknown(x),
        }
    }
}

impl From<VoxLevel> for u8 {
    fn from(value: VoxLevel) -> Self {
        match value {
            VoxLevel::Off => 0,
            VoxLevel::Level1 => 1,
            VoxLevel::Level2 => 2,
            VoxLevel::Level3 => 3,
            VoxLevel::Level4 => 4,
            VoxLevel::Level5 => 5,
            VoxLevel::Level6 => 6,
            VoxLevel::Level7 => 7,
            VoxLevel::Level8 => 8,
            VoxLevel::Level9 => 9,
            VoxLevel::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum VoxDelayTimes {
    #[default]
    HalfSecond,
    OneSecond,
    OneAndAHalfSeconds,
    TwoSeconds,
    TwoAndAHalfSeconds,
    ThreeSeconds,
    Unknown(u8),
}

impl From<u8> for VoxDelayTimes {
    fn from(value: u8) -> Self {
        match value {
            0 => VoxDelayTimes::HalfSecond,
            1 => VoxDelayTimes::OneSecond,
            2 => VoxDelayTimes::OneAndAHalfSeconds,
            3 => VoxDelayTimes::TwoSeconds,
            4 => VoxDelayTimes::TwoAndAHalfSeconds,
            5 => VoxDelayTimes::ThreeSeconds,
            x => VoxDelayTimes::Unknown(x),
        }
    }
}

impl From<VoxDelayTimes> for u8 {
    fn from(value: VoxDelayTimes) -> Self {
        match value {
            VoxDelayTimes::HalfSecond => 0,
            VoxDelayTimes::OneSecond => 1,
            VoxDelayTimes::OneAndAHalfSeconds => 2,
            VoxDelayTimes::TwoSeconds => 3,
            VoxDelayTimes::TwoAndAHalfSeconds => 4,
            VoxDelayTimes::ThreeSeconds => 5,
            VoxDelayTimes::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum ScanMode {
    #[default]
    Carrier,
    Time,
    Unknown(u8),
}

impl From<u8> for ScanMode {
    fn from(value: u8) -> Self {
        match value {
            0 => ScanMode::Carrier,
            1 => ScanMode::Time,
            x => ScanMode::Unknown(x),
        }
    }
}

impl From<ScanMode> for u8 {
    fn from(value: ScanMode) -> Self {
        match value {
            ScanMode::Carrier => 0,
            ScanMode::Time => 1,
            ScanMode::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum VoiceAnnunciation {
    #[default]
    Off,
    Chinese,
    English,
    Unknown(u8),
}

impl From<u8> for VoiceAnnunciation {
    fn from(value: u8) -> Self {
        match value {
            0 => VoiceAnnunciation::Off,
            1 => VoiceAnnunciation::Chinese,
            2 => VoiceAnnunciation::English,
            x => VoiceAnnunciation::Unknown(x),
        }
    }
}

impl From<VoiceAnnunciation> for u8 {
    fn from(value: VoiceAnnunciation) -> Self {
        match value {
            VoiceAnnunciation::Off => 0,
            VoiceAnnunciation::Chinese => 1,
            VoiceAnnunciation::English => 2,
            VoiceAnnunciation::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum SideKeyFunction {
    #[default]
    Off,
    ChannelBroadcast,
    Monitor,
    Scan,
    VoiceSwitch,
    EmergencyAlarm,
    Unknown(u8),
}

impl From<u8> for SideKeyFunction {
    fn from(value: u8) -> Self {
        match value {
            0 => SideKeyFunction::Off,
            1 => SideKeyFunction::ChannelBroadcast,
            2 => SideKeyFunction::Monitor,
            3 => SideKeyFunction::Scan,
            4 => SideKeyFunction::VoiceSwitch,
            5 => SideKeyFunction::EmergencyAlarm,
            x => SideKeyFunction::Unknown(x),
        }
    }
}

impl From<SideKeyFunction> for u8 {
    fn from(value: SideKeyFunction) -> Self {
        match value {
            SideKeyFunction::Off => 0,
            SideKeyFunction::ChannelBroadcast => 1,
            SideKeyFunction::Monitor => 2,
            SideKeyFunction::Scan => 3,
            SideKeyFunction::VoiceSwitch => 4,
            SideKeyFunction::EmergencyAlarm => 5,
            SideKeyFunction::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum BatterySave {
    #[default]
    On,
    Off,
    Unknown(u8),
}

impl From<u8> for BatterySave {
    fn from(value: u8) -> Self {
        match value {
            0 => BatterySave::Off,
            1 => BatterySave::On,
            x => BatterySave::Unknown(x),
        }
    }
}

impl From<BatterySave> for u8 {
    fn from(value: BatterySave) -> Self {
        match value {
            BatterySave::Off => 0,
            BatterySave::On => 1,
            BatterySave::Unknown(x) => x,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum BeepTone {
    #[default]
    On,
    Off,
    Unknown(u8),
}

impl From<u8> for BeepTone {
    fn from(value: u8) -> Self {
        match value {
            0 => BeepTone::Off,
            1 => BeepTone::On,
            x => BeepTone::Unknown(x),
        }
    }
}

impl From<BeepTone> for u8 {
    fn from(value: BeepTone) -> Self {
        match value {
            BeepTone::Off => 0,
            BeepTone::On => 1,
            BeepTone::Unknown(x) => x,
        }
    }
}

impl TryFrom<&[u8]> for Settings {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            beep_tone: (value[0] & 0x01).into(),
            battery_save: ((value[0] & 0x02) >> 1).into(),
            voice_annunciation: ((value[0] & 0x0C) >> 2).into(),
            scan_mode: ((value[0] & 0x40) >> 6).into(),
            squelch: value[1].into(),
            time_out_timer: (value[2] & 0x0F).into(),
            vox_delay_time: ((value[2] & 0xF0) >> 4).into(),
            vox: value[3].into(),
            side_key_1_long: ((value[4] & 0xF0) >> 4).into(),
            side_key_2_long: (value[4] & 0x0F).into(),
            _unknown5: value[5],
            _unknown6: value[6],
            _unknown7: value[7],
            _unknown8: value[8],
            _unknown9: value[9],
            _unknown10: value[10],
            data: value.to_vec(),
        })
    }
}

impl From<Settings> for Vec<u8> {
    fn from(value: Settings) -> Self {
        vec![
            (<BeepTone as Into<u8>>::into(value.beep_tone) & 0x01)
                | ((<BatterySave as Into<u8>>::into(value.battery_save) & 0x01) << 1)
                | ((<VoiceAnnunciation as Into<u8>>::into(value.voice_annunciation) & 0x03) << 2)
                | ((<ScanMode as Into<u8>>::into(value.scan_mode) & 0x01) << 4),
            value.squelch.into(),
            (<TimeOutTimer as Into<u8>>::into(value.time_out_timer) & 0x0F)
                | ((<VoxDelayTimes as Into<u8>>::into(value.vox_delay_time) & 0x0F) << 4),
            value.vox.into(),
            (<SideKeyFunction as Into<u8>>::into(value.side_key_2_long) & 0x0F)
                | ((<SideKeyFunction as Into<u8>>::into(value.side_key_1_long) & 0x0F) << 4),
            value._unknown5,
            value._unknown6,
            value._unknown7,
            value._unknown8,
            value._unknown9,
            value._unknown10,
        ]
    }
}
