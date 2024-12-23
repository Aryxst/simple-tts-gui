use piper_rs::synth::PiperSpeechSynthesizer;
use rodio::buffer::SamplesBuffer;
use rodio::Sink;
use std::path::Path;
use std::thread;
use tauri::{AppHandle, Emitter};

#[tauri::command]
pub fn synthesize(
    app: AppHandle,
    config_path: String,
    input: String,
    channels: u16,
    sample_rate: u32,
) {
    println!("Config path: {} | Input: {}", config_path, input);

    thread::spawn(move || {
        app.emit("started-synth", ()).unwrap();
        let model = piper_rs::from_config_path(Path::new(&config_path)).unwrap();
        let synth = PiperSpeechSynthesizer::new(model).unwrap();

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).expect("Sink open failed");

        let mut samples = Vec::<f32>::new();
        let audio = synth.synthesize_parallel(input, None).unwrap();

        for result in audio {
            samples.append(&mut result.unwrap().into_vec());
        }

        sink.append(SamplesBuffer::new(channels, sample_rate, samples));

        sink.sleep_until_end();
        sink.clear();

        app.emit("ended-synth", ()).unwrap();
    });
}
