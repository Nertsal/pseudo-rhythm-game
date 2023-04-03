use super::*;

#[derive(Clone, Debug)]
pub struct Beat {
    frequency: f32,
    duration: f32,
    num_sample: usize,
}

impl Beat {
    pub fn new(frequency: f32, duration: f32) -> Self {
        Self {
            frequency,
            duration,
            num_sample: 0,
        }
    }
}

impl Default for Beat {
    fn default() -> Self {
        Self::new(100.0, 0.3)
    }
}

impl Iterator for Beat {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);
        let t = self.num_sample as f32 / 48000.0 * 1.0;
        if t > self.duration {
            return None;
        }
        let freq_t = 1.0 - t / self.duration / 2.0;
        let ampl = (freq_t * 2.0 - 1.0).min(0.5) / 0.5;
        let freq = self.frequency * freq_t;
        let value = freq * 2.0 * std::f32::consts::PI * t;
        Some(ampl * value.sin())
    }
}

impl Source for Beat {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
