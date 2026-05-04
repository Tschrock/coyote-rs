use std::{sync::LazyLock, time::Duration};

use super::{
    raw::{RawWaveform, RawWaveformElement, RawWaveformSection},
    slider::SliderCalc,
};

#[derive(Debug)]
pub enum PlaybackSpeed {
    Speed1X,
    Speed2X,
    Speed4X,
}

impl From<PlaybackSpeed> for u8 {
    fn from(speed: PlaybackSpeed) -> u8 {
        match speed {
            PlaybackSpeed::Speed1X => 1,
            PlaybackSpeed::Speed2X => 2,
            PlaybackSpeed::Speed4X => 4,
        }
    }
}

impl TryFrom<u8> for PlaybackSpeed {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PlaybackSpeed::Speed1X),
            2 => Ok(PlaybackSpeed::Speed2X),
            4 => Ok(PlaybackSpeed::Speed4X),
            _ => Err("Invalid playback speed multiplier"),
        }
    }
}

#[derive(Debug)]
pub enum WaveformMode {
    Mode1,
    Mode2,
    Mode3,
    Mode4,
}

impl From<WaveformMode> for u8 {
    fn from(mode: WaveformMode) -> u8 {
        match mode {
            WaveformMode::Mode1 => 1,
            WaveformMode::Mode2 => 2,
            WaveformMode::Mode3 => 3,
            WaveformMode::Mode4 => 4,
        }
    }
}

impl TryFrom<u8> for WaveformMode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(WaveformMode::Mode1),
            2 => Ok(WaveformMode::Mode2),
            3 => Ok(WaveformMode::Mode3),
            4 => Ok(WaveformMode::Mode4),
            _ => Err("Invalid pulse mode"),
        }
    }
}

#[derive(Debug)]
pub struct Waveform {
    /// The amount of time to pause between each section.
    pub pause_duration: Duration,
    /// Playback speed multiplier
    pub playback_speed: PlaybackSpeed,
    /// Unknown field 3
    pub unknown_3: u8,
    /// Waveform sections
    pub sections: Vec<WaveformSection>,
}

impl Default for Waveform {
    fn default() -> Self {
        Self {
            pause_duration: Duration::from_secs(0),
            playback_speed: PlaybackSpeed::Speed1X,
            unknown_3: 8,
            sections: vec![WaveformSection::default()],
        }
    }
}

impl Waveform {
    pub fn import<S: AsRef<str>>(data: S) -> Result<Self, &'static str> {
        RawWaveform::import(data)?.try_into()
    }
}

