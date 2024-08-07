import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useEffect, useState } from "react";
import { Button } from "../defaults";

export type InputNodeData = {
    filePath?: string;
};

export type InputNode = Node<InputNodeData>;

export default function InputNode({
    data,
}: NodeProps<InputNode>) {
    const [filePath, setFilePath] = useState("");
    useEffect(() => {
        data.filePath = filePath;
    }, [filePath])

    return (
        <div className="w-48 p-0 text-white bg-gray-800 rounded-lg text-xs border-0 overflow-hidden">
            <div className="px-2 py-1 bg-green-600 font-semibold ">Input - File</div>
            <div className="px-2 py-2 flex flex-row justify-between">
                <input value={filePath != "" ? filePath : "N/A"} disabled className="rounded-sm bg-gray-500 py-1 px-2 w-24"></input>
                <Button onClick={(_) => {
                    invoke("pick_file").then((val) => {
                        let filepath = val as string | undefined;
                        if (filepath) {
                            setFilePath(filepath)
                        }
                    })
                }}>Choose</Button>
                <Handle type="source" position={Position.Right} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4" />
            </div>
        </div>
    );
}