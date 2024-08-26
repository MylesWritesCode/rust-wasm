"use client";

import dynamic from "next/dynamic";
import React from "react";

export default function GreetButton() {
	const load = async () => {
		const wasm = await import("metamorph");
		return wasm;
	};

	const Component = dynamic(() =>
		load().then((wasm) => () => {
			const { greet } = wasm;
			return (
				<button
					type='button'
					className='border p-2 rounded-sm'
					onClick={() => greet("from WASM")}
				>
					Click for an alert from WASM
				</button>
			);
		}),
	);

	return <Component />;
}
