use std::collections::HashMap;

use petgraph::{graph::NodeIndex, visit::EdgeRef, Direction::Incoming};

use crate::{graph_json::{GraphJson, NodeJsonData}, messages::send_status};

// the audio graph!!!
pub struct AudioGraph {
    graph: petgraph::Graph<AudioGraphNode, AudioGraphEdge> 
} 

enum AudioGraphNode {
    Input {
        file_path: String
    },
    Output {
        final_buffer: Vec<i16>
    }
}

impl AudioGraphNode {
    pub fn num_outputs(&self) -> usize {
        match self {
            AudioGraphNode::Input { file_path: _  } => 1,
            AudioGraphNode::Output { final_buffer: _ } => 0,
        }
    }

    pub fn process(&mut self, 
        window: &tauri::Window, 
        my_idx: NodeIndex, 
        inputs: &mut HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32>, usize, usize)>>, 
        outputs: &mut HashMap<NodeIndex<u32>, Vec<Option<EdgeData>>>
    ) {
        match self {
            AudioGraphNode::Input { file_path } => {
                send_status(window, "Processing input node");
                
                println!("filepath: {}", file_path);

                let mut wav = match hound::WavReader::open(file_path) {
                    Ok(val) => val,
                    Err(err) => {
                        send_status(window, format!("Failed to open wav: {}", err.to_string()));
                        return;
                    },
                };

                send_status(window, format!("Reading samples (this part is slow)"));
                println!("Wav format is {:?} - {:?}", wav.spec().sample_format, wav.spec().bits_per_sample);

                let mut samples: Vec<i16> = Vec::with_capacity(wav.duration() as usize * 2);
                for sample in wav.samples::<i16>() {
                    match sample {
                        Ok(val) => {
                            samples.push(val);
                        },
                        Err(err) => {
                            send_status(window, format!("Failed to read wav: {}", err.to_string()));
                            return;
                        },
                    }
                }

                send_status(window, format!("Reading samples complete!"));
                
                match outputs.get_mut(&my_idx) {
                    Some(vec) => {
                        vec[0] = Some(EdgeData::SoundBuffer { buf: samples })
                    },
                    None => {
                        send_status(window, format!("Potential issue with outputs matrix initialization"))
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
        }
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
        buf: Vec<i16>
    }
}

impl AudioGraph {
    pub fn process(&mut self, window: &tauri::Window) -> Result<Vec<i16>, ()> {
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
            self.graph[*idx].process(window, *idx, &mut inputs, &mut outputs);
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
                NodeJsonData::Output {} => {
                    let idx = graph.add_node(AudioGraphNode::Output {
                        final_buffer: Vec::new()
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

            graph.add_edge(*from_node.unwrap(), *to_node.unwrap(), AudioGraphEdge {
                from_idx: 0,
                to_idx: 0
            });
        }

        Ok(AudioGraph { graph })
    }
}   

