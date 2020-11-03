use mmb_core::{compose, CompositionSettings, OutputSettings};

fn main() {
    let composition = CompositionSettings::default();
    let output = OutputSettings::default();

    compose(composition, output);
}
