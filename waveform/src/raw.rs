#[derive(Debug, Clone, PartialEq)]
pub struct RawWaveform {
    /// The number of steps selected for the "Section Pause Duration" slider.
    /// - Range: 0-100
    /// - Default: 0
    pub section_pause_duration_steps: u8,
    /// The "Playback Speed" toggle.
    /// - Values: 1, 2, 4
    pub playback_speed_multiplier: u8,
    /// Unknown field. Always an 8 in my tests. None of the waveform settings seem to effect it.
    pub unknown_3: u8,
    /// The waveform sections.
    /// - Minimum: 1
    /// - Maximum: 10
    pub sections: Vec<RawWaveformSection>,
}

impl RawWaveform {
    pub fn export(&self) -> String {
        format!(
            "Dungeonlab+pulse:{},{},{}={}",
            self.section_pause_duration_steps,
            self.playback_speed_multiplier,
            self.unknown_3,
            self.sections
                .iter()
                .map(RawWaveformSection::export)
                .collect::<Vec<_>>()
                .join("+section+")
        )
    }
    pub fn import<S: AsRef<str>>(data: S) -> Result<Self, &'static str> {
        let data = data
            .as_ref()
            .strip_prefix("Dungeonlab+pulse:")
            .ok_or("Missing prefix")?;

        let (waveform_data, segments_data) = data.split_once("=").ok_or("Expected '='")?;

        let mut waveform_fields = waveform_data.split(',');

        let section_pause_duration_steps = waveform_fields
            .next()
            .ok_or("Missing section pause duration")?
            .parse()
            .map_err(|_| "Failed to parse section pause duration")?;

        let playback_speed_multiplier = waveform_fields
            .next()
            .ok_or("Missing playback speed multiplier")?
            .parse()
            .map_err(|_| "Failed to parse playback speed multiplier")?;

        let unknown_3 = waveform_fields
            .next()
            .ok_or("Missing unknown field 3")?
            .parse()
            .map_err(|_| "Failed to parse unknown field 3")?;

        if waveform_fields.next().is_some() {
            return Err("Unknown extra field");
        }

        let sections = segments_data
            .split("+section+")
            .filter(|s| !s.is_empty())
            .map(RawWaveformSection::import)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            section_pause_duration_steps,
            playback_speed_multiplier,
            unknown_3,
            sections,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawWaveformSection {
    /// The number of steps selected for the upper bound of the "Pulse Frequency" slider.
    /// - Range: 0-100
    /// - Default: 0
    pub pulse_frequency_steps_upper: u8,
    /// The number of steps selected for the lower bound of the "Pulse Frequency" slider.
    /// - Range: 0-100
    /// - Default: 0
    pub pulse_frequency_steps_lower: u8,
    /// The number of steps selected for the "Section Duration" slider.
    /// - Range: 0-100
    /// - Default: 0
    pub section_duration_steps: u8,
    /// The Pulse Mode selection
    /// - Values: 1, 2, 3, 4
    pub pulse_mode: u8,
    /// Indicates if the section is enabled or not.
    /// - Values: 0, 1
    pub enabled: u8,
    /// The section elements.
    /// - Minimum: 2
    /// - Maximum: Unknown
    pub elements: Vec<RawWaveformElement>,
}

impl RawWaveformSection {
    pub fn export(&self) -> String {
        format!(
            "{},{},{},{},{}/{}",
            self.pulse_frequency_steps_upper,
            self.pulse_frequency_steps_lower,
            self.section_duration_steps,
            self.pulse_mode,
            self.enabled,
            self.elements
                .iter()
                .map(RawWaveformElement::export)
                .collect::<Vec<_>>()
                .join(",")
        )
    }
    pub fn import<S: AsRef<str>>(data: S) -> Result<Self, &'static str> {
        let (section_data, elements_data) = data.as_ref().split_once("/").ok_or("Expected '/'")?;

        let mut section_fields = section_data.split(',');

        let pulse_frequency_steps_upper = section_fields
            .next()
            .ok_or("Missing pulse frequency upper")?
            .parse()
            .map_err(|_| "Failed to parse pulse frequency upper")?;

        let pulse_frequency_steps_lower = section_fields
            .next()
            .ok_or("Missing pulse frequency lower")?
            .parse()
            .map_err(|_| "Failed to parse pulse frequency lower")?;

        let section_duration_steps = section_fields
            .next()
            .ok_or("Missing section duration")?
            .parse()
            .map_err(|_| "Failed to parse section duration")?;

        let pulse_mode = section_fields
            .next()
            .ok_or("Missing pulse mode")?
            .parse()
            .map_err(|_| "Failed to parse pulse mode")?;

