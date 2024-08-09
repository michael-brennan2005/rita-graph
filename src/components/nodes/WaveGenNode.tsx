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

type WaveType = "sine" | "sawtooth" | "triangle" | "square"

const comboboxChoices = [
    {
        value: "sine",
        label: "Sine",
    },
    {
        value: "sawtooth",
        label: "Sawtooth",
    },
    {
        value: "triangle",
        label: "Triangle"
    },
    {
        value: "square",
        label: "Square"
    }
]

export type WaveGenNodeData = {
    wave_type: WaveType,
    frequency: number,
    amplitude: number,
    seconds: number
};

export type WaveGenNode = Node<WaveGenNodeData>;

export default function WaveGenNode({
    data,
}: NodeProps<WaveGenNode>) {
    const [waveType, setWaveType] = useState(data.wave_type);
    const [waveTypeBoxOpen, setWaveTypeBoxOpen] = useState(false);
    const [frequency, setFrequency] = useState(data.frequency.toString());
    const [amplitude, setAmplitude] = useState(data.amplitude.toString());
    const [seconds, setSeconds] = useState(data.seconds.toString());

    useEffect(() => {
        data.wave_type = waveType;
        data.frequency = parseFloat(frequency);
        data.amplitude = parseFloat(amplitude);
        data.seconds = parseFloat(seconds);
    }, [waveType, frequency, amplitude, seconds])

    return (
        <Card>
            <CardHeader className="bg-green-600">
                <CardTitle>
                    Input - {waveType.charAt(0).toUpperCase() + waveType.slice(1)}
                </CardTitle>
            </CardHeader>
            <CardContent className="flex flex-col gap-2 mt-2">
                <Label>Wave type</Label>
                <Popover open={waveTypeBoxOpen} onOpenChange={setWaveTypeBoxOpen}>
                    <PopoverTrigger asChild>
                        <Button
                        variant="outline"
                        role="combobox"
                        aria-expanded={waveTypeBoxOpen}
                        className="w-[200px] justify-between">
                            {comboboxChoices.find((val) => val.value == waveType)?.label}
                            <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                    </PopoverTrigger>
                    <PopoverContent className="w-[200px] p-0">
                        <Command>
                            <CommandList>
                                <CommandGroup>
                                    {comboboxChoices.map((val) => {
                                        return <CommandItem
                                            key={val.value}
                                            value={val.value}
                                            onSelect={(currentValue) => {
                                                setWaveType(currentValue as WaveType)
                                                setWaveTypeBoxOpen(false)
                                            }}>
                                            {val.label}
                                        </CommandItem>
                                    })}
                                </CommandGroup>
                            </CommandList>
                        </Command>
                    </PopoverContent>
                </Popover>
                <Label>Frequency (Hz)</Label>
                <Input onChange={(evt) => {
                    setFrequency(evt.target.value)
                }} value={frequency} className="w-[200px]"/>
                <Label>Amplitude (0-100 (what))</Label>
                <Input onChange={(evt) => {
                    setAmplitude(evt.target.value)
                }} value={amplitude} className="w-[200px]"/>
                <Label>Seconds (s)</Label>
                <Input onChange={(evt) => {
                    setSeconds(evt.target.value)
                }} value={seconds} className="w-[200px]"/>
                <Handle type="source" position={Position.Right} className="border border-solid border-gray-300 bg-gray-600 rounded-full w-4 h-4" />
            </CardContent>
        </Card>
    );
}