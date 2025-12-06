import { createUserSlice, UserSlice } from './userSlice';
import { createWithEqualityFn } from 'zustand/traditional';

export type StoreState = UserSlice;

export const useStore = createWithEqualityFn<StoreState>()((...a) => ({
  ...createUserSlice(...a),
}));
