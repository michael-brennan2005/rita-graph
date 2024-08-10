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
    useReactFlow,
} from "@xyflow/react";

import { initialNodes, nodeTypes, type CustomNodeType } from "./components/nodes";
import { initialEdges, edgeTypes, type CustomEdgeType } from "./components/edges";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";
import { formatTime } from "@/lib/utils";

export default function App() {
    const [nodes, setNodes, onNodesChange] = useNodesState<CustomNodeType>(initialNodes);
    const [edges, setEdges, onEdgesChange] =
        useEdgesState<CustomEdgeType>(initialEdges);
    const [rfInstance, setRfInstance] = useState<ReactFlowInstance | undefined>(undefined);


    const onConnect: OnConnect = useCallback(
        (connection) => {
            setEdges((edges) => addEdge(connection, edges))
        },
        [setEdges]
    );

    const [statusMessage, setStatusMessage] = useState("statuses show up here!")
    const [currentSeconds, setCurrentSeconds] = useState(0);
    const [totalSeconds, setTotalSeconds] = useState(0);

    const [totalNodes, setTotalNodes] = useState(0)
    const [completedNodes, setCompletedNodes] = useState(0)

    listen("send_status_message", (evt) => {
        const msg = (evt.payload as { message: string }).message
        setStatusMessage(msg)
    })

    listen("set_total_nodes", (evt) => {
        setTotalNodes(evt.payload as number)
    })

    listen("set_completed_nodes", (evt) => {
        setCompletedNodes(evt.payload as number)
    })

    listen("update_playback_position", (evt) => {
        console.log(evt)
        let msg = evt.payload as { current_seconds: number, total_seconds: number }
        setCurrentSeconds(msg.current_seconds)
        setTotalSeconds(msg.total_seconds)
    })

    useEffect(() => {
        if (rfInstance) {
            console.log(rfInstance.toObject());
        }
    })

    return (
        <div className="w-full h-full font-sans">
            <div className="bg-gray-800 text-white p-2">
                <div className="flex flex-row justify-start gap-5">
                    <Button onClick={() => {
                        if (!rfInstance) {
                            return
                        }

                        invoke("compile_graph", { graphPayload: JSON.stringify(rfInstance.toObject()) }).then((_val) => {
                            return
                        })
                    }}>Compile</Button>
                
                    <Button onClick={() => {
                        invoke("play")
                    }}>Play</Button>

                    <Button onClick={() => {
                        invoke("pause")
                    }}>Pause</Button>

                    {/* TODO: figure out how to do flex growing and shrinking, so this takes up a ton of space and buttons take up little */}
                    <div className="text-nowrap">
                        {formatTime(currentSeconds)} / {formatTime(totalSeconds)}
                    </div>
                </div>
                <div className="flex flex-row mt-4 gap-5">
                    <div className="text-nowrap overflow-hidden">
                        {statusMessage}
                    </div>
                    <div className="bg-red-500">
                        Processed nodes: {completedNodes} / {totalNodes}
                    </div>
                </div>
                <div className="flex flex-row mt-4 gap-5">
                    <Button onClick={() => {
                        setNodes((nds) => {
                            return nds.concat({
                                // todo: more robust ID generation
                                id: `${nds.length + 1}`,
                                position: rfInstance!.screenToFlowPosition({
                                    x: 400,
                                    y: 400,
                                }),
                                origin: [0.5, 0.0],
                                type: "input",
                                data: { filePath: undefined }
                            })
                        })
                    }}>Input</Button>

                    <Button onClick={() => {
                        setNodes((nds) => {
                            return nds.concat({
                                // todo: more robust ID generation
                                id: `${nds.length + 1}`,
                                position: rfInstance!.screenToFlowPosition({
                                    x: 400,
                                    y: 400,
                                }),
                                origin: [0.5, 0.0],
                                type: "wave_gen",
                                data: { 
                                    wave_type: "sine",
                                    frequency: 0.0,
                                    amplitude: 0.0,
                                    seconds: 0.0
                                }
                            })
                        })
                    }}>WaveGen</Button>

                    <Button onClick={() => {
                        setNodes((nds) => {
                            return nds.concat({
                                // todo: more robust ID generation
                                id: `${nds.length + 1}`,
                                position: rfInstance!.screenToFlowPosition({
                                    x: 400,
                                    y: 400,
                                }),
                                origin: [0.5, 0.0],
                                type: "binop",
                                data: { 
                                    bin_op: "add",
                                    on_short_a: "zero",
                                    on_short_b: "zero"
                                }
                            })
                        })
                    }}>BinOp</Button>
                </div>
            </div>

            <div className="w-full h-[80%]">
                <ReactFlow<CustomNodeType, CustomEdgeType>
                    nodes={nodes}
                    nodeTypes={nodeTypes}
                    onNodesChange={onNodesChange}
                    edges={edges}
                    edgeTypes={edgeTypes}
                    onEdgesChange={onEdgesChange}
                    onConnect={onConnect}
                    onInit={(instance) => {
                        // @ts-ignore
                        setRfInstance(instance)
                    }}
                    fitView>   
                    
                    <Background />
                    <Controls />
                </ReactFlow>
            </div>
        </div>
    );
}