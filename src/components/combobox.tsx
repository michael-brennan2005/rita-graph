import { ChevronsUpDown } from "lucide-react"
import { Button } from "./ui/button"
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover"
import { Command, CommandGroup, CommandItem, CommandList } from "./ui/command"
import { useState } from "react";

export interface ComboboxProps {
    label: () => string, 
    choices: { value: string, label: string}[], 
    onChoice: (value: string) => void   
}

export default function Combobox(props: ComboboxProps) {
    const [open, setOpen] = useState(false);
    
    return (<Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
            <Button
            variant="outline"
            role="combobox"
            aria-expanded={open}
            className="w-[200px] justify-between">
                {props.label()}
                <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
            </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[200px] p-0">
            <Command>
                <CommandList>
                    <CommandGroup>
                        {props.choices.map((val) => {
                            return <CommandItem
                                key={val.value}
                                value={val.value}
                                onSelect={(currentValue) => {
                                    props.onChoice(currentValue)
                                    setOpen(false)
                                }}>
                                {val.label}
                            </CommandItem>
                        })}
                    </CommandGroup>
                </CommandList>
            </Command>
        </PopoverContent>
    </Popover>)
}
