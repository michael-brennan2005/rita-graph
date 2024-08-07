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

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum NodeJsonData {
    Input {
        #[serde(rename = "filePath")]
        file_path: String
    },
    Output {}
}

#[derive(serde::Deserialize, Debug)]
pub struct EdgeJson {
    pub source: String,
    pub target: String
}