        let enabled = section_fields
            .next()
            .ok_or("Missing enabled")?
            .parse()
            .map_err(|_| "Failed to parse enabled")?;

        if section_fields.next().is_some() {
            return Err("Unknown extra field");
        }

        let elements = elements_data
            .split(',')
            .filter(|s| !s.is_empty())
            .map(RawWaveformElement::import)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            pulse_frequency_steps_upper,
            pulse_frequency_steps_lower,
            section_duration_steps,
            pulse_mode,
            enabled,
            elements,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawWaveformElement {
    /// The intensity of the element, stored as hundredths of a percent. For example, 34.56% is represented as 3456.
    /// - Range: 0 - 10000 (0.00% - 100.00%)
    /// - Default: 0 (first element) or 10000 (last element)
    pub intensity_percent: u16,
    /// Indicates if the element is fixed or not. Fixed elements are manually selected, while non-fixed elements are automatically smoothed between fixed elements. The first and last elements are always fixed.
    /// - Values: 0, 1
    pub fixed: u8,
}

impl RawWaveformElement {
    pub fn new(intensity_percent: u16, fixed: u8) -> Self {
        Self {
            intensity_percent,
            fixed,
        }
    }

    pub fn export(&self) -> String {
        let whole = self.intensity_percent / 100;
        let fraction = self.intensity_percent % 100;
        format!("{}.{:0>2}-{}", whole, fraction, self.fixed)
    }

    pub fn import<S: AsRef<str>>(import: S) -> Result<Self, &'static str> {
        let mut element_fields = import.as_ref().split('-');

        let intensity_percent = element_fields
            .next()
            .ok_or("Missing intensity")?
            .replace('.', "")
            .parse()
            .map_err(|_| "Failed to parse intensity")?;

        let fixed = element_fields
            .next()
            .ok_or("Missing fixed")?
            .parse()
            .map_err(|_| "Failed to parse fixed")?;

        if element_fields.next().is_some() {
            return Err("Unknown extra field");
        }

        Ok(Self::new(intensity_percent, fixed))
    }

