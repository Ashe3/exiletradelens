import { PropsWithChildren } from 'react';

export const Sidebar: React.FC<PropsWithChildren> = ({ children }) => (
  <nav className="py-8 bg-linear-to-b from-gray-100 via-gray-200 to-gray-100 border-r border-gray-300">
    {children}
  </nav>
);
