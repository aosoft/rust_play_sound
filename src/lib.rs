use std::f32::consts::PI;

#[derive(Copy, Clone)]
pub enum NoteName {
    C = 0,
    Csh = 1,
    D = 2,
    Dsh = 3,
    E = 4,
    F = 5,
    Fsh = 6,
    G = 7,
    Gsh = 8,
    A = 9,
    Ash = 10,
    B = 11,
}

pub struct Note {
    octave: i32,
    note: NoteName,
}

const BASE_PITCH: f32 = 440.0;
const BASE_NOTE: Note = Note { octave: 4, note: NoteName::A };

#[derive(Copy, Clone)]
pub struct Time(f32);

#[derive(Copy, Clone)]
pub struct Pitch(f32);

#[derive(Copy, Clone)]
pub struct Wave(f32);

impl Note {
    pub fn new(octave: i32, note: NoteName) -> Note {
        Note { octave: octave, note: note }
    }

    pub fn note_index(&self) -> i32 {
        self.octave * 12 + self.note as i32
    }

    pub fn pitch(&self) -> Pitch {
        Pitch(BASE_PITCH * 2.0_f32.powf(((self.note_index() - BASE_NOTE.note_index()) as f32) / 12.0))
    }
}


impl Time {
    pub fn samples_to_time(sample_rate: u32, samples: u32) -> Time {
        Time(samples as f32 / sample_rate as f32)
    }

    pub fn per_cycle(self, pitch: Pitch) -> f32 {
        let x = self.0 * pitch.0;
        x - x.floor()
    }
}

impl Pitch {
    pub fn sine_wave(self, time: Time) -> Wave {
        Wave((2.0 * PI * time.per_cycle(self)).sin())
    }

    pub fn square_wave(self, time: Time) -> Wave {
        Wave(1.0 - time.per_cycle(self).round() * 2.0)
    }
}

impl Wave {
    pub fn to_value(self) -> f32 { self.0 }

    pub fn volume(self, vol: f32) -> Wave { Wave(self.0 * vol) }
}

impl std::ops::Add for Wave {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output
    {
        Wave(self.0 + rhs.0)
    }
}