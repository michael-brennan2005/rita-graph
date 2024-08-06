import { useCallback, useEffect, useState } from "react";
import {
    Background,
    Controls,
    MiniMap,
    ReactFlow,
    addEdge,
    useNodesState,
    useEdgesState,
    type OnConnect,
    ReactFlowInstance,
} from "@xyflow/react";

import "@xyflow/react/dist/style.css";

import { initialNodes, nodeTypes, type CustomNodeType } from "./nodes";
import { initialEdges, edgeTypes, type CustomEdgeType } from "./edges";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";

export default function App() {
    const [nodes, , onNodesChange] = useNodesState<CustomNodeType>(initialNodes);
    const [edges, setEdges, onEdgesChange] =
        useEdgesState<CustomEdgeType>(initialEdges);
    const [rfInstance, setRfInstance] = useState<ReactFlowInstance | undefined>(undefined);
    
    const onConnect: OnConnect = useCallback(
        (connection) => setEdges((edges) => addEdge(connection, edges)),
        [setEdges]
    );

    const [statusMessage, setStatusMessage] = useState("statuses show up here!")

    listen("send_status_message", (evt) => {
        const msg = (evt.payload as { message: string }).message
        setStatusMessage(msg)
    })

    useEffect(() => {
        if (rfInstance) {
            console.log(rfInstance.toObject());
        }
    })
    return (
        <div style={{"height": "100%", "width": "100%"}}>
            <div>
                <button onClick={() => {
                    if (!rfInstance) {
                        return
                    }

                    invoke("compile_graph", { graphPayload: JSON.stringify(rfInstance.toObject()) }).then((val) => {
                        return
                    })
                }}>Compile</button>
                <button onClick={() => {
                    invoke("play")
                }}>Play</button>
                <h3>{statusMessage}</h3>
            </div>
            <div style={{"height": "80%", "width": "100%"}}>
            <ReactFlow<CustomNodeType, CustomEdgeType>
                style={{"height": 300}}
                nodes={nodes}
                nodeTypes={nodeTypes}
                onNodesChange={onNodesChange}
                edges={edges}
                edgeTypes={edgeTypes}
                onEdgesChange={onEdgesChange}
                onConnect={onConnect}
                onInit={setRfInstance}
                fitView>   
                
                <Background />
                <MiniMap />
                <Controls />
            </ReactFlow>

            </div>
        </div>
    );
}