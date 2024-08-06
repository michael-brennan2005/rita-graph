- (SOLVED) **BIG IDEA IN PLAY HERE**: Nodes should not own their outputs - this creates the issues.
    - Node has X inputs - how does it get them?
        - For each node create a vec of (usize, usize) - first usize is NodeIndex, second usize is index of that node's output vec.
        - from this we can either create a new vec of &mut edgedata, or pass in some central Vec<Vec<EdgeData>> where they can get the data themselves
            - this seems like a move
    - Sort nodes and then run in order, passing in the input_dependencies or something
    - How does a node get its input?
    -  process(window: &tauri::Window, 
        my_idx: NodeIndex, 
        inputs: &mut HashMap<NodeIndex<u32>, Vec<(NodeIndex<u32> (node), usize (from_idx), usize (to_idx))>>, 
        outputs: &mut HashMap<NodeIndex<u32>, Vec<Option<EdgeData>>>)
            - How does a node get its input?
                - Node needs to find the node and slot for input:
                    - inputs.get(my_idx).find(to_idx == desired input slot)
                - Once node and slot are acquired search through outputs:
                    - outputs.get(found_node_idx).get(found_from_idx)
            - How does a node set its output?
                - outputs.get(my_idx).set(desired_output_slot, val)
            - Input_slot == to_idx, output_slot == from_idx
    - (ACTUAL TODO MOST OF THIS IS DISCUSSION): Make nice methods for getting input and setting output.

- TODO audit - i guess this should be a routine thing
- SampleConfig - Graph needs to know correct sample rate for everythang
- Prevent compile button from being clicked while compilation is already happening
    - or at least provide a prompt or sumn