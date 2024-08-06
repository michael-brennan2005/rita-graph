import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useState } from "react";

export type InputNodeData = {
    filepath?: string;
};

export type InputNode = Node<InputNodeData>;

export default function InputNode({
    positionAbsoluteX,
    positionAbsoluteY,
    data,
}: NodeProps<InputNode>) {
    const [filePath, setFilePath] = useState("");
    const x = `${Math.round(positionAbsoluteX)}px`;
    const y = `${Math.round(positionAbsoluteY)}px`;

    return (
        // We add this class to use the same styles as React Flow's default nodes.
        <div>
            <div>Input</div>
            <div>{filePath != "" ? filePath : "None"}</div>
            <button onClick={(evt) => {
                invoke("pickFile").then((val) => {
                    let filepath = val as string | undefined;
                    if (filepath) {
                        setFilePath(filepath)
                    }
                })
            }}>Choose</button>
            <Handle type="source" position={Position.Right} />
        </div>
    );
}