pub mod tts {
    use piper_rs::synth::PiperSpeechSynthesizer;
    use rodio::buffer::SamplesBuffer;
    use rodio::Sink;
    use std::path::Path;
    use std::thread;

    #[tauri::command]
    pub fn synthesize(config_path: String, input: String, channels: u16, sample_rate: u32) {
        println!("Config path: {} | Input: {}", config_path, input);
        thread::spawn(move || {
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
        });
    }
}

pub mod fs_extras {
    use serde::Serialize;
    use std::path::Path;
    use walkdir::WalkDir;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct FileInfo {
        path: String,
        is_file: bool,
        size: Option<u64>,
    }

    #[tauri::command]
    pub fn list_files(path: String) -> Result<Vec<FileInfo>, String> {
        // Check if path exists
        if !Path::new(&path).exists() {
            return Err(format!("Path does not exist: {}", path));
        }

        let mut files = Vec::new();
        // Iterate over entries in the provided path
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.file_type().is_dir())
        {
            let metadata = entry.metadata().map_err(|err| err.to_string())?;

            files.push(FileInfo {
                path: entry.path().display().to_string(),
                is_file: metadata.is_file(),
                size: Some(metadata.len()),
            });
        }
        Ok(files)
    }
}
