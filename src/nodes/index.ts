import type { BuiltInNode, Node, NodeTypes } from "@xyflow/react";

import InputNode, { type InputNode as InputNodeType } from "./InputNode.tsx";
import OutputNode, { type OutputNode as OutputNodeType } from "./OutputNode.tsx";

export const initialNodes = [
  {
    id: "aa",
    type: "input",
    position: { x: -100, y: 200 },
    data: { filepath: "audio.wav" },
  },
  {
    id: "aaa",
    type: "output",
    position: { x: -100, y: 300 },
    data: {},
  },
] satisfies Node[];

export const nodeTypes = {
  "output": OutputNode,
  "input": InputNode
  // Add any of your custom nodes here!
} satisfies NodeTypes;

// Append the types of you custom edges to the BuiltInNode type
export type CustomNodeType = InputNodeType | OutputNodeType;