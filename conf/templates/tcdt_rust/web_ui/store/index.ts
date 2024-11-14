import {
  configureStore,
} from '@reduxjs/toolkit';
import { slice } from './slice';

const reducer:any = {};
reducer[slice.name] = slice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});