    /// Returns the intensity as a float between 0.0 and 1.0.
    pub fn intensity_float(&self) -> f32 {
        self.intensity_percent as f32 / 10000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import() {
        let import_string = "Dungeonlab+pulse:28,2,8=29,7,24,4,1/0.00-1,50.00-0,100.00-1+section+3,0,16,1,0/0.00-1,100.00-1";
        let imported = RawWaveform::import(import_string);

        let waveform = imported.unwrap();
        assert_eq!(waveform.section_pause_duration_steps, 28);
        assert_eq!(waveform.playback_speed_multiplier, 2);
        assert_eq!(waveform.unknown_3, 8);
        assert_eq!(waveform.sections.len(), 2);

        let section1 = &waveform.sections[0];
        assert_eq!(section1.pulse_frequency_steps_upper, 29);
        assert_eq!(section1.pulse_frequency_steps_lower, 7);
        assert_eq!(section1.section_duration_steps, 24);
        assert_eq!(section1.pulse_mode, 4);
        assert_eq!(section1.enabled, 1);
        assert_eq!(section1.elements.len(), 3);

        let element1 = &section1.elements[0];
        assert_eq!(element1.intensity_percent, 0);
        assert_eq!(element1.fixed, 1);

        let element2 = &section1.elements[1];
        assert_eq!(element2.intensity_percent, 5000);
        assert_eq!(element2.fixed, 0);

        let element3 = &section1.elements[2];
        assert_eq!(element3.intensity_percent, 10000);
        assert_eq!(element3.fixed, 1);

        let section2 = &waveform.sections[1];
        assert_eq!(section2.pulse_frequency_steps_upper, 3);
        assert_eq!(section2.pulse_frequency_steps_lower, 0);
        assert_eq!(section2.section_duration_steps, 16);
        assert_eq!(section2.pulse_mode, 1);
        assert_eq!(section2.enabled, 0);
        assert_eq!(section2.elements.len(), 2);

        let element1 = &section2.elements[0];
        assert_eq!(element1.intensity_percent, 0);
        assert_eq!(element1.fixed, 1);

        let element2 = &section2.elements[1];
        assert_eq!(element2.intensity_percent, 10000);
        assert_eq!(element2.fixed, 1);
    }

    #[test]
    fn test_export() {
        let element1 = RawWaveformElement::new(0, 1);
        let element2 = RawWaveformElement::new(5000, 0);
        let element3 = RawWaveformElement::new(10000, 1);

        let section1 = RawWaveformSection {
            pulse_frequency_steps_upper: 29,
            pulse_frequency_steps_lower: 7,
            section_duration_steps: 24,
            pulse_mode: 4,
            enabled: 1,
            elements: vec![element1.clone(), element2.clone(), element3.clone()],
        };

        let section2 = RawWaveformSection {
            pulse_frequency_steps_upper: 3,
            pulse_frequency_steps_lower: 0,
            section_duration_steps: 16,
            pulse_mode: 1,
            enabled: 0,
            elements: vec![element1.clone(), element3.clone()],
        };

        let waveform = RawWaveform {
            section_pause_duration_steps: 28,
            playback_speed_multiplier: 2,
            unknown_3: 8,
            sections: vec![section1, section2],
        };

        let export_string = waveform.export();
        assert_eq!(export_string, "Dungeonlab+pulse:28,2,8=29,7,24,4,1/0.00-1,50.00-0,100.00-1+section+3,0,16,1,0/0.00-1,100.00-1");
    }

    #[test]
    fn test_export_import() {
        let element1 = RawWaveformElement::new(0, 1);
        let element2 = RawWaveformElement::new(5000, 0);
        let element3 = RawWaveformElement::new(10000, 1);

        let section1 = RawWaveformSection {
            pulse_frequency_steps_upper: 29,
            pulse_frequency_steps_lower: 7,
            section_duration_steps: 24,
            pulse_mode: 4,
            enabled: 1,
            elements: vec![element1.clone(), element2.clone(), element3.clone()],
        };

        let section2 = RawWaveformSection {
            pulse_frequency_steps_upper: 3,
            pulse_frequency_steps_lower: 0,
            section_duration_steps: 16,
            pulse_mode: 1,
            enabled: 0,
            elements: vec![element1.clone(), element3.clone()],
        };

        let waveform = RawWaveform {
            section_pause_duration_steps: 28,
            playback_speed_multiplier: 2,
            unknown_3: 8,
            sections: vec![section1, section2],
        };

        let export_string = waveform.export();
        let imported_waveform = RawWaveform::import(export_string).unwrap();

        assert_eq!(waveform, imported_waveform);
    }

    #[test]
    fn test_export_import_empty_sections() {
        let waveform = RawWaveform {
            section_pause_duration_steps: 0,
            playback_speed_multiplier: 1,
            unknown_3: 8,
            sections: vec![],
        };

        let export_string = waveform.export();
        let imported_waveform = RawWaveform::import(export_string).unwrap();

        assert_eq!(waveform, imported_waveform);
    }

    #[test]
    fn test_export_import_single_section() {
        let element = RawWaveformElement::new(10000, 1);

        let section = RawWaveformSection {
            pulse_frequency_steps_upper: 50,
            pulse_frequency_steps_lower: 25,
            section_duration_steps: 75,
            pulse_mode: 2,
            enabled: 1,
            elements: vec![element.clone()],
        };

        let waveform = RawWaveform {
            section_pause_duration_steps: 10,
            playback_speed_multiplier: 4,
            unknown_3: 8,
            sections: vec![section],
        };

        let export_string = waveform.export();
        let imported_waveform = RawWaveform::import(export_string).unwrap();

        assert_eq!(waveform, imported_waveform);
    }

    #[test]
    fn test_import_invalid_prefix() {
        let import_string = "InvalidPrefix:28,2,8=29,7,24,4,1/0.00-1,50.00-0,100.00-1+section+3,0,16,1,0/0.00-1,100.00-1";
        let result = RawWaveform::import(import_string);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Missing prefix"));
    }

    #[test]
    fn test_import_missing_field() {
        let import_string = "Dungeonlab+pulse:28,2=29,7,24,4,1/0.00-1,50.00-0,100.00-1+section+3,0,16,1,0/0.00-1,100.00-1";
        let result = RawWaveform::import(import_string);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Missing unknown field 3"));
    }

    #[test]
    fn test_import_invalid_field() {
        let import_string = "Dungeonlab+pulse:28,2,invalid=29,7,24,4,1/0.00-1,50.00-0,100.00-1+section+3,0,16,1,0/0.00-1,100.00-1";
        let result = RawWaveform::import(import_string);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Failed to parse unknown field 3"));
    }

    #[test]
    fn test_import_extra_field() {
        let import_string = "Dungeonlab+pulse:28,2,8,extra=29,7,24,4,1/0.00-1,50.00-0,100.00-1+section+3,0,16,1,0/0.00-1,100.00-1";
        let result = RawWaveform::import(import_string);
        assert!(result.is_err());
        assert_eq!(result.err(), Some("Unknown extra field"));
    }
}
