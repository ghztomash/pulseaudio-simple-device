use anyhow::{Error, Result};
use std::sync::mpsc;
use std::thread;

pub struct Stream {
    audio_thread: Option<thread::JoinHandle<()>>,
    thread_channel_sender: Option<mpsc::Sender<Command>>,
}

pub enum Command {
    Play,
    Pause,
    Quit,
}

impl Stream {
    pub fn new(
        audio_thread: Option<thread::JoinHandle<()>>,
        thread_channel_sender: Option<mpsc::Sender<Command>>,
    ) -> Self {
        Stream {
            audio_thread,
            thread_channel_sender,
        }
    }

    pub fn play(&self) -> Result<()> {
        if let Some(sender) = self.thread_channel_sender.as_ref() {
            sender.send(Command::Play)?;
        }
        Ok(())
    }

    pub fn pause(&self) -> Result<()> {
        if let Some(sender) = self.thread_channel_sender.as_ref() {
            sender.send(Command::Pause)?;
        }
        Ok(())
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        if let Some(sender) = self.thread_channel_sender.take() {
            sender.send(Command::Quit).unwrap();
        }
        if let Some(thread) = self.audio_thread.take() {
            thread.join().unwrap();
        }
    }
}
