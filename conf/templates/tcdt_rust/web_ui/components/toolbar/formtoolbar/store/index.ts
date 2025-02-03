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

export const formToolBarSlice = createSlice({
  name: componentName,
  initialState: initialState,
  reducers: {
    ...reducers,
  },
});

export const actions = formToolBarSlice.actions;

const reducer: any = {};
reducer[formToolBarSlice.name] = formToolBarSlice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});