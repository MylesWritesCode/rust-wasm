'use client';

import { useCallback, useReducer, useState } from 'react';
// import { transformRs } from 'metamorph';
import * as wasm from 'metamorph';
import type { Glyph, Edge, Vertex } from 'metamorph';

import { Control, PerformanceCard, type Performance } from './_components';

const initialPerformance: Required<Performance> = { api: 0, transform: 0 };

type PerformanceActions = { type: 'api'; payload: number } | { type: 'transform'; payload: number } | { type: 'reset' };

function performanceReducer(state: Required<Performance>, action: PerformanceActions) {
  switch (action.type) {
    case 'api':
      return { ...state, api: action.payload };
    case 'transform':
      return { ...state, transform: action.payload };
    case 'reset':
      return initialPerformance;
  }
}

const initialCount: Count = { vertices: 10000, edges: 10000 };

interface Count {
  vertices: number;
  edges: number;
}

type CountActions = { type: 'vertices' | 'edges'; payload: number } | { type: 'reset' };

function countReducer(state: Count, action: CountActions) {
  switch (action.type) {
    case 'vertices':
      return { ...state, vertices: action.payload };
    case 'edges':
      return { ...state, edges: action.payload };
    case 'reset':
      return initialCount;
  }
}

interface GraphVertex {
  id: string;
  label: string;
  parent?: string;
}

interface GraphEdge {
  id: string;
  source: string;
  target: string;
}

type GraphElement = GraphVertex | GraphEdge;

function isGraphEdge(element: unknown): element is GraphEdge {
  const e = element as GraphEdge;
  return e.source !== undefined && e.target !== undefined;
}

export default function Page(): JSX.Element {
  const [count, dispatchCount] = useReducer(countReducer, initialCount);

  const [wasmPerf, dispatchWasmPerf] = useReducer(performanceReducer, initialPerformance);
  const [jsPerf, dispatchJsPerf] = useReducer(performanceReducer, initialPerformance);

  const [vertices, setVertices] = useState<GraphVertex[]>([]);
  const [edges, setEdges] = useState<GraphEdge[]>([]);

  const fetchGraphData = useCallback(async () => {
    const start = performance.now();
    const res = await fetch('http://localhost:5001/generate-graph', {
      method: 'POST',
      body: JSON.stringify(count),
      headers: {
        'Content-Type': 'application/json',
      },
    });
    const end = performance.now();
    const duration = end - start;

    dispatchWasmPerf({ type: 'api', payload: duration });
    dispatchJsPerf({ type: 'api', payload: duration });

    const data = await res.json();

    if (data.vertices) {
      setVertices(data.vertices);
    }

    if (data.edges) {
      setEdges(data.edges);
    }

    return data;
  }, [count]);

  const handleWasm = useCallback(async () => {
    let _v = vertices;
    let _e = edges;
    if (_v.length === 0 || _e.length === 0 || _v.length !== count.vertices || _e.length !== count.edges) {
      const { vertices, edges } = await fetchGraphData();
      _v = vertices;
      _e = edges;
    }

    const { transformRs } = await import('metamorph');

    const start = performance.now();
    const s = transformRs([..._v, ..._e]);
    const end = performance.now();
    const duration = end - start;

    console.log(s);
    dispatchWasmPerf({ type: 'transform', payload: duration });
  }, [vertices, edges, count, fetchGraphData]);

  const transformJs = useCallback((v: GraphVertex[], e: GraphEdge[]) => {
    const result: (Vertex | Edge)[] = [];
    for (const [i, element] of [...v, ...e].entries()) {
      if (isGraphEdge(element)) {
        const edge: Edge = {
          id: element.id,
          source: element.source,
          target: element.target,
          free: () => { },
        };
        result.push(edge);
      } else {
        let glyphs: Glyph[] | undefined;

        if (i % 8 === 0) {
          glyphs = [
            {
              label: 'some-glyph',
              angle: 45,
              free: () => { },
            },
          ];
        }

        const vertex: Vertex = {
          id: element.id,
          label: element.label,
          glyphs,
          parent: element.parent,
          free: () => { },
        };
        result.push(vertex);
      }
    }

    return result;
  }, []);

  const handleJs = useCallback(async () => {
    let _v = vertices;
    let _e = edges;
    if (_v.length === 0 || _e.length === 0 || _v.length !== count.vertices || _e.length !== count.edges) {
      const { vertices, edges } = await fetchGraphData();
      _v = vertices;
      _e = edges;
    }

    const start = performance.now();
    const s = transformJs(_v, _e);
    const end = performance.now();
    const duration = end - start;

    console.log(s);
    dispatchJsPerf({ type: 'transform', payload: duration });
  }, [vertices, edges, count, fetchGraphData, transformJs]);

  const items = [
    {
      name: 'rust (wasm)',
      action: handleWasm,
    },
    {
      name: 'js',
      action: handleJs,
    },
  ];

  const handleResetState = useCallback(() => {
    setVertices([]);
    setEdges([]);
    dispatchWasmPerf({ type: 'reset' });
    dispatchJsPerf({ type: 'reset' });
  }, []);

  return (
    <div className='container max-w-3xl'>
      <div className='flex-1 w-full max-w-3xl gap-4 flex flex-col'>
        <h1 className='title'>
          <span>metamorph</span>
        </h1>
        <div className='grid grid-cols-2 gap-2'>
          <div className='col-span-2 p-4 grid grid-cols-2 gap-4'>
            <Control
              name='vertices'
              value={count.vertices}
              onChange={(payload) => dispatchCount({ type: 'vertices', payload })}
            />
            <Control
              name='edges'
              value={count.edges}
              onChange={(payload) => dispatchCount({ type: 'edges', payload })}
            />
          </div>
          <div
            id='preload-api'
            className='btn btn-success rounded-none'
            onClick={() => fetchGraphData()}
            onKeyDown={() => fetchGraphData()}>
            <span>preload api</span>
            <span>({vertices.length + edges.length} elements)</span>
          </div>
          <div
            id='reset-state'
            className='btn btn-error rounded-none'
            onClick={handleResetState}
            onKeyDown={handleResetState}>
            <span>reset state</span>
          </div>
          {items.map(({ name, action }) => (
            <div key={name} className='btn btn-primary rounded-none' onClick={action} onKeyDown={action}>
              {name}
            </div>
          ))}
          {(wasmPerf?.api > 0 || jsPerf?.api > 0) && (
            <div className='card bg-info text-info-content shadow-xl rounded-none border-info-content col-span-2 mt-4'>
              <div className='card-title w-full py-2'>
                <h1 className='w-full'>performance stats</h1>
              </div>
            </div>
          )}
          {wasmPerf.api > 0 && <PerformanceCard name='rust (wasm)' stats={wasmPerf} />}
          {jsPerf.api > 0 && <PerformanceCard name='js' stats={jsPerf} />}
        </div>
      </div>
    </div>
  );
}
