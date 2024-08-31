import { useMemo } from "react";

// All of these should be in ms
export interface Performance {
  api?: number;
  transform?: number;
}

interface PerformanceCardProps {
  name: string;
  stats: Performance;
}

export default function PerformanceCard({ name, stats }: PerformanceCardProps) {
  const total = useMemo(() => Object.values(stats).reduce((acc, curr) => acc + curr, 0), [stats]);

  return (
    <div className='card bg-info text-info-content shadow-xl col-span-1 rounded-none border-info-content'>
      <div className='card-body'>
        <h2 className='card-title'>{name}</h2>
        <div className='flex flex-col gap-2'>
          {Object.entries(stats).map(([k, v]) => (
            <StatComponent key={k} name={k} value={`${v}ms`} isHidden={!v} />
          ))}
          <StatComponent name='total' value={`${total}ms`} isHidden={total === 0} />
        </div>
      </div>
    </div>
  );
}

interface PerformanceComponentProps {
  name: string;
  value: string;
  isHidden?: boolean;
}

function StatComponent({ name, value, isHidden = true }: PerformanceComponentProps) {
  return (
    <div className='w-full flex justify-between items-center'>
      <span className=''>{name}</span>
      {isHidden ? (
        <span className='loading loading-dots loading-md'>{value}</span>
      ) : (
        <span className='badge badge-lg'>{value}</span>
      )}
    </div>
  );
}
