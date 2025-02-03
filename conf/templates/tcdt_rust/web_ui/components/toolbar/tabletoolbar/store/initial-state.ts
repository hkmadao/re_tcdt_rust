import { TTableToolBarStore } from '../models';

export const initialState: TTableToolBarStore = {
  status: 'idle',
  fgDisabled: false,
  fgHidden: false,
  rowSelectionType: 'radio',
  selectRows: []
};
