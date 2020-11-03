use crate::CompositionSettings;

pub type Pitches = Vec<f64>;

const C: f64 = 261.626;
const C_SHARP: f64 = 277.183;
const D: f64 = 293.665;
const D_SHARP: f64 = 311.127;
const E: f64 = 329.628;
const F: f64 = 349.228;
const F_SHARP: f64 = 369.994;
const G: f64 = 391.995;
const G_SHARP: f64 = 415.305;
const A: f64 = 440.0;
const A_SHARP: f64 = 466.164;
const B: f64 = 493.883;
const FLAT: &str = "b";
const SHARP: &str = "#";

const OCTAVE: &'static [f64] = &[
    C, C_SHARP, D, D_SHARP, E, F, F_SHARP, G, G_SHARP, A, A_SHARP, B,
];
const OCTAVE_SCALE: &'static [&'static str] = &[
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

//todo doc
// fixme casting horror
pub fn calculate_pitches(composition: &CompositionSettings) -> Pitches {
    let (note_idx, max_idx, mut octave) = (0, composition.scale.len() - 1, composition.min_octave);
    let mut i: isize = note_idx;
    let mut pitches = vec![];

    while i < composition.scale.len() as isize {
        pitches.push(note(OCTAVE_SCALE[i as usize], octave));
        // reached end of the note list ? increment octave
        if i == max_idx as isize && octave < composition.max_octave {
            i = -1; // todo maybe because of index
            octave += 1;
        }

        i += 1;
    }
    pitches
}

/// Generates the frequency in Hz corresponding to the given note at the given octave.
///
/// # Arguments
///
/// * `note` - musical note to return ( A, B, C, D, E, F, G with
/// possible enharmonic notes ( 'b' meaning 'flat', '#' meaning 'sharp' )
/// NOTE: flats are CASE sensitive ( to prevent seeing the note 'B' instead of 'b' )
///
/// * `octave` - the octave to return ( accepted range 0 - 9 )
///
/// Returns frequency in Hz for the requested note.
fn note(note: &str, octave: usize) -> f64 {
    let idx = index_of(&note, &FLAT);
    let mut note = note;
    let mut enharmonic = 0;

    // detect flat enharmonic
    if idx > -1 {
        let from = if idx == 0 { 0 } else { (idx - 1) as usize }; //FIXME - doesn't make sense
        note = &note[from..1];
        enharmonic = -1;
    }

    // detect sharp enharmonic

    let idx = index_of(&note, SHARP);
    if idx > -1 {
        let from = if idx == 0 { 0 } else { (idx - 1) as usize }; //FIXME - doesn't make sense
        note = &note[from..1];
        enharmonic = 1;
    }

    let mut freq = get_octave_idx(note, enharmonic);

    if octave == 4 {
        freq
    } else {
        // translate the pitches to the requested octave
        let d = (octave as isize) - 4;
        for _i in 0..d.abs() {
            if d > 0 {
                freq *= 2.0;
            } else {
                freq *= 0.5;
            }
        }

        freq
    }
}

//todo doc
fn get_octave_idx(note: &str, enharmonic: isize) -> f64 {
    let mut idx = 0.0;
    for (i, _) in (0..OCTAVE.len()).enumerate() {
        let ii = i as isize;
        if OCTAVE_SCALE[i] == note {
            let k = ii + enharmonic;
            if k > ii {
                idx = OCTAVE[0];
                break;
            } else if k < 0 {
                idx = OCTAVE[OCTAVE.len() - 1];
                break;
            } else {
                idx = OCTAVE[k as usize];
                break;
            }
        }
    }

    idx
}

fn index_of(s: &str, search: &str) -> isize {
    if let Some(idx) = s.find(search) {
        idx as isize
    } else {
        -1
    }
}

pub fn frequency_to_midi_note(frequency: f64) -> usize {
    //fixme oh well
    let r = 1.05946309436;
    let mut ref_ = 523.251;
    let mut sup_inf: isize = 0;
    let mut i: i8 = 0;
    let mut hautnb = 1.0;
    let mut ref1 = 0.0;
    let mut ref2 = 0.0;
    let mut flag = 0.0;
    let mut nmidi = 72;

    while frequency < ref_ {
        ref_ = (1000.0 * ref_ / r).floor() / 1000.0;
        i += 1;
        sup_inf = -1;
        flag = 1.0;
        ref1 = ref_;
    }

    while frequency > ref_ {
        ref_ = (1000.0 * ref_ * r).floor() / 1000.0;
        i -= 1;
        sup_inf = 1;
        ref2 = ref_;
    }

    if (frequency - ref1).abs() < (frequency - ref2).abs() {
        sup_inf = -1;
        i += 1;
    } else if flag == 1.0 {
        sup_inf = -1;
    }

    if ref1 == 0.0 {
        ref1 = (1000.0 * ref_ / r).floor() / 1000.0;
        if (frequency - ref1).abs() < (frequency - ref2).abs() {
            i += 1;
            sup_inf = 1;
        }
    }
    i = i.abs();

    while i != 0 {
        if hautnb == 1.0 && sup_inf == -1 || hautnb == 12.0 && sup_inf == 1 {
            if sup_inf == 1 {
                hautnb = 0.0;
            }

            if sup_inf == -1 {
                hautnb = 13.0;
            }
        }

        hautnb = hautnb + sup_inf as f64;
        nmidi = nmidi + sup_inf;

        i -= 1;
    }

    nmidi as usize
}
