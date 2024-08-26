import React from "react";
import GreetButton from "./greet-button";

export default function Page() {
	return (
		<div className='container'>
			<div className='flex-1 w-full mt-8'>
				<h1 className='title'>
					<span>alert</span>
				</h1>
				<div>
					<GreetButton />
				</div>
			</div>
		</div>
	);
}
