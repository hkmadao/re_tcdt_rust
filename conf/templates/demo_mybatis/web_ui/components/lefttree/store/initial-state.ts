import { TLeftTreeStore } from '../models';

export const initialState: TLeftTreeStore = {
  status: 'idle',
  fgDisabled: false,
  selectedKeys: [],
  expandedKeys: [],
  foundKeys: []
};
