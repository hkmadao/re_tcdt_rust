import {
  createSlice,
  createAsyncThunk,
  PayloadAction,
  configureStore,
} from '@reduxjs/toolkit';
import { initialState } from './initial-state';
import { componentName } from '../../conf';

export const slice = createSlice({
  name: componentName,
  initialState,
  reducers: {
    changePageStatus: (state, action: PayloadAction<string>) => {
      state.pageCode = action.payload;
    },
  },
  extraReducers: (builder) => {},
});

export const actions = slice.actions;
