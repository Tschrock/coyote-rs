/// Helper struct for calculating slider values based on defined steps - used for calculating the true values for Pulse Frequency and Section Duration sliders.
#[derive(Debug)]
pub struct SliderCalc {
    ranges: Vec<SliderRange>,
}

impl SliderCalc {
    pub fn new(start_value: u64, value_inc: u64) -> Self {
        Self {
            ranges: vec![SliderRange::new(0, start_value, value_inc)],
        }
    }
    pub fn push(mut self, value: u64, inc: u64) -> Self {
        let step = match self.ranges.last() {
            Some(last) => last.step + (value - last.value).div_ceil(last.inc) as u8,
            None => 0,
        };
        self.ranges.push(SliderRange::new(step, value, inc));
        self
    }
    pub fn steps_to_value(&self, steps: u8) -> u64 {
        for range in self.ranges.iter().rev() {
            if steps >= range.step {
                return range.value + ((steps - range.step) as u64 * range.inc);
            }
        }
        0
    }
    pub fn value_to_steps(&self, value: u64) -> u8 {
        for range in self.ranges.iter().rev() {
            if value >= range.value {
                return range.step + ((value - range.value) / range.inc) as u8;
            }
        }
        0
    }
}

#[derive(Debug)]
pub struct SliderRange {
    step: u8,
    value: u64,
    inc: u64,
}

impl SliderRange {
    pub fn new(step: u8, value: u64, inc: u64) -> Self {
        Self { step, value, inc }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_calc_new() {
        let slider = SliderCalc::new(10, 2);
        assert_eq!(slider.ranges.len(), 1);
        assert_eq!(slider.ranges[0].step, 0);
        assert_eq!(slider.ranges[0].value, 10);
        assert_eq!(slider.ranges[0].inc, 2);
    }

    #[test]
    fn test_slider_calc_push() {
        let slider = SliderCalc::new(10, 2).push(18, 3).push(26, 1);
        assert_eq!(slider.ranges.len(), 3);
        assert_eq!(slider.ranges[1].step, 4);
        assert_eq!(slider.ranges[1].value, 18);
        assert_eq!(slider.ranges[1].inc, 3);
        assert_eq!(slider.ranges[2].step, 7);
        assert_eq!(slider.ranges[2].value, 26);
        assert_eq!(slider.ranges[2].inc, 1);
    }

    #[test]
    fn test_steps_to_value() {
        let slider = SliderCalc::new(10, 2).push(18, 3).push(26, 1);
        assert_eq!(slider.steps_to_value(0), 10);
        assert_eq!(slider.steps_to_value(1), 12);
        assert_eq!(slider.steps_to_value(2), 14);
        assert_eq!(slider.steps_to_value(3), 16);
        assert_eq!(slider.steps_to_value(4), 18);
        assert_eq!(slider.steps_to_value(5), 21);
        assert_eq!(slider.steps_to_value(6), 24);
        assert_eq!(slider.steps_to_value(7), 26);
        assert_eq!(slider.steps_to_value(8), 27);
        assert_eq!(slider.steps_to_value(9), 28);
    }

    #[test]
    fn test_value_to_steps() {
        let slider = SliderCalc::new(10, 2).push(18, 3).push(26, 1);
        assert_eq!(slider.value_to_steps(0), 0);
        assert_eq!(slider.value_to_steps(1), 0);
        assert_eq!(slider.value_to_steps(2), 0);
        assert_eq!(slider.value_to_steps(3), 0);
        assert_eq!(slider.value_to_steps(4), 0);
        assert_eq!(slider.value_to_steps(5), 0);
        assert_eq!(slider.value_to_steps(6), 0);
        assert_eq!(slider.value_to_steps(7), 0);
        assert_eq!(slider.value_to_steps(8), 0);
        assert_eq!(slider.value_to_steps(9), 0);
        assert_eq!(slider.value_to_steps(10), 0);
        assert_eq!(slider.value_to_steps(11), 0);
        assert_eq!(slider.value_to_steps(12), 1);
        assert_eq!(slider.value_to_steps(13), 1);
        assert_eq!(slider.value_to_steps(14), 2);
        assert_eq!(slider.value_to_steps(15), 2);
        assert_eq!(slider.value_to_steps(16), 3);
        assert_eq!(slider.value_to_steps(17), 3);
        assert_eq!(slider.value_to_steps(18), 4);
        assert_eq!(slider.value_to_steps(19), 4);
        assert_eq!(slider.value_to_steps(20), 4);
        assert_eq!(slider.value_to_steps(21), 5);
        assert_eq!(slider.value_to_steps(22), 5);
        assert_eq!(slider.value_to_steps(23), 5);
        assert_eq!(slider.value_to_steps(24), 6);
        assert_eq!(slider.value_to_steps(25), 6);
        assert_eq!(slider.value_to_steps(26), 7);
        assert_eq!(slider.value_to_steps(27), 8);
        assert_eq!(slider.value_to_steps(28), 9);
        assert_eq!(slider.value_to_steps(29), 10);
    }
}
