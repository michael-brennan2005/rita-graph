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
    #[serde(rename = "type")]
    pub node_type: String,
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
    Output {}
}

#[derive(serde::Deserialize, Debug)]
pub struct EdgeJson {
    pub source: String,
    pub target: String
}
