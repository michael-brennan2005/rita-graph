import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { Input } from "../ui/input";
import { Label } from "../ui/label";

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
        <Card>
            <CardHeader className="bg-blue-500">
                <CardTitle className="text-">
                    Input - File
                </CardTitle>
            </CardHeader>
            <CardContent>
                <Label>Filename</Label>
                <Input value={filePath != "" ? filePath : "N/A"} disabled/>
                <Button className="h-6" onClick={(_) => {
                    invoke("pick_file").then((val) => {
                        let filepath = val as string | undefined;
                        if (filepath) {
                            setFilePath(filepath)
                        }
                    })
                }}>Choose</Button>
                <Handle type="source" position={Position.Right} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4" />
            </CardContent>
        </Card>
    );
}