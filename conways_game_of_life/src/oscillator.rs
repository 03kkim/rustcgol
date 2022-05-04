/* 

code credit: WoldSound(Youtube)

"Wavetable Synthesis in Rust?? Rust Basics Tutorial [Synth #004]"
https://www.youtube.com/watch?v=v0Qp7eWVyes

*/

use std::time::Duration;
use rodio::Source;

pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WavetableOscillator {
        WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    pub fn lerp(&self) -> f32 {
        let truncated_index: usize = self.index as usize;
        let next_index: usize = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index] + 
                next_index_weight * self.wave_table[next_index];
    }

    pub fn init_sine_oscillator() -> WavetableOscillator {
        let wave_table_size = 64;
        let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

        for n in 0..wave_table_size {
            wave_table.push((2.0 * std::f32::consts::PI * n as f32 / 
                wave_table_size as f32).sin());
        }

        WavetableOscillator {
            sample_rate: 44100,
            wave_table: wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }
}

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        return Some(self.get_sample());
    }
}

impl Source for WavetableOscillator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}