impl TryFrom<RawWaveform> for Waveform {
    type Error = &'static str;
    fn try_from(raw: RawWaveform) -> Result<Self, Self::Error> {
        Ok(Self {
            pause_duration: steps_to_pause_duration(raw.section_pause_duration_steps),
            playback_speed: raw.playback_speed_multiplier.try_into()?,
            unknown_3: raw.unknown_3,
            sections: raw
                .sections
                .into_iter()
                .map(WaveformSection::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<Waveform> for RawWaveform {
    fn from(waveform: Waveform) -> Self {
        RawWaveform {
            section_pause_duration_steps: pause_duration_to_steps(waveform.pause_duration),
            playback_speed_multiplier: waveform.playback_speed.into(),
            unknown_3: waveform.unknown_3,
            sections: waveform.sections.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug)]
pub struct WaveformSection {
    pub enabled: bool,
    pub frequency_top: Duration,
    pub frequency_bottom: Duration,
    pub mode: WaveformMode,
    pub duration: Duration,
    pub elements: Vec<WaveformElement>,
}

impl Default for WaveformSection {
    fn default() -> Self {
        Self {
            enabled: false,
            frequency_top: Duration::from_millis(10),
            frequency_bottom: Duration::from_secs(0),
            mode: WaveformMode::Mode1,
            duration: Duration::from_millis(100),
            elements: vec![
                WaveformElement::new(0.00, true),
                WaveformElement::new(100.00, true),
            ],
        }
    }
}

impl TryFrom<RawWaveformSection> for WaveformSection {
    type Error = &'static str;
    fn try_from(raw: RawWaveformSection) -> Result<Self, Self::Error> {
        Ok(Self {
            enabled: raw.enabled != 0,
            frequency_top: steps_to_pulse_frequency(raw.pulse_frequency_steps_upper),
            frequency_bottom: steps_to_pulse_frequency(raw.pulse_frequency_steps_lower),
            mode: raw.pulse_mode.try_into()?,
            duration: steps_to_section_duration(raw.section_duration_steps),
            elements: raw
                .elements
                .into_iter()
                .map(WaveformElement::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl From<WaveformSection> for RawWaveformSection {
    fn from(section: WaveformSection) -> Self {
        RawWaveformSection {
            pulse_frequency_steps_upper: pulse_frequency_to_steps(section.frequency_top),
            pulse_frequency_steps_lower: pulse_frequency_to_steps(section.frequency_bottom),
            section_duration_steps: section_duration_to_steps(section.duration),
            pulse_mode: section.mode.into(),
            enabled: if section.enabled { 1 } else { 0 },
            elements: section.elements.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug)]
pub struct WaveformElement {
    pub fixed: bool,
    pub intensity_percent: u16,
}

impl WaveformElement {
    pub fn new(percentage: f32, fixed: bool) -> Self {
        Self {
            fixed,
            intensity_percent: (percentage * 10000.0) as u16,
        }
    }
}

impl TryFrom<RawWaveformElement> for WaveformElement {
    type Error = &'static str;
    fn try_from(raw: RawWaveformElement) -> Result<Self, Self::Error> {
        Ok(Self {
            fixed: raw.fixed != 0,
            intensity_percent: raw.intensity_percent,
        })
    }
}

impl From<WaveformElement> for RawWaveformElement {
    fn from(element: WaveformElement) -> Self {
        RawWaveformElement {
            fixed: if element.fixed { 1 } else { 0 },
            intensity_percent: element.intensity_percent,
        }
    }
}

static PULSE_FREQ_CONVERSION: LazyLock<SliderCalc> = LazyLock::new(|| {
    SliderCalc::new(10, 1)
        .push(50, 2)
        .push(80, 5)
        .push(100, 10)
        .push(200, 33)
        .push(400, 50)
        .push(600, 100)
});

static SECTION_DURATION_CONVERSION: LazyLock<SliderCalc> = LazyLock::new(|| {
    SliderCalc::new(1, 1)
        .push(50, 2)
        .push(80, 5)
        .push(100, 10)
        .push(200, 33)
        .push(400, 50)
        .push(600, 100)
        .push(1000, 200)
        .push(2000, 500)
});

/// Converts the Pulse Frequency slider steps into a Duration.
fn steps_to_pulse_frequency(steps: u8) -> Duration {
    Duration::from_millis(PULSE_FREQ_CONVERSION.steps_to_value(steps))
}

/// Converts a Duration into Pulse Frequency slider steps.
fn pulse_frequency_to_steps(duration: Duration) -> u8 {
    PULSE_FREQ_CONVERSION.value_to_steps(duration.as_millis() as u64)
}

/// Converts the Section Duration slider steps into a Duration.
fn steps_to_section_duration(steps: u8) -> Duration {
    Duration::from_millis(SECTION_DURATION_CONVERSION.steps_to_value(steps))
}

/// Converts a Duration into Section Duration slider steps.
fn section_duration_to_steps(duration: Duration) -> u8 {
    SECTION_DURATION_CONVERSION.value_to_steps(duration.as_millis() as u64)
}

/// Converts the Pause Duration slider steps into a Duration.
fn steps_to_pause_duration(steps: u8) -> Duration {
    let deciseconds = ((steps - 1) / 10) + 1;
    Duration::from_millis(deciseconds as u64 * 100)
}

/// Converts a Duration into Pause Duration slider steps.
fn pause_duration_to_steps(duration: Duration) -> u8 {
    let deciseconds = duration.as_millis() / 100;
    deciseconds as u8 * 10
}
