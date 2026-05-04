/// The output settings for a single waveform frame.
/// A frame lasts for 250ms, and consists of a frequency and intensity value.
pub struct WaveformFrame {
    /// The output frequency. Valid values: 10 - 240
    pub frequency: u8,

    pub intensity: u8,
}

/// A clip of four waveform frames.
pub type WaveformClip = [WaveformFrame; 4];

pub enum IntensityChange {
    Ignore,
    Increase(u8),
    Decrease(u8),
    Absolute(u8),
}

pub struct ChannelOutput {
    pub intensity: IntensityChange,
    pub waveform: WaveformClip,
}

pub struct OutputCommand {
    pub serial_number: u8,
    pub channel_a: ChannelOutput,
    pub channel_b: ChannelOutput,
}

pub struct ChannelSettings {
    pub intensity_limit: u8,
    pub frequency_balance: u8,
    pub intensity_balance: u8,
}

pub struct SettingsCommand {
    pub channel_a: ChannelSettings,
    pub channel_b: ChannelSettings,
}

pub struct OutputNotification {
    pub serial_number: u8,
    pub channel_a_intensity: u8,
    pub channel_b_intensity: u8,
}

pub struct SettingsNotification {
    pub channel_a: ChannelSettings,
    pub channel_b: ChannelSettings,
}

pub struct Frequency(u8);

impl Frequency {
    pub const MIN: u8 = 10;
    pub const MAX: u8 = 240;
    pub fn new(value: u8) -> Option<Self> {
        if value < Self::MIN || value > Self::MAX {
            None
        } else {
            Some(Frequency(value))
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.0
    }

    pub fn as_percent(&self) -> f32 {
        (self.0 - Self::MIN) as f32 / (Self::MAX - Self::MIN) as f32
    }
}
