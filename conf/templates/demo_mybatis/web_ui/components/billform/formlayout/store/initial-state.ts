import { TFormStore } from '../models';

export const initialState: TFormStore = {
  status: 'idle',
  fgDisabled: false,
  newDataArr: [],
  formData: {},
};
