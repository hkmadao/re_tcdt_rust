import { TLeftTreeStore } from '../models';

export const initialState: TLeftTreeStore = {
  status: 'idle',
  selectedKeys: [],
  expandedKeys: [],
  foundKeys: [],
  fgDisabled: false,
  fgInnerDisabled: false
};
