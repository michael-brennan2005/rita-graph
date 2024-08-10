import { invoke } from "@tauri-apps/api/tauri";
import type { Node, NodeProps } from "@xyflow/react";
import { Handle, Position } from "@xyflow/react";
import { useEffect, useState } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "../ui/card";
import { Input } from "../ui/input";
import { Label } from "../ui/label";
import { Popover, PopoverContent, PopoverTrigger } from "../ui/popover";
import { ChevronsUpDown } from "lucide-react";
import { Command, CommandGroup, CommandItem, CommandList } from "../ui/command";
import { validateNumericInput } from "@/lib/utils";
import Combobox from "../combobox";

type BinOpType = "add" | "sub" | "mul";
type ShortBehavior = "zero" | "last_sample";

const binOpChoices = [
    {
        value: "add",
        label: "Add",
    },
    {
        value: "sub",
        label: "Subtract",
    },
    {
        value: "mul",
        label: "Multiply"
    },
]

const shortBehaviorChoices = [
    {
        value: "zero",
        label: "Use 0",
    },
    {
        value: "last_sample",
        label: "Use Last Sample"
    }
]

export type BinOpNodeData = {
    bin_op: BinOpType,
    on_short_a: ShortBehavior,
    on_short_b: ShortBehavior,
};

export type BinOpNode = Node<BinOpNodeData>;

export default function BinOpNode({
    data,
}: NodeProps<BinOpNode>) {
    const [binOp, setBinOp] = useState(data.bin_op);
    const [onShortA, setOnShortA] = useState(data.on_short_a);
    const [onShortB, setOnShortB] = useState(data.on_short_b);

    useEffect(() => {
        data.bin_op = binOp;
        data.on_short_a = onShortA;
        data.on_short_b = onShortB;
    }, [binOp, onShortA, onShortB])

    return (
        <Card>
            <CardHeader className="bg-purple-600">
                <CardTitle>
                    Binary Op - {binOpChoices.find((val) => val.value === binOp)?.label}
                </CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-2 mt-2">
                <Label>Operation</Label>
                <Combobox 
                    label={() => binOpChoices.find((val) => val.value == binOp)!.label}
                    choices={binOpChoices}
                    onChoice={(val: string) => setBinOp(val as BinOpType)}/>
                <Label>If samples(A) {"<"} samples(B)</Label>
                <Combobox 
                    label={() => shortBehaviorChoices.find((val) => val.value == onShortA)!.label}
                    choices={shortBehaviorChoices}
                    onChoice={(val: string) => setOnShortA(val as ShortBehavior)}/>
                <Label>If samples(B) {"<"} samples(A)</Label>
                <Combobox 
                    label={() => shortBehaviorChoices.find((val) => val.value == onShortB)!.label}
                    choices={shortBehaviorChoices}
                    onChoice={(val: string) => setOnShortB(val as ShortBehavior)}/>   

                <Handle type="target" id="0" position={Position.Left} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4 top-[45%]" />
                <Handle type="target" id="1" position={Position.Left} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4 top-[55%]" />

                <Handle type="source" position={Position.Right} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4" />
            </CardContent>
        </Card>
    );
}