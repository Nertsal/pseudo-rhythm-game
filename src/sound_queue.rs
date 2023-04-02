use std::collections::VecDeque;

use rodio::Source;

use crate::source::{IntoRawSource, RawSource};

const EARLY_PLAY: f32 = 0.5;

struct QueuedSound {
    inner: RawSource,
    time_left: f32,
}

pub struct SoundQueue {
    queue: VecDeque<QueuedSound>,
}

impl SoundQueue {
    pub fn new() -> Self {
        SoundQueue {
            queue: Default::default(),
        }
    }

    pub fn play_delayed(&mut self, sound: RawSource, delay: f32) {
        self.queue.push_back(QueuedSound {
            inner: sound,
            time_left: delay,
        });
    }

    /// The sound will be played after the next `update` call.
    pub fn play_immediately(&mut self, sound: RawSource) {
        self.queue.push_front(QueuedSound {
            inner: sound,
            time_left: 0.0,
        });
    }

    /// Returns the sounds that should start playing now.
    pub fn update(&mut self, delta_time: f32) -> Vec<RawSource> {
        let mut to_play = Vec::new();

        // Update timers
        for sound in &mut self.queue {
            sound.time_left -= delta_time;

            if sound.time_left <= EARLY_PLAY {
                // Remove the sound from the queue to play it
                let mut source = rodio::source::Empty::new().into_raw_source();
                std::mem::swap(&mut source, &mut sound.inner);
                let source = source
                    .delay(std::time::Duration::from_secs_f64(
                        sound.time_left.max(0.0).into(),
                    ))
                    .into_raw_source();
                to_play.push(source);
                sound.time_left = 0.0;
            }
        }

        // Remove played sounds from the front of the queue
        while let Some(sound) = self.queue.pop_front() {
            if sound.time_left > 0.0 {
                self.queue.push_front(sound);
                break;
            }
        }

        to_play
    }
}

impl Default for SoundQueue {
    fn default() -> Self {
        Self::new()
    }
}
