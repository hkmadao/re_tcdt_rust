import {
  configureStore,
  createSlice,
  CaseReducer,
  PayloadAction,
  nanoid,
} from '@reduxjs/toolkit';
import { arrToTree, } from '@/util';
import { subject, treeConf, } from '../../../conf';
import { fetchTree, remove, } from './async-thunk';
import { componentName } from '../conf';
import { initialState } from './initial-state';
import * as reducers from './actions';

export * from './async-thunk';

export const leftTreeSlice = createSlice({
  name: componentName,
  initialState: initialState,
  reducers: {
    ...reducers,
  },
  extraReducers: (builder) => {
    builder
      .addCase(fetchTree.pending, (state, action) => {
        subject.publish({
          topic: '/page/addLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(fetchTree.rejected, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(fetchTree.fulfilled, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
        const treeData = arrToTree(treeConf?.firstTreeRef?.parentIdAttr ?? 'idParent', treeConf?.firstTreeRef?.keyAttr!, treeConf?.firstTreeRef?.labelAttr!, action.payload, true, null);
        state.sourchTreeData = treeData;
        state.treeData = treeData;
        state.foundKeys = [];
        if (treeData && treeData.length > 0 && state.expandedKeys.length === 0) {
          state.expandedKeys = [treeData[0].key];
        }
      })
      .addCase(remove.pending, (state, action) => {
        subject.publish({
          topic: '/page/addLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(remove.rejected, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(remove.fulfilled, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
        const treeData = arrToTree(treeConf?.firstTreeRef?.parentIdAttr ?? 'idParent', treeConf?.firstTreeRef?.keyAttr!, treeConf?.firstTreeRef?.labelAttr!, action.payload, true, null);
        state.sourchTreeData = treeData;
        state.treeData = treeData;
        if (treeData && treeData.length > 0 && state.expandedKeys.length === 0) {
          state.expandedKeys = [treeData[0].key];
        }
      });
  },
});

export const actions = leftTreeSlice.actions;

const reducer: any = {};
reducer[leftTreeSlice.name] = leftTreeSlice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});