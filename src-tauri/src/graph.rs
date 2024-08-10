use core::f32;
use std::{borrow::Borrow, collections::HashMap};

use petgraph::{data::Build, graph::NodeIndex, visit::EdgeRef, Direction::Incoming};
use samplerate::{convert, ConverterType};

use crate::{graph_json::{BinOp, GraphJson, NodeJsonData, ShortBehavior, WaveType}, messages::send_status, playback::spec::{F32Convert, F32FormatSpec}};

// the audio graph!!!
pub struct AudioGraph {
    graph: petgraph::Graph<AudioGraphNode, AudioGraphEdge> 
} 

enum AudioGraphNode {
    Input {
        file_path: String
    },
    WaveGen {
        wave_type: WaveType,
        frequency: f32,
        amplitude: f32,
        seconds: f32
    },
    BinOp {
        operation: BinOp,
        on_short_a: ShortBehavior,
        on_short_b: ShortBehavior
    },
    Output {
        final_buffer: Vec<f32>
    }
}

impl AudioGraphNode {
    pub fn num_outputs(&self) -> usize {
        match self {
            AudioGraphNode::Input { file_path: _  } => 1,
            AudioGraphNode::WaveGen { wave_type: _, frequency: _, amplitude: _, seconds: _ } => 1,
            AudioGraphNode::Output { final_buffer: _ } => 0,
            AudioGraphNode::BinOp { operation, on_short_a, on_short_b } => 1,
            
        }
    }

