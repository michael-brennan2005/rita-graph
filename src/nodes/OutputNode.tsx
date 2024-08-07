import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useState } from "react";

export type OutputNodeData = {};

export type OutputNode = Node<OutputNodeData>;

export default function OutputNode(props: NodeProps<OutputNode>) {
    return (
        <div className="w-48 p-0 text-white bg-gray-800 rounded-lg text-xs border-0 overflow-hidden">
            <div className="px-2 py-1 bg-red-600 font-semibold ">Final Output</div>
            <div className="px-2 py-3 flex flex-row justify-between">
               
            </div>
            <Handle type="target" position={Position.Left} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4" />
        </div>
        // // We add this class to use the same styles as React Flow's default nodes.
        // <div className="w-48 p">
        //     <div>Output</div>
        //     <Handle type="target" position={Position.Left} />
        // </div>
    );
}