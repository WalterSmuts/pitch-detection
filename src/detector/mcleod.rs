use crate::detector::internals::normalized_square_difference;
use crate::detector::internals::pitch_from_peaks;
use crate::detector::internals::DetectorInternals;
use crate::detector::internals::Pitch;
use crate::detector::PitchDetector;
use crate::float::Float;
use crate::utils::buffer::square_sum;
use crate::utils::peak::PeakCorrection;

pub struct McLeodDetector<T>
where
    T: Float + std::iter::Sum,
{
    internals: DetectorInternals<T>,
}

impl<T> McLeodDetector<T>
where
    T: Float + std::iter::Sum,
{
    pub fn new(size: usize, padding: usize) -> Self {
        let internals = DetectorInternals::new(size, padding);
        McLeodDetector { internals }
    }
}

impl<T> PitchDetector<T> for McLeodDetector<T>
where
    T: Float + std::iter::Sum,
{
    fn get_pitch(
        &mut self,
        signal: &[T],
        sample_rate: usize,
        power_threshold: T,
        clarity_threshold: T,
    ) -> Option<Pitch<T>> {
        assert_eq!(signal.len(), self.internals.size);

        if square_sum(signal) < power_threshold {
            return None;
        }
        let result = &mut self.internals.buffers.get_real_buffer();

        normalized_square_difference(signal, &self.internals.buffers, result);
        pitch_from_peaks(
            result,
            sample_rate,
            clarity_threshold,
            PeakCorrection::Quadratic,
        )
    }
}
