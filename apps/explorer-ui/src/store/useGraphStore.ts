import { create } from 'zustand';
import { Node, Edge } from 'reactflow';

interface GraphState {
  nodes: Node[];
  edges: Edge[];
  selectedNodeId: string | null;
  setNodes: (nodes: Node[]) => void;
  setEdges: (edges: Edge[]) => void;
  setSelectedNodeId: (id: string | null) => void;
  updateNodeData: (nodeId: string, data: Record<string, any>) => void;
}

export const useGraphStore = create<GraphState>((set) => ({
  nodes: [
    {
      id: 'root',
      position: { x: 400, y: 100 },
      data: {
        label: 'Your dApp (Cargo.toml)',
        registrationStatus: 'pending',
      },
      type: 'input',
      style: { background: '#1e293b', border: '2px solid #3b82f6' },
    },
    {
      id: 'dep1',
      position: { x: 200, y: 300 },
      data: {
        label: 'soroban-sdk',
        registrationStatus: 'registered',
        address: 'GBZX...XXXX',
      },
      style: { background: '#1e293b', border: '2px solid #10b981' },
    },
    {
      id: 'dep2',
      position: { x: 600, y: 300 },
      data: {
        label: 'serde',
        registrationStatus: 'unregistered',
      },
      style: { background: '#1e293b', border: '2px solid #ef4444' },
    },
  ],
  edges: [
    {
      id: 'e-root-dep1',
      source: 'root',
      target: 'dep1',
      animated: true,
      style: { stroke: '#10b981', strokeWidth: 2 },
    },
    {
      id: 'e-root-dep2',
      source: 'root',
      target: 'dep2',
      animated: false,
      style: { stroke: '#ef4444', strokeWidth: 2 },
    },
  ],
  selectedNodeId: null,

  setNodes: (nodes) => set({ nodes }),
  setEdges: (edges) => set({ edges }),
  setSelectedNodeId: (id) => set({ selectedNodeId: id }),
  updateNodeData: (nodeId, data) =>
    set((state) => ({
      nodes: state.nodes.map((node) =>
        node.id === nodeId ? { ...node, data: { ...node.data, ...data } } : node,
      ),
    })),
}));
