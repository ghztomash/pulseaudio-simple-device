use psimple::Simple;
use pulse::sample::{Format, Spec};
use pulse::stream::Direction;

use std::sync::mpsc;
use std::thread;

use crate::stream::Stream;

pub struct Device {
    name: String,
    spec: Spec,
}

impl Device {
    pub fn new(name: String, spec: Spec) -> Self {
        Device { name, spec }
    }

    pub fn build_output_stream<T, E>(&mut self, data_callback: T, error_callback: E) -> Stream {
        let (thread_channel_sender, thread_channel_receiver) = mpsc::channel();

        let spec = Spec {
            format: Format::F32le,
            channels: 2,
            rate: 44100,
        };
        assert!(spec.is_valid());

        let s = Simple::new(
            None,                // Use the default server
            "FooApp",            // Our application’s name
            Direction::Playback, // We want a playback stream
            None,                // Use the default device
            "samples",           // Description of our stream
            &spec,               // Our sample format
            None,                // Use default channel map
            None,                // Use default buffering attributes
        )
        .unwrap();

        let audio_thread = thread::spawn(move || loop {
            thread_channel_receiver.recv().unwrap();
            println!("Audio thread received message");
        });

        Stream::new(Some(audio_thread), Some(thread_channel_sender))
    }
}
