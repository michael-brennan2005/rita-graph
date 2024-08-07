import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useEffect, useState } from "react";

export type InputNodeData = {
    filepath?: string;
};

export type InputNode = Node<InputNodeData>;

export default function InputNode({
    data,
}: NodeProps<InputNode>) {
    const [filePath, setFilePath] = useState("");
    useEffect(() => {
        data.filepath = filePath;
    }, [filePath])

    return (
        // We add this class to use the same styles as React Flow's default nodes.
        <div>
            <div>Input</div>
            <div>{filePath != "" ? filePath : "None"}</div>
            <button onClick={(_) => {
                invoke("pick_file").then((val) => {
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