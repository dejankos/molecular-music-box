mod composer;
mod pitch;

use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::Path;

pub struct CompositionSettings {
    tempo: f64,
    beat_amount: usize,
    beat_unit: usize,
    n1_length: f64,
    n2_length: f64,
    pattern_bars: usize,
    pattern_num: usize,
    min_octave: usize,
    max_octave: usize,
    scale: Vec<String>,
    unique_per_pattern: bool,
}

impl Default for CompositionSettings {
    fn default() -> Self {
        Self {
            tempo: 120.0,
            beat_amount: 4,
            beat_unit: 4,
            n1_length: 4.0,
            n2_length: 3.0,
            pattern_bars: 4,
            pattern_num: 4,
            min_octave: 2,
            max_octave: 3,
            scale: vec![
                "E".into(),
                "F".into(),
                "G".into(),
                "A".into(),
                "B".into(),
                "C".into(),
                "D".into(),
            ],
            unique_per_pattern: false,
        }
    }
}

impl Display for CompositionSettings {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} bpm in {} / {} time\n\
             with a 1st note length of {} and a 2nd note length of {}\n\
             for {} patterns with a length of {} bars each\n\
             and an octave range of {} to {} for the following notes:\n\
             {}\n\
             rendering a unique track [{}] for each pattern.",
            self.tempo,
            self.beat_amount,
            self.beat_unit,
            self.n1_length,
            self.n2_length,
            self.pattern_num,
            self.pattern_bars,
            self.min_octave,
            self.max_octave,
            self.scale.join(", "),
            self.unique_per_pattern
        )
    }
}

pub struct OutputSettings<P>
where
    P: AsRef<Path>,
{
    path: P,
    file_name: String,
}

impl Default for OutputSettings<&str> {
    fn default() -> Self {
        Self {
            path: "",
            file_name: "output.mid".to_string(),
        }
    }
}

pub fn compose<P>(composition: CompositionSettings, _output: OutputSettings<P>)
where
    P: AsRef<Path>,
{
    let pitches = pitch::calculate_pitches(&composition);
    composer::compose(&composition, pitches);

    println!("done");
}
