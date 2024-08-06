import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useState } from "react";

export type OutputNodeData = {};

export type OutputNode = Node<OutputNodeData>;

export default function OutputNode(props: NodeProps<OutputNode>) {
    return (
        // We add this class to use the same styles as React Flow's default nodes.
        <div>
            <div>Output</div>
            <Handle type="target" position={Position.Left} />
        </div>
    );
}