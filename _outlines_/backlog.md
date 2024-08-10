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

--------------------------------
- TODO audit - i guess this should be a routine thing


- (ACTUAL TODO MOST OF THIS IS DISCUSSION): Make nice methods for getting input and setting output.
    - Do this when we have more nodes and a sense of the pattern for what nodes do with their inputs and outputs
    - I think we figured out a very nice way to do this in AudioNodeInput::process() - see how BinOp does it
- AudioGraphNode::process() should be returning results
    - Getting inputs and outputs is a million .unwraps() and .expects() but if you put into a method that returns a single Option/Result, this becomes
    very doable.
    - Errs can be a string type for now, emit an event when it happens and we can use shadcn Sonner components for a cool interface
- Switch out Window to AppHandle in rust code
- PLEASE get rid of buttons and use command menu for adding nodes

- Figuring out a way for nice validation on inputs and stuff
    - Making sure numeric inputs only allow numeric inputs, in a nice/ergonomic way
    - There is probably libraries/resources for this - common problem
    - I think this ties into node builder api if we make that - it will have functions for creating inputs that validate their values
- Node builder for UI - this is gonna get very annoying very quickly