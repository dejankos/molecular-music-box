
use std::path;


use ghakuf::messages::{Message, MetaEvent, MidiEvent};
use ghakuf::writer::Writer;

use crate::pitch::{frequency_to_midi_note, Pitches};
use crate::CompositionSettings;

const WHOLE_NOTE: usize = 1920;
const HALF_NOTE: usize = WHOLE_NOTE / 2;
const QUARTER_NOTE: usize = HALF_NOTE / 2;
const EIGHT_NOTE: usize = QUARTER_NOTE / 2;
const SIXTEENTH_NOTE: usize = EIGHT_NOTE / 2;
const THIRTY_SECOND_NOTE: usize = SIXTEENTH_NOTE / 2;
const SIXTY_FOURTH_NOTE: usize = THIRTY_SECOND_NOTE / 2;

#[derive(Clone)]
struct VONote {
    note: usize,
    // MIDI note number
    offset: usize,
    // offset within the sequence
    duration: usize, // length of the note
}

#[derive(Clone)]
struct VOPattern {
    notes: Vec<VONote>,
    // all the notes within the pattern
    pattern_num: usize,
    // the number of this pattern within the total sequence
    offset: usize, // the start offset of the pattern
}

impl VOPattern {
    fn conflicts(&self, other: usize) -> bool {
        let mut res = false;
        for note_offset in self.get_offsets() {
            let actual_offset = note_offset - self.offset;
            if actual_offset == other || actual_offset > 0 && other % actual_offset == 0 {
                res = true;
                break;
            }
        }
        res
    }

    fn add_note(&mut self, note: VONote) {
        self.notes.push(note);
    }

    fn get_offsets(&self) -> Vec<usize> {
        self.notes.iter().map(|n| n.offset).collect()
    }

    fn get_range_start_offset(&self) -> usize {
        if let Some(n) = self.notes.get(0) {
            n.offset
        } else {
            0
        }
    }

    fn get_range_end_offset(&self) -> usize {
        if let Some(n) = self.notes.last() {
            n.offset + n.duration
        } else {
            0
        }
    }

    fn get_range_length(&self) -> usize {
        self.get_range_end_offset() - self.get_range_start_offset()
    }
}

pub fn compose(composition: &CompositionSettings, mut pitches: Pitches) {
    let tempo = composition.tempo as usize;
    let _channel = 0;
    let mut messages: Vec<Message> = vec![];

    let tempo_msg = Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::SetTempo,
        data: [(tempo >> 16) as u8, (tempo >> 8) as u8, tempo as u8].to_vec(),
    };

    messages.push(tempo_msg);
    messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    });
    messages.push(Message::TrackChange);

    let _velocity = 100;
    let mut current_position = 0;
    let mut current_bar_length = 0;
    let mut note_length = composition.n1_length;

    let mut patterns: Vec<VOPattern> = vec![];

    let mut current_pattern = VOPattern {
        notes: vec![],
        pattern_num: 0,
        offset: current_position,
    };

    for mut i in 0..pitches.len() as isize {
        let pitch = *pitches.get(i as usize).unwrap(); // fixme

        // swap note length if conflicts with previously added note in other pattern
        let offset = current_pattern.offset;
        if offset_conflict(current_position - offset, patterns.as_slice()) {
            if note_length == composition.n1_length {
                note_length = composition.n2_length;
            } else {
                note_length = composition.n1_length;
            }
        }

        // create new note
        println!("before freq to midi");
        let note = VONote {
            note: frequency_to_midi_note(pitch),
            offset: current_position,
            duration: note_length as usize * QUARTER_NOTE,
        };
        println!("after freq to midi");
        // update current sequence position
        current_position += &note.duration;
        current_bar_length += &note.duration;

        // add note to Vector (so it can be re-added in next iterations)
        current_pattern.add_note(note);

        if current_bar_length / WHOLE_NOTE > composition.pattern_bars {
            patterns.push(current_pattern.clone());

            let _notes = current_pattern.notes.clone();
            // store current notes in new pattern
            current_pattern = VOPattern {
                notes: vec![],
                pattern_num: patterns.len(),
                offset: current_position,
            };

            current_bar_length = 0;
        }

        // break the loop when we've rendered the desired amount of patterns
        if patterns.len() >= composition.pattern_num {
            break;
        }

        // if we have reached the end of the pitch range, start again
        // from the beginning until we have rendered all the patterns
        if i == (pitches.len() - 1) as isize {
            pitches.reverse(); // go down the scale
            i = -1; // todo  in origin source was -1,
        }

        let total_length = current_position;

        //  for ( final VOPattern pattern : patterns )
        //         {
        //             float patternLength = 0f;
        //
        //             while ( patternLength < ( totalLength - pattern.offset ))
        //             {
        //                 for ( final VONote note : pattern.notes )
        //                 {
        //                     noteTrack.insertNote( channel, note.note, velocity,
        //                                         ( long ) ( note.offset + patternLength ), note.duration );
        //                 }
        //                 patternLength += pattern.getRangeLength();
        //             }
        //
        //             // create new track for pattern, if specified
        //
        //             if ( props.UNIQUE_TRACK_PER_PATTERN )
        //             {
        //                 noteTrack = createTrack( "melody" );
        //                 tracks.add( noteTrack );
        //             }
        //         }

        for p in &patterns {
            let mut pattern_length = 0;
            println!("for 1");
            while pattern_length < (total_length - p.offset) {
                println!("while 1");
                for n in p.notes.iter() {
                    println!("for 2");

                    //todo hm ?
                    messages.push(Message::MidiEvent {
                        delta_time: n.duration as u32,
                        event: MidiEvent::NoteOn {
                            ch: _channel,
                            note: n.note as u8,
                            velocity: _velocity,
                        },
                    });
                }

                pattern_length += p.get_range_length();
                println!("pattern_length {}", pattern_length);
            }
        }
    }

    messages.push(Message::MetaEvent {
        delta_time: 0,
        event: MetaEvent::EndOfTrack,
        data: Vec::new(),
    });

    let path = path::Path::new("examples/example.mid");
    let mut writer = Writer::new();
    writer.running_status(true);
    for message in &messages {
        writer.push(message);
    }
    let _ = writer.write(&path);
}

//fixme
fn offset_conflict(note_offset: usize, patterns: &[VOPattern]) -> bool {
    for p in patterns {
        if p.notes.len() > 0 && p.conflicts(note_offset) {
            return true;
        }
    }

    false
}
