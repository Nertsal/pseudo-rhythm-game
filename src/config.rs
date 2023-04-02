use std::collections::VecDeque;

use super::*;

pub type SFName = String;
pub type SectionName = String;
pub type Ticks = u32;

#[derive(Debug, Clone, Deserialize, geng::Assets)]
#[asset(json)]
pub struct MusicConfig {
    pub soundfonts: HashMap<SFName, String>,
    pub ticks_per_beat: Ticks,
    pub sections: HashMap<SectionName, SectionConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SectionConfig {
    pub soundfont: SFConfig,
    pub bpm_range: [Ticks; 2],
    pub volume: f32,
    pub key: Note,
    pub default_velocity: u32,
    pub default_duration: Ticks,
    pub events: VecDeque<SectionEvent>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "serder::SFConfigSerde")]
pub struct SFConfig {
    pub name: SFName,
    pub program: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum SectionEvent {
    Delay { delay: Ticks },
    NoteDefault(NoteSpec),
    Note(NoteOn),
    Tagged(SectionEventTagged),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum SectionEventTagged {
    // NoteOff { note: NoteSpec },
    ChangeDefaultVelocity { value: u32 },
    ChangeDefaultDuration { value: Ticks },
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoteOn {
    pub note: NoteSpec,
    pub velocity: Option<u32>,
    pub duration: Option<Ticks>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(untagged)]
pub enum NoteSpec {
    KeyDelta(i8),
    Note(Note),
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(from = "serder::NoteSerde")]
pub struct Note {
    pub octave: u8,
    pub note: u8,
}

impl NoteSpec {
    pub fn to_note(self, key: Note) -> Note {
        match self {
            Self::KeyDelta(delta) => key.shift(delta),
            Self::Note(note) => note,
        }
    }
}

impl Note {
    pub fn shift(self, delta: i8) -> Self {
        let note_delta = delta.rem_euclid(12);
        let octave_delta = (delta - note_delta) / 12;
        Self {
            octave: self.octave.saturating_add_signed(octave_delta),
            note: self.note.saturating_add_signed(note_delta),
        }
    }

    pub fn from_midi(midi: u8) -> Self {
        Self {
            octave: midi / 12,
            note: midi % 12,
        }
    }

    pub fn to_midi(self) -> u8 {
        self.octave * 12 + self.note
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self.note {
            0 => "C",
            1 => "C#",
            2 => "D",
            3 => "D#",
            4 => "E",
            5 => "F",
            6 => "F#",
            7 => "G",
            8 => "G#",
            9 => "A",
            10 => "A#",
            11 => "B",
            _ => unreachable!(),
        };
        let octave = self.octave;
        write!(f, "{name}{octave}")
    }
}

impl std::str::FromStr for Note {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Proper error message
        let (note, octave) = match s.strip_suffix(|c: char| c.is_ascii_digit()) {
            Some(note) => (note, s.get(s.len() - 1..).unwrap().parse().unwrap()),
            None => (s, 4),
        };
        let note = match note {
            "C" => 0,
            "C#" => 1,
            "D" => 2,
            "D#" => 3,
            "E" => 4,
            "F" => 5,
            "F#" => 6,
            "G" => 7,
            "G#" => 8,
            "A" => 9,
            "A#" => 10,
            "B" => 11,
            _ => return Err(()),
        };
        Ok(Self { octave, note })
    }
}

impl Default for MusicConfig {
    fn default() -> Self {
        Self {
            soundfonts: default(),
            ticks_per_beat: 4,
            sections: default(),
        }
    }
}

mod serder {
    use super::*;

    #[derive(Deserialize)]
    #[serde(untagged)]
    pub(super) enum SFConfigSerde {
        Name(String),
        Full { name: String, program: Option<u8> },
    }

    impl From<SFConfigSerde> for SFConfig {
        fn from(value: SFConfigSerde) -> Self {
            match value {
                SFConfigSerde::Name(name) => Self {
                    name,
                    program: None,
                },
                SFConfigSerde::Full { name, program } => Self { name, program },
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    pub(super) enum NoteSerde {
        Midi(u8),
        Eng(String),
    }

    impl From<NoteSerde> for Note {
        fn from(value: NoteSerde) -> Self {
            match value {
                NoteSerde::Midi(midi) => Self::from_midi(midi),
                NoteSerde::Eng(note) => note.parse().expect("Unrecognized note name"),
            }
        }
    }
}
