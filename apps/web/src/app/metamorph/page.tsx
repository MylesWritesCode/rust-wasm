"use client";

import { useCallback, useReducer, useState } from "react";

import { Control, PerformanceCard, type Performance } from "./_components";

const initial: Required<Performance> = { api: 0, transform: 0 };

type PerformanceActions = { type: "api"; payload: number } | { type: "transform"; payload: number } | { type: "reset" };

function performanceReducer(state: Required<Performance>, action: PerformanceActions) {
	switch (action.type) {
		case "api":
			return { ...state, api: action.payload };
		case "transform":
			return { ...state, transform: action.payload };
		case "reset":
			return initial;
	}
}

export default function Page(): JSX.Element {
	const [vertices, setVertices] = useState(1000);
	const [edges, setEdges] = useState(1000);
	const [elements, setElements] = useState([]);

	const [wasmPerf, dispatchWasmPerf] = useReducer(performanceReducer, initial);
	const [jsPerf, dispatchJsPerf] = useReducer(performanceReducer, initial);

	const fetchGraphData = useCallback(async (vertices: number, edges: number) => {
		const start = performance.now();
		const res = await fetch("http://localhost:5001/generate-graph", {
			method: "POST",
			body: JSON.stringify({ vertices, edges }),
			headers: {
				"Content-Type": "application/json",
			},
		});
		const end = performance.now();
		const duration = end - start;

		dispatchWasmPerf({ type: "api", payload: duration });
		dispatchJsPerf({ type: "api", payload: duration });

		const data = await res.json();
		setElements(data);
	}, []);

	const handleWasm = useCallback(async () => {
		if (elements.length === 0 || elements.length !== vertices + edges) {
			await fetchGraphData(vertices, edges);
		}

		const start = performance.now();
		// do work here

		// mock doing work
		setTimeout(() => {
			const end = performance.now();

			const duration = end - start;
			dispatchWasmPerf({ type: "transform", payload: duration });
		}, 1000);
	}, [elements, vertices, edges, fetchGraphData]);

	const items = [
		{
			name: "rust (wasm)",
			action: async () => {
				await handleWasm();
			},
		},
		{
			name: "js",
			action: async () => {
				await fetchGraphData(vertices, edges);
			},
		},
	];

	const handleResetState = useCallback(() => {
		setElements([]);
		dispatchWasmPerf({ type: "reset" });
		dispatchJsPerf({ type: "reset" });
	}, []);

	return (
		<div className='container max-w-3xl'>
			<div className='flex-1 w-full max-w-3xl gap-4 flex flex-col'>
				<h1 className='title'>
					<span>metamorph</span>
				</h1>
				<div className='grid grid-cols-2 gap-2'>
					<div className='col-span-2 p-4 grid grid-cols-2 gap-4'>
						<Control name='vertices' value={vertices} onChange={setVertices} />
						<Control name='edges' value={edges} onChange={setEdges} />
					</div>
					<div
						id='preload-api'
						className='btn btn-success rounded-none'
						onClick={() => fetchGraphData(vertices, edges)}
						onKeyDown={() => fetchGraphData(vertices, edges)}>
						<span>preload api</span>
						<span>({elements.length} elements)</span>
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
