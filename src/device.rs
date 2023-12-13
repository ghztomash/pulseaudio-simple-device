use anyhow::{Error, Result};
use psimple::Simple;
use pulse::sample::{Format, Spec};
use pulse::stream::Direction;

use std::sync::mpsc;
use std::thread;

use crate::config::Config;
use crate::stream::{Command, Stream};

pub struct Device {
    application_name: String,
}

impl Device {
    pub fn new(application_name: String) -> Self {
        Device { application_name }
    }

    pub fn build_output_stream<D, E>(
        &self,
        config: &Config,
        mut data_callback: D,
        mut error_callback: E,
    ) -> Result<Stream>
    where
        D: FnMut(&mut [f32]) + Send + 'static,
        E: FnMut(Error) + Send + 'static,
    {
        let (thread_channel_tx, thread_channel_rx) = mpsc::channel();

        let spec = Spec {
            format: Format::F32le,
            channels: config.channels,
            rate: config.sample_rate,
        };
        assert!(spec.is_valid());

        let s = Simple::new(
            None,                   // Use the default server
            &self.application_name, // Our application’s name
            Direction::Playback,    // We want a playback stream
            None,                   // Use the default device
            "samples",              // Description of our stream
            &spec,                  // Our sample format
            None,                   // Use default channel map
            None,                   // Use default buffering attributes
        )?;

        let mut buffer = vec![0f32; config.buffer_size];

        let audio_thread = thread::spawn(move || {
            println!("audio_thread started!");

            let mut paused = true;
            let mut data;

            loop {
                // loop until we get an exit signal
                match thread_channel_rx.try_recv() {
                    Ok(Command::Quit) | Err(mpsc::TryRecvError::Disconnected) => {
                        break;
                    }
                    Ok(Command::Play) => {
                        paused = false;
                    }
                    Ok(Command::Pause) => {
                        paused = true;
                    }
                    Err(_) => {}
                }

                if paused {
                    buffer.fill(0f32);
                } else {
                    data_callback(&mut buffer);
                }

                // convert the buffer to a byte array
                data = buffer
                    .iter()
                    .flat_map(|v| v.to_le_bytes().to_vec())
                    .collect::<Vec<u8>>();

                // write the data to the stream
                if let Err(err) = s.write(&data) {
                    error_callback(err.into());
                }
            }

            // drain the stream
            if let Err(err) = s.drain() {
                error_callback(err.into());
            }

            println!("audio_thread done!");
        });

        Ok(Stream::new(Some(audio_thread), Some(thread_channel_tx)))
    }
}