    pub fn process(&mut self, 
        window: &tauri::Window, 
        my_idx: NodeIndex, 
        inputs: &mut HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, usize, usize)>>, 
        outputs: &mut HashMap<NodeIndex<u32>, Vec<Option<EdgeData>>>,
        output_spec: F32FormatSpec
    ) {
        match self {
            AudioGraphNode::Input { file_path } => {
                send_status(window, "Processing input node");
                
                let mut wav = match hound::WavReader::open(file_path) {
                    Ok(val) => val,
                    Err(err) => {
                        send_status(window, format!("Failed to open wav: {}", err.to_string()));
                        return;
                    },
                };

                send_status(window, format!("Reading samples (this part is slow)"));
                println!("Wav format is {:?} - {:?}", wav.spec().sample_format, wav.spec().bits_per_sample);
                
                if wav.spec().channels as usize != output_spec.channels {
                    panic!("Doesnt support unequal channels right now");
                }

                let samples: Vec<f32> = match (wav.spec().sample_format, wav.spec().bits_per_sample) {
                    (hound::SampleFormat::Float, 32) => {
                        wav.samples::<f32>().map(|s| {
                            s.expect("Reading samples failed")
                        }).collect()
                    },
                    (hound::SampleFormat::Int, 16) => {
                        wav.samples::<i16>().map(|s| {
                            F32FormatSpec::convert(s.expect("Reading samples failed"))
                        }).collect()
                    },
                    _ => {
                        println!("Hit this");
                        todo!();
                    }
                };

                send_status(window, format!("Resampling..."));

                let interleaved_samples: Vec<f32> = convert(wav.spec().sample_rate as u32, output_spec.sample_rate as u32, 2, ConverterType::SincFastest, samples.as_slice()).expect("Resampling failed");

                send_status(window, format!("Reading audio completed!"));
                
                match outputs.get_mut(&my_idx) {
                    Some(vec) => {
                        vec[0] = Some(EdgeData::SoundBuffer { buf: interleaved_samples })
                    },
                    None => {
                        send_status(window, format!("Potential issue with outputs matrix initialization (what)"))
                    }
                };
            },
            AudioGraphNode::WaveGen { wave_type, frequency, amplitude, seconds } => {
                let total_frames = *seconds * (output_spec.sample_rate as f32);
                let mut samples: Vec<f32> = Vec::with_capacity((total_frames * 2.0) as usize);

                send_status(window, format!("Generating samples..."));

                let sample_rate_f: f32 = output_spec.sample_rate as f32;

                match *wave_type {
                    WaveType::Sine => {
                        let angle = 2.0 * f32::consts::PI * *frequency;
                        for i in 0..(total_frames as usize) {
                            let t = i as f32 / sample_rate_f;
                            let sample = *amplitude * (angle * t).sin();
                            samples.push(sample);
                            samples.push(sample);
                        }        
                    },
                    WaveType::Triangle => {
                        let period = 1.0 / *frequency;
                        for i in 0..(total_frames as usize) {
                            let t = i as f32 / sample_rate_f;
                            let sample = 
                                (4.0 * *amplitude / period)
                                * f32::abs(((t - period / 4.0) % period) - period / 2.0)
                                - *amplitude;
                            samples.push(sample);
                            samples.push(sample);
                        }
                    },
                    WaveType::Square => {
                        for i in 0..(total_frames as usize) {
                            let t = i as f32 / sample_rate_f;
                            let sample = *amplitude * 2.0 * (2.0 * (*frequency * t).floor() - (2.0 * *frequency * t).floor()) + 1.0;
                            samples.push(sample);
                            samples.push(sample);
                        }
                    },
                    WaveType::Sawtooth => {
                        let period = 1.0 / *frequency;
                        for i in 0..(total_frames as usize) {
                            let t = i as f32 / sample_rate_f;
                            let sample = 2.0 * *amplitude * ( (t / period) - (1.0 / 2.0 + t / period).floor());
                            samples.push(sample);
                            samples.push(sample);
                        }
                    },
                }

                send_status(window, format!("Generating samples completed!"));
                
                match outputs.get_mut(&my_idx) {
                    Some(vec) => {
                        vec[0] = Some(EdgeData::SoundBuffer { buf: samples })
                    },
                    None => {
                        send_status(window, format!("Potential issue with outputs matrix initialization (what)"))
                    }
                };
            },
            AudioGraphNode::Output { final_buffer } => {
                send_status(window, "Processing output node");

                match inputs.get(&my_idx) {
                    Some(vec) => {
                        match vec.iter().find(|x| {
                            x.2 == 0
                        }) {
                            Some(val) => {
                                if let Some(vec) = outputs.get_mut(&val.0) {
                                    if let Some(edge_data) = std::mem::replace(&mut vec[val.1], None) {
                                        match edge_data {
                                            EdgeData::SoundBuffer { buf } => {
                                                *final_buffer = buf;
                                            },
                                        }
                                    } else {
                                        println!("UH OH (A)")
                                    }
                                } else {
                                    println!("UH OH (B)");
                                }

                            },
                            None => todo!(),
                        }
                    },
                    None => todo!(),
                }
            },
            AudioGraphNode::BinOp { operation, on_short_a, on_short_b } => {
                let a_buf = {
                    let input = inputs.get(&my_idx).unwrap()
                    .iter().find(|x| {
                        x.2 == 0
                    }).unwrap();

                    match outputs.get(&input.0).unwrap()[input.1].as_ref().unwrap() {
                        EdgeData::SoundBuffer { buf } => buf,
                    }
                
                };
                    
                let b_buf = {
                    let input = inputs.get(&my_idx).unwrap()
                    .iter().find(|x| {
                        x.2 == 1
                    }).unwrap();

                    match outputs.get(&input.0).unwrap()[input.1].as_ref().unwrap() {
                        EdgeData::SoundBuffer { buf } => buf,
                    }
                };

                let apply: fn(f32, f32) -> f32 = match *operation {
                    BinOp::Add => |x, y| { x + y },
                    BinOp::Sub => |x, y| { x - y},
                    BinOp::Mul => |x, y| { x * y},
                };

                let get_a = |i: usize| {
                    if i < a_buf.len() {
                        a_buf[i]
                    } else {
                        match on_short_a {
                            ShortBehavior::Zero => 0.0,
                            ShortBehavior::UseLastSample => a_buf[a_buf.len() - 1],
                        }
                    }
                };

                let get_b = |i: usize| {
                    if i < b_buf.len() {
                        b_buf[i]
                    } else {
                        match on_short_b {
                            ShortBehavior::Zero => 0.0,
                            ShortBehavior::UseLastSample => b_buf[b_buf.len() - 1],
                        }
                    }
                };

                let total_frames = usize::max(a_buf.len(), b_buf.len());
                let mut samples: Vec<f32> = Vec::with_capacity(total_frames);

                for i in 0..total_frames {
                    samples.push(apply(get_a(i), get_b(i)));
                }

                match outputs.get_mut(&my_idx) {
                    Some(vec) => {
                        vec[0] = Some(EdgeData::SoundBuffer { buf: samples })
                    },
                    None => {
                        send_status(window, format!("Potential issue with outputs matrix initialization (what)"))
                    }
                }
            },
        }
    }

    /// Returns `true` if the audio graph node is [`BinOp`].
    ///
    /// [`BinOp`]: AudioGraphNode::BinOp
    #[must_use]
    fn is_bin_op(&self) -> bool {
        matches!(self, Self::BinOp { .. })
    }
}

// We have an edge from X node to Y node. This edge struct maps the output index
// from the outputs of X to the the input index of the inputs of Y.
struct AudioGraphEdge {
    from_idx: usize,
    to_idx: usize
}

