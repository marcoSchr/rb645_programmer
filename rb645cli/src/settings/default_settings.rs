use crate::settings::{
    BatterySave, BeepTone, ScanMode, Settings, SideKeyFunction, Squelch, TimeOutTimer,
    VoiceAnnunciation, VoxDelayTimes, VoxLevel,
};

pub fn default_settings() -> Settings {
    Settings {
        squelch: Squelch::Level3,
        time_out_timer: TimeOutTimer::Off,
        vox: VoxLevel::Off,
        vox_delay_time: VoxDelayTimes::HalfSecond,
        scan_mode: ScanMode::Carrier,
        voice_annunciation: VoiceAnnunciation::English,
        side_key_1_long: SideKeyFunction::Off,
        side_key_2_long: SideKeyFunction::Off,
        battery_save: BatterySave::On,
        beep_tone: BeepTone::On,
        _unknown5: 0,
        _unknown6: 0,
        _unknown7: 0,
        _unknown8: 255,
        _unknown9: 255,
        _unknown10: 255,
        data: Vec::new(),
    }
}
