use portaudio::{PortAudio, Error};
use std::f32::consts::PI;
use rust_play_sound::{ Note, NoteName, Wave, Time };

const SAMPLE_RATE: f32 = 48000.0;

fn main() {
    match main2() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn main2() -> Result<(), Error> {
    let pa = PortAudio::new()?;
    let mut settings = pa.default_output_stream_settings(2, SAMPLE_RATE as f64, 48000)?;
    settings.flags = portaudio::stream_flags::CLIP_OFF;

    let mut sample_number:i32 = 0;
    let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut i = 0;
        for _ in 0..frames {
            let time = Time::samples_to_time(SAMPLE_RATE,sample_number);
            let value =
                (Wave::square_wave(Note::new(4, NoteName::C).pitch(), time) +
                 Wave::square_wave(Note::new(4, NoteName::E).pitch(), time) +
                 Wave::square_wave(Note::new(4, NoteName::G).pitch(), time)).volume(0.3);
            buffer[i] = value.to_value();
            buffer[i + 1] = value.to_value();

            i += 2;
            sample_number += 1;
        }

        portaudio::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;
    pa.sleep(3 * 1_000);
    stream.stop()?;
    stream.close()?;

    Ok(())
}