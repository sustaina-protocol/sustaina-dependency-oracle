'use client';

import React, { useState } from 'react';
import ReactFlow, { Background, Controls, MiniMap } from 'reactflow';
import 'reactflow/dist/style.css';
import { useGraphStore } from '../store/useGraphStore';

export default function DependencyExplorer() {
  const { nodes, edges, selectedNodeId, setSelectedNodeId } = useGraphStore();
  const [splitPercentage, setSplitPercentage] = useState(10);
  const [isGenerating, setIsGenerating] = useState(false);

  const registeredDeps = nodes.filter((n) => n.data.registrationStatus === 'registered');
  const unregisteredDeps = nodes.filter((n) => n.data.registrationStatus === 'unregistered');

  const handleGenerateFunding = async () => {
    setIsGenerating(true);
    try {
      const bpsPerDep = (splitPercentage * 100) / registeredDeps.length;
      const xdrCommand = generateXdrCommand(registeredDeps, bpsPerDep);

      alert(`Generated XDR Command:\n\n${xdrCommand}`);
    } finally {
      setIsGenerating(false);
    }
  };

  const generateXdrCommand = (deps: typeof nodes, bps: number): string => {
    let cmd = 'soroban contract invoke \\\n  --id <FACTORY_ID> \\\n  -- deploy_split';

    for (const dep of deps) {
      cmd += ` \\\n  --recipient ${dep.data.address} --bps ${Math.floor(bps)}`;
    }

    return cmd;
  };

  return (
    <div className="h-screen w-full flex flex-col bg-slate-950 text-white">
      <header className="p-6 border-b border-slate-800 flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold">Sustaina Dependency Explorer</h1>
          <p className="text-slate-400 text-sm">
            Visualize and fund your open-source dependency tree on Stellar
          </p>
        </div>
        <button
          onClick={handleGenerateFunding}
          disabled={registeredDeps.length === 0 || isGenerating}
          className="bg-blue-600 hover:bg-blue-500 disabled:bg-slate-600 text-white px-6 py-2 rounded-md font-semibold transition"
        >
          {isGenerating ? 'Generating...' : 'Generate Split'}
        </button>
      </header>

      <div className="flex flex-1 overflow-hidden">
        <main className="flex-1">
          <ReactFlow nodes={nodes} edges={edges} fitView>
            <Background color="#334155" gap={16} />
            <Controls className="bg-slate-800 fill-white" />
            <MiniMap />
          </ReactFlow>
        </main>

        <aside className="w-96 bg-slate-900 border-l border-slate-800 p-6 overflow-y-auto">
          <h2 className="text-xl font-semibold mb-6">Funding Configuration</h2>

          <div className="space-y-6">
            <div>
              <label className="block text-sm font-medium mb-2">
                Split Percentage: {splitPercentage}%
              </label>
              <input
                type="range"
                min="1"
                max="100"
                value={splitPercentage}
                onChange={(e) => setSplitPercentage(Number(e.target.value))}
                className="w-full h-2 bg-slate-700 rounded-lg appearance-none cursor-pointer"
              />
              <p className="text-xs text-slate-400 mt-2">
                {registeredDeps.length > 0
                  ? `~${Math.floor((splitPercentage * 100) / registeredDeps.length)} BPS per dependency`
                  : 'No registered dependencies'}
              </p>
            </div>

            <div className="border-t border-slate-700 pt-6">
              <h3 className="font-semibold mb-4">Dependencies</h3>

              <div className="space-y-4">
                <div>
                  <h4 className="text-green-400 text-sm font-semibold mb-2">
                    Registered ({registeredDeps.length})
                  </h4>
                  <ul className="space-y-1">
                    {registeredDeps.map((dep) => (
                      <li
                        key={dep.id}
                        onClick={() => setSelectedNodeId(dep.id)}
                        className={`text-xs p-2 rounded cursor-pointer transition ${
                          selectedNodeId === dep.id
                            ? 'bg-blue-600'
                            : 'bg-slate-800 hover:bg-slate-700'
                        }`}
                      >
                        ✅ {dep.data.label}
                      </li>
                    ))}
                  </ul>
                </div>

                <div>
                  <h4 className="text-red-400 text-sm font-semibold mb-2">
                    Unregistered ({unregisteredDeps.length})
                  </h4>
                  <ul className="space-y-1">
                    {unregisteredDeps.map((dep) => (
                      <li
                        key={dep.id}
                        onClick={() => setSelectedNodeId(dep.id)}
                        className={`text-xs p-2 rounded cursor-pointer transition ${
                          selectedNodeId === dep.id
                            ? 'bg-blue-600'
                            : 'bg-slate-800 hover:bg-slate-700'
                        }`}
                      >
                        ❌ {dep.data.label}
                      </li>
                    ))}
                  </ul>
                </div>
              </div>
            </div>

            <div className="border-t border-slate-700 pt-6">
              <h3 className="font-semibold mb-2">Summary</h3>
              <dl className="space-y-1 text-sm">
                <div className="flex justify-between">
                  <dt className="text-slate-400">Total:</dt>
                  <dd>{nodes.length - 1}</dd>
                </div>
                <div className="flex justify-between text-green-400">
                  <dt>Registered:</dt>
                  <dd>{registeredDeps.length}</dd>
                </div>
                <div className="flex justify-between text-red-400">
                  <dt>Unregistered:</dt>
                  <dd>{unregisteredDeps.length}</dd>
                </div>
              </dl>
            </div>
          </div>
        </aside>
      </div>
    </div>
  );
}
