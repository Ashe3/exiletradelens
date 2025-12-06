import { PropsWithChildren } from 'react';

export const Content: React.FC<PropsWithChildren> = ({ children }) => (
  <main className="flex-1 overflow-auto p-8">{children}</main>
);
