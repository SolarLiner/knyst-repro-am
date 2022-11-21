use std::time::Duration;
use knyst::audio_backend::CpalBackend;
use knyst::prelude::*;
use knyst::wavetable::WavetableOscillatorOwned;

fn main() {
    let mut backend = CpalBackend::new(Default::default()).unwrap();
    let mut graph = Graph::new(GraphSettings::default());

    let carrier = graph.push_gen(WavetableOscillatorOwned::new(Wavetable::sine()));
    let modulator = graph.push_gen(WavetableOscillatorOwned::new(Wavetable::sine()));
    let mod_amp = graph.push_gen(Mult);

    graph.connect(constant(440.).to(carrier).to_label("freq")).unwrap();
    graph.connect(constant(100.).to(modulator).to_label("freq")).unwrap();
    graph.connect(carrier.to(mod_amp)).unwrap();
    graph.connect(modulator.to(mod_amp).to_index(1)).unwrap();
    graph.connect(mod_amp.to_graph_out()).unwrap();
    graph.connect(mod_amp.to_graph_out().to_index(1)).unwrap();

    backend.start_processing(&mut graph, Resources::new(Default::default())).unwrap();

    let mut discard = String::new();
    std::io::stdin().read_line(&mut discard).unwrap();
}
