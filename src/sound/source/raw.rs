use super::*;

#[derive(Clone)]
pub struct RawSource<T: rodio::Sample = f32> {
    data: std::sync::Arc<Vec<T>>,
    sample_rate: u32,
    num_sample: usize,
}

impl<T: rodio::Sample> RawSource<T> {
    pub fn new(sample_rate: u32, data: Vec<T>) -> Self {
        Self {
            data: std::sync::Arc::new(data),
            sample_rate,
            num_sample: 0,
        }
    }

    pub fn get_frame(&self) -> &[T] {
        if self.num_sample >= self.data.len() {
            return &self.data[0..0];
        }
        &self.data[self.num_sample..]
    }
}

impl<T: rodio::Sample> Iterator for RawSource<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.data.get(self.num_sample).copied();
        self.num_sample += 1;
        sample
    }
}

impl<T: rodio::Sample> Source for RawSource<T> {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub trait IntoRawSource {
    type Sample: rodio::Sample;
    fn into_raw_source(self) -> RawSource<Self::Sample>;
}

impl<T: Source> IntoRawSource for T
where
    <T as Iterator>::Item: rodio::Sample,
{
    type Sample = T::Item;
    fn into_raw_source(self) -> RawSource<T::Item> {
        RawSource::new(self.sample_rate(), self.collect())
    }
}
