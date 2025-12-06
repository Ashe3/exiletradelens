import React from 'react';
import { SwitchVersion } from './SwitchVersion';

export const Header: React.FC = () => {
  return (
    <header className="flex items-center justify-between px-4 py-2 border-b bg-white dark:bg-neutral-900">
      <div className="font-bold text-lg">ExileTradeLens</div>
      <div className="flex items-center gap-4">
        <SwitchVersion />
        <div className="flex items-center gap-2">
          <span className="flex w-8 h-8 rounded-full bg-gray-300 items-center justify-center text-gray-700">
            G
          </span>
        </div>
      </div>
    </header>
  );
};
