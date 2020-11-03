use std::path;

use ghakuf::messages::{Message, MetaEvent};
use ghakuf::writer::Writer;

use crate::pitch::Pitches;
use crate::CompositionSettings;

const WHOLE_NOTE: usize = 1920;
const HALF_NOTE: usize = WHOLE_NOTE / 2;
const QUARTER_NOTE: usize = HALF_NOTE / 2;
const EIGHT_NOTE: usize = QUARTER_NOTE / 2;
const SIXTEENTH_NOTE: usize = EIGHT_NOTE / 2;
const THIRTY_SECOND_NOTE: usize = SIXTEENTH_NOTE / 2;
const SIXTY_FOURTH_NOTE: usize = THIRTY_SECOND_NOTE / 2;

struct VONote {
    note: usize,
    // MIDI note number
    offset: usize,
    // offset within the sequence
    duration: usize, // length of the note
}

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
            if actual_offset == other || other % actual_offset == 0 {
                res = true;
                break;
            }
        }
        res
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

pub fn compose(composition: &CompositionSettings, pitches: Pitches) {
    let tempo = composition.tempo as usize;
    let _channel = 0;

    // // create the MIDI track for the tempo and time signature
    //
    // MidiTrack tempoTrack = new MidiTrack();
    //
    // // track 0 is typically the tempo map, set tempo and time signature
    // TimeSignature ts = new TimeSignature();
    // ts.setTimeSignature( props.TIME_SIGNATURE_BEAT_AMOUNT, props.TIME_SIGNATURE_BEAT_UNIT,
    //                      TimeSignature.DEFAULT_METER, TimeSignature.DEFAULT_DIVISION );
    //
    // Tempo t = new Tempo();
    // t.setBpm( theTempo );

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
    let current_position = 0;
    let _current_bar_length = 0;
    let mut note_length = composition.n1_length;

    let patterns: Vec<VOPattern> = vec![];
    let notes = vec![];

    let current_pattern = VOPattern {
        notes: notes,
        pattern_num: 0,
        offset: current_position,
    };

    for i in 0..pitches.len() {
        let _pitch = pitches.get(i).unwrap(); // fixme

        // swap note length if conflicts with previously added note in other pattern
        if offset_conflict(current_position - current_pattern.offset, &patterns) {
            if note_length == composition.n1_length {
                note_length = composition.n2_length;
            } else {
                note_length = composition.n1_length;
            }
        }

        // create new note

        //final VONote note = new VONote( Pitch.frequencyToMIDINote( pitch ),
        //                                             currentPosition,
        //                                           ( long )( noteLength * MIDI.QUARTER_NOTE ));
        //
        //             // add note to Vector (so it can be re-added in next iterations)
        //
        //             notes.add( note );
        //
        //             // update current sequence position
        //
        //             currentPosition  += note.duration;
        //             currentBarLength += note.duration;

        let _note = VONote {
            note: 0, //ftomidi,
            offset: current_position,
            duration: note_length as usize * QUARTER_NOTE,
        };
    }

    // ###############################
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
fn offset_conflict(note_offset: usize, patterns: &Vec<VOPattern>) -> bool {
    for p in patterns {
        if p.notes.len() > 0 && p.conflicts(note_offset) {
            return true;
        }
    }

    false
}
