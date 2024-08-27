"use client";

import { useEffect, useState } from "react";

export default function Page(): JSX.Element {
	const [box, _setBox] = useState(() => {
		return Array.from({ length: 10 }).map((_, i) => {
			return i + 1;
		});
	});

	return (
		<div className='container'>
			<div className='flex-1 w-full gap-4 flex flex-col'>
				<h1 className='title'>
					<span>metamorph</span>
				</h1>
				<div className='grid grid-cols-2 gap-2 lg:grid-cols-4'>
					{box.map((count) => (
						<div
							key={count}
							className='px-4 bg-violet-400/20 hover:bg-emerald-400/20 h-36 flex items-center justify-center rounded-md border-2 border-black select-none cursor-pointer'>
							{count}
						</div>
					))}
				</div>
			</div>
		</div>
	);
}
