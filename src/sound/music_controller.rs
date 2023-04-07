use std::collections::HashMap;

use geng::prelude::{thread_rng, IteratorRandom};
use rustysynth::Synthesizer;

use super::{
    config::*,
    sound_queue::SoundQueue,
    source::{IntoRawSource, RawSource},
    synthesize,
};

pub struct MusicController {
    config: MusicConfig,
    bpm: f32,
    tick_t: f32,
    tick: Ticks,
    synthesizers: HashMap<SectionName, Synthesizer>,
    current_section: Option<(SectionName, SectionConfig)>,
    sounds_queue: SoundQueue,
    buffer: RawSource,
}

impl MusicController {
    pub fn new(
        config: MusicConfig,
        bpm: f32,
        synthesizers: HashMap<SectionName, Synthesizer>,
    ) -> Self {
        let mut controller = Self {
            config,
            bpm: 1.0,
            tick_t: 1.0,
            tick: 0,
            synthesizers,
            current_section: None,
            sounds_queue: SoundQueue::new(),
            buffer: RawSource::new(44100, vec![]),
        };
        controller.set_bpm(bpm);
        controller
    }

    pub fn get_buffer(&self) -> &RawSource {
        &self.buffer
    }

    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm;
        self.tick_t = 60.0 / (bpm * self.config.ticks_per_beat as f32);
    }

    pub fn update(&mut self, delta_time: f32) -> Vec<RawSource> {
        // Audio buffer
        for _ in 0..(rodio::Source::sample_rate(&self.buffer) as f32 * delta_time) as u32 {
            if self.buffer.next().is_none() {
                break;
            }
        }

        // Sound queue
        let mut mix = RawSource::new(rodio::Source::sample_rate(&self.buffer), vec![]);
        std::mem::swap(&mut self.buffer, &mut mix);
        let mut sounds = Vec::new();
        for sound in self.sounds_queue.update(delta_time) {
            mix = rodio::Source::mix(sound.clone(), mix).into_raw_source();
            sounds.push(sound);
        }
        self.buffer = mix;
        sounds
    }

    pub fn tick(&mut self) {
        self.tick += 1;

        let is_beat = self.tick % self.config.ticks_per_beat == 0;
        if is_beat {
            self.beat();
        }

        self.section_tick(is_beat);
    }

    fn section_tick(&mut self, immediate_next_section: bool) {
        if let Some((section_name, section)) = &mut self.current_section {
            let synthesizer = self
                .synthesizers
                .get_mut(section_name)
                .expect("Failed to get the section's synthesizer");

            // Next section event
            while let Some(event) = section.events.pop_front() {
                match event {
                    SectionEvent::Delay { delay } => {
                        if delay > 0 {
                            section
                                .events
                                .push_front(SectionEvent::Delay { delay: delay - 1 });
                            break;
                        }
                    }
                    SectionEvent::NoteDefault(note) => {
                        section.events.push_front(SectionEvent::Note(NoteOn {
                            note,
                            velocity: None,
                            duration: None,
                        }))
                    }
                    SectionEvent::Note(note) => {
                        let duration = note.duration.unwrap_or(section.default_duration);
                        self.sounds_queue
                            .play_immediately(synthesize::synthesize_note(
                                note.note.to_note(section.key),
                                note.velocity.unwrap_or(section.default_velocity),
                                duration,
                                self.tick_t,
                                synthesizer,
                            ));
                        section
                            .events
                            .push_front(SectionEvent::Delay { delay: duration });
                    }
                    SectionEvent::Tagged(tagged) => match tagged {
                        SectionEventTagged::ChangeDefaultVelocity { value } => {
                            section.default_velocity = value;
                        }
                        SectionEventTagged::ChangeDefaultDuration { value } => {
                            section.default_duration = value;
                        }
                    },
                }
            }

            if section.events.is_empty() {
                // End of the section
                self.current_section = None;
                if immediate_next_section {
                    self.next_section();
                    self.section_tick(false);
                }
            }
        }
    }

    fn beat(&mut self) {
        // let beat = self.tick / self.config.ticks_per_beat;

        // let sound = if beat % 2 == 0 {
        //     // Major beat
        //     source::Beat::default().into_raw_source()
        // } else {
        //     // Minor beat
        //     source::Beat::new(150.0, 0.2).into_raw_source()
        // };

        // self.sounds_queue.play_immediately(sound);

        if self.current_section.is_none() {
            self.next_section();
        }
    }

    fn next_section(&mut self) {
        // Get the next section
        if let Some((section_name, section)) = self
            .config
            .sections
            .iter()
            .filter(|(_, section)| {
                // Filter BPM range
                self.bpm >= section.bpm_range[0] as f32 && self.bpm <= section.bpm_range[1] as f32
            })
            .choose(&mut thread_rng())
        {
            self.current_section = Some((section_name.to_owned(), section.clone()));
        }
    }
}
