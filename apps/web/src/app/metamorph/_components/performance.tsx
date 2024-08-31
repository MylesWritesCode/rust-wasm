export interface Performance {
  api?: number; // in ms
}

interface PerformanceCardProps {
  name: string;
  api?: number;
}

export default function PerformanceCard({ name, api }: PerformanceCardProps) {
  return (
    <div className='card bg-info text-info-content shadow-xl col-span-1 rounded-none border-info-content'>
      <div className='card-body'>
        <h2 className='card-title'>{name}</h2>
        <div className='flex flex-col'>
          {api && (
            <div className='w-full flex justify-between items-center'>
              <span className=''>api</span>
              <span className='badge badge-lg'>{api}ms</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
