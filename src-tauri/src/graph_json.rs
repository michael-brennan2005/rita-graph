use serde;

/// The JSON incoming from our frontend: basically a 1:1 of what reactflow.toobject returns
#[derive(serde::Deserialize, Debug)]
pub struct GraphJson {
    pub nodes: Vec<NodeJson>, 
    pub edges: Vec<EdgeJson>
}

#[derive(serde::Deserialize, Debug)]
pub struct NodeJson {
    pub id: String,
    // So far all node types have different nodejsondata so we can just this to decipher types,
    // and not need the node_json that comes with reactflow's JSON output.
    pub data: NodeJsonData
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub enum WaveType {
    Sine,
    Triangle,
    Square,
    Sawtooth
}

impl TryFrom<String> for WaveType {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "sine" => Ok(WaveType::Sine),
            "triangle" => Ok(WaveType::Triangle),
            "square" => Ok(WaveType::Square),
            "sawtooth" => Ok(WaveType::Sawtooth),
            _ => Err(())
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul
}

impl TryFrom<String> for BinOp {
    type Error = ();
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "add" => Ok(BinOp::Add),
            "sub" => Ok(BinOp::Sub),
            "mul" => Ok(BinOp::Mul),
            _ => Err(())
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone, Copy)]
pub enum ShortBehavior {
    Zero,
    UseLastSample,
}

impl TryFrom<String> for ShortBehavior {
    type Error = ();
    
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "zero" => Ok(ShortBehavior::Zero),
            "last_sample" => Ok(ShortBehavior::UseLastSample),
            _ => Err(())
        }
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum NodeJsonData {
    Input {
        #[serde(rename = "filePath")]
        file_path: String
    },
    WaveGen {
        wave_type: String,
        frequency: f32,
        amplitude: f32,
        seconds: f32
    },
    BinOp {
        bin_op: String,
        on_short_a: String,
        on_short_b: String,
    },
    Output {}
}

#[derive(serde::Deserialize, Debug)]
pub struct EdgeJson {
    pub source: String,
    pub target: String,
    #[serde(rename="targetHandle")]
    pub target_handle: Option<String>
}
