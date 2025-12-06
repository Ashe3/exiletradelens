import { StateCreator } from 'zustand';

interface User {
  id: string;
  name: string;
}

export interface UserSlice {
  user: User | null;
  poeVersion: 'poe1' | 'poe2';
  setUser: (user: User | null) => void;
  setPoeVersion: (version: 'poe1' | 'poe2') => void;
}

export const createUserSlice: StateCreator<UserSlice> = (set) => ({
  user: null,
  poeVersion: 'poe2',
  setUser: (user) => set({ user }),
  setPoeVersion: (version) => set({ poeVersion: version }),
});
