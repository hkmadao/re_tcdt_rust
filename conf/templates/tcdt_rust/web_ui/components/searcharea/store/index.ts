import {
  configureStore,
  createSlice,
  CaseReducer,
  PayloadAction,
  nanoid,
} from '@reduxjs/toolkit';
import { componentName } from '../conf';
import { initialState } from './initial-state';
import * as reducers from './actions';

export * from './async-thunk';

export const searchAreaSlice = createSlice({
  name: componentName,
  initialState: initialState,
  reducers: {
    ...reducers,
  },
});

export const actions = searchAreaSlice.actions;

const reducer: any = {};
reducer[searchAreaSlice.name] = searchAreaSlice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});