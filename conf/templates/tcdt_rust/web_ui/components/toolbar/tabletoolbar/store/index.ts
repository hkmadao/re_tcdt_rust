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

export const tableToolBarSlice = createSlice({
  name: componentName,
  initialState: initialState,
  reducers: {
    ...reducers,
  },
});

export const actions = tableToolBarSlice.actions;

const reducer: any = {};
reducer[tableToolBarSlice.name] = tableToolBarSlice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});