use rodio::Source;
use rustysynth::Synthesizer;

use crate::config::{Note, Ticks};
use crate::source::RawSource;

pub fn synthesize_note(
    note: Note,
    velocity: u32,
    duration: Ticks,
    tick_t: f32,
    synthesizer: &mut Synthesizer,
) -> RawSource {
    // Set the note
    let note = note.to_midi().into();
    synthesizer.note_on(0, note, velocity as i32);

    // Synthesize the note playing
    let note_on = synthesize_ticks(duration, tick_t, synthesizer);

    // Turn off the note
    synthesizer.note_off(0, note);

    // Render the note turning off
    let note_off = synthesize_ticks(1, tick_t, synthesizer);

    RawSource::new(note_on.sample_rate(), note_on.chain(note_off).collect())
}

fn synthesize_ticks(ticks: Ticks, tick_t: f32, synthesizer: &mut Synthesizer) -> RawSource {
    // The output buffer
    let sample_count = (ticks as f32 * tick_t * synthesizer.get_sample_rate() as f32) as usize;
    let mut left: Vec<f32> = vec![0_f32; sample_count];
    let mut right: Vec<f32> = vec![0_f32; sample_count];

    // Render the waveform
    synthesizer.render(&mut left[..], &mut right[..]);

    // TODO: stereo
    RawSource::new(synthesizer.get_sample_rate() as u32, left)
}
