"use client";

import { useEffect, useState } from "react";

export default function Page(): JSX.Element {
	const [vertices, setVertices] = useState(25);
	const [edges, setEdges] = useState(25);
	const [elements, setElements] = useState([]);

	const fetchGraphData = async (vertices: number, edges: number) => {
		const res = await fetch("http://localhost:5001/generate-graph", {
			method: "POST",
			body: JSON.stringify({ vertices, edges }),
			headers: {
				"Content-Type": "application/json",
			},
		});
		const data = await res.json();
		setElements(data);
	};

	const items = [
		{
			name: "1000 vertices, 1000 edges",
			action: async () => {
				await fetchGraphData(1000, 1000);
			},
		},
	];

	return (
		<div className='container'>
			<div className='flex-1 w-full gap-4 flex flex-col'>
				<h1 className='title'>
					<span>metamorph</span>
				</h1>
				<div className='grid grid-cols-2 gap-2 lg:grid-cols-4'>
					<div className='lg:col-span-4 col-span-2 p-4 bg-emerald-400/20 grid grid-cols-2 gap-4'>
						<div id='range-vertices' className='col-span-1 md:col-span-1'>
							<span>Vertices: </span>
							<span>{vertices}</span>
							<input
								type='range'
								min={0}
								max='10000'
								value={vertices}
								className='range mt-4 range-xs'
								step='200'
								onChange={(e) => setVertices(Number(e.target.value))}
							/>
						</div>
						<div id='range-edges' className='col-span-1 md:col-span-1'>
							<span>Edges: </span>
							<span>{edges}</span>
							<input
								type='range'
								min={0}
								max='10000'
								value={edges}
								className='range mt-4 range-xs'
								step='200'
								onChange={(e) => setEdges(Number(e.target.value))}
							/>
						</div>
					</div>

					<div
						className='px-4 bg-violet-400/20 hover:bg-emerald-400/20 h-36 flex items-center justify-center rounded-md border-2 border-black select-none cursor-pointer'
						onClick={() => fetchGraphData(vertices, edges)}
						onKeyDown={() => fetchGraphData(vertices, edges)}>
						fetchGraphData
					</div>
					{items.map(({ name, action }) => (
						<div
							key={name}
							className='px-4 bg-violet-400/20 hover:bg-emerald-400/20 h-36 flex items-center justify-center rounded-md border-2 border-black select-none cursor-pointer'
							onClick={action}
							onKeyDown={action}>
							{name}
						</div>
					))}
				</div>
			</div>
		</div>
	);
}
