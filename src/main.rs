use portaudio::{PortAudio, Error};
use rust_play_sound::{ Note, NoteName, Time };

const SAMPLE_RATE: u32 = 48000;

fn main() {
    match main2() {
        Ok(_) => {},
        Err(e) => eprintln!("{:?}", e),
    }
}

fn main2() -> Result<(), Error> {
    let pa = PortAudio::new()?;
    let mut settings = pa.default_output_stream_settings(2, SAMPLE_RATE as f64, SAMPLE_RATE)?;
    settings.flags = portaudio::stream_flags::CLIP_OFF;

    let mut sample_number:u32 = 0;
    let callback = move |args: portaudio::OutputStreamCallbackArgs<f32>| {
        let mut i = 0;
        for _ in 0..args.frames {
            let time = Time::samples_to_time(SAMPLE_RATE,sample_number);
            let value =
                (Note::new(4, NoteName::C).pitch().square_wave(time) +
                 Note::new(4, NoteName::E).pitch().square_wave(time) +
                 Note::new(4, NoteName::G).pitch().square_wave(time)).volume(0.3);
            args.buffer[i] = value.to_value();
            args.buffer[i + 1] = value.to_value();

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