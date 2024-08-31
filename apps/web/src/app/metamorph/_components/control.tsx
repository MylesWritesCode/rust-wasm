interface SliderProps {
	name: string;
	onChange: (value: number) => void;
	value: number;
}

export default function Control({ name, onChange, value }: SliderProps) {
	return (
		<div id={name} className='col-span-1 md:col-span-1'>
			<div className='flex flex-col'>
				<div className='w-full flex'>
					<kbd className='kbd kbd-md text-left'>{name}</kbd>
				</div>
				<div className='divider'>
					<span className='countdown font-mono text-5xl'>{value}</span>
				</div>
			</div>
			<input
				type='range'
				min={0}
				max='10000'
				value={value}
				className='range mt-4 range-xs'
				step='200'
				onChange={(e) => onChange(Number(e.target.value))}
			/>
		</div>
	);
}