#[derive(Clone)]
enum EdgeData {
    SoundBuffer {
        buf: Vec<f32>
    }
}

impl AudioGraph {
    pub fn process(&mut self, window: &tauri::Window, output_spec: F32FormatSpec) -> Result<Vec<f32>, ()> {
        send_status(window, "Beginning graph processing.");

        // get sorted order
        let sorted = match petgraph::algo::toposort(&self.graph, None) {
            Ok(order) => order, 
            Err(_) => {
                send_status(window, "Error sorting graph - cycle exists");
                return Err(());
            },
        };
        let _ = window.emit("set_total_nodes", sorted.len());

        // get input dependencies
        // Each Vec is the inputs for that node, first usize is what node it originates from, second usize is from what "slot" output it is, third usize is to what "slot" input it is.
        let mut inputs: HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, usize, usize)>> = HashMap::new();
        for idx in &sorted {
            let mut vec: Vec<(NodeIndex<u32>, usize, usize)> = Vec::new();

            for edge in self.graph.edges_directed(*idx, Incoming) {
                vec.push((edge.source(), edge.weight().from_idx, edge.weight().to_idx));
            }

            inputs.insert(*idx, vec);
        }

        // Initialize outputs
        // Each vec is the outputs of that node
        let mut outputs: HashMap<NodeIndex<u32>, Vec<Option<EdgeData>>> = HashMap::new();
        for idx in &sorted {
            let output_num = self.graph[*idx].num_outputs();

            let vec: Vec<Option<EdgeData>> = vec![None; output_num];
            
            outputs.insert(*idx, vec);
        }

        // Processing (the fun step)
        let _ = window.emit("set_completed_nodes", 0);
        for (i, idx) in sorted.iter().enumerate() {
            self.graph[*idx].process(window, *idx, &mut inputs, &mut outputs, output_spec);
            let _ = window.emit("set_completed_nodes", i + 1);
        }
        
        // Find output and return the final buffer
        for idx in &sorted {
            match &mut self.graph[*idx] {
                AudioGraphNode::Output { final_buffer } => {
                    return Ok(std::mem::replace(final_buffer, Vec::new()));
                },
                _ => continue
            }
        }
        
        Err(())
    }
}

impl TryFrom<GraphJson> for AudioGraph {
    // todo: probably do real error handling
    type Error = ();

    fn try_from(value: GraphJson) -> Result<Self, Self::Error> {
        let mut graph = petgraph::Graph::<AudioGraphNode, AudioGraphEdge>::new();

        let mut node_indexes: HashMap<String, NodeIndex> = HashMap::new();

        for node_json in value.nodes {
            match node_json.data {
                NodeJsonData::Input { file_path } => {
                    let idx = graph.add_node(AudioGraphNode::Input {
                        file_path: file_path 
                    });
                    node_indexes.insert(node_json.id, idx);
                },
                NodeJsonData::WaveGen { wave_type, frequency, amplitude, seconds } => {
                    let wave_type: WaveType = wave_type.try_into().expect("Unknown wave type");
                    let idx = graph.add_node(AudioGraphNode::WaveGen { 
                        wave_type, 
                        frequency, 
                        amplitude, 
                        seconds 
                    });
                    node_indexes.insert(node_json.id, idx);
                },
                NodeJsonData::Output {} => {
                    let idx = graph.add_node(AudioGraphNode::Output {
                        final_buffer: Vec::new()
                    });
                    node_indexes.insert(node_json.id, idx);
                },
                NodeJsonData::BinOp { bin_op, on_short_a, on_short_b } => {
                    let idx = graph.add_node(AudioGraphNode::BinOp { 
                        operation: bin_op.try_into().expect("Failed to parse binop"), 
                        on_short_a: on_short_a.try_into().expect("Failed to parse onshorta"), 
                        on_short_b: on_short_b.try_into().expect("Falied to parse onshortb") 
                    });
                    node_indexes.insert(node_json.id, idx);
                },
            }
        }

        for edge_json in value.edges {
            let from_node = node_indexes.get(&edge_json.source);
            let to_node = node_indexes.get(&edge_json.target);

            if from_node.is_none() || to_node.is_none() {
                println!("This triggered");
                return Err(());
            }

            let target_handle = match edge_json.target_handle {
                Some(val) => val.parse::<usize>().expect("Failed to parse target handle"),
                None => 0,
            };

            graph.add_edge(*from_node.unwrap(), *to_node.unwrap(), AudioGraphEdge {
                from_idx: 0,
                to_idx: target_handle
            });
        }

        Ok(AudioGraph { graph })
    }
}   

