import {
  configureStore,
  createSlice,
  CaseReducer,
  PayloadAction,
  nanoid,
} from '@reduxjs/toolkit';
import { fetchByTreeNode, search, reflesh, pageChange, batchRemove } from './async-thunk';
import { componentName } from '../conf';
import { initialState } from './initial-state';
import * as reducers from './actions';
import { subject } from '../../../../conf';

export * from './async-thunk';

export const tableSlice = createSlice({
  name: componentName,
  initialState: initialState,
  reducers: {
    ...reducers,
  },
  extraReducers: (builder) => {
    builder
      .addCase(fetchByTreeNode.pending, (state, action) => { })
      .addCase(fetchByTreeNode.rejected, (state, action) => { })
      .addCase(fetchByTreeNode.fulfilled, (state, action) => {
        if (!action.payload) {
          return;
        }
        const { selectedTreeNode, pageInfo } = action.payload;
        state.pageIndex = pageInfo.pageInfoInput.pageIndex;
        state.pageSize = pageInfo.pageInfoInput.pageSize;
        state.totalCount = pageInfo.pageInfoInput.totalCount;
        state.tableData = pageInfo.dataList;
        state.selectedTreeNode = selectedTreeNode;
        state.selectedRowKeys  = [];
        state.searchData = undefined;
        subject.publish({
          topic: 'listReload',
          producerId: state.idUiConf!,
          data: undefined,
        });
      })
      .addCase(search.pending, (state, action) => { })
      .addCase(search.rejected, (state, action) => { })
      .addCase(search.fulfilled, (state, action) => {
        if (!action.payload) {
          return;
        }
        const { searchData, pageInfo } = action.payload;
        state.pageIndex = pageInfo.pageInfoInput.pageIndex;
        state.pageSize = pageInfo.pageInfoInput.pageSize;
        state.totalCount = pageInfo.pageInfoInput.totalCount;
        state.tableData = pageInfo.dataList;
        // state.selectedTreeNode = selectedTreeNode;
        state.selectedRowKeys  = [];
        state.searchData = searchData;
        subject.publish({
          topic: 'listReload',
          producerId: state.idUiConf!,
          data: undefined,
        });
      })
      .addCase(reflesh.pending, (state, action) => { })
      .addCase(reflesh.rejected, (state, action) => { })
      .addCase(reflesh.fulfilled, (state, action) => {
        if (!action.payload) {
          return;
        }
        const pageInfo = action.payload;
        state.pageIndex = pageInfo.pageInfoInput.pageIndex;
        state.pageSize = pageInfo.pageInfoInput.pageSize;
        state.totalCount = pageInfo.pageInfoInput.totalCount;
        state.tableData = pageInfo.dataList;
        // state.selectedTreeNode = selectedTreeNode;
        state.selectedRowKeys  = [];
        // state.searchData = undefined;
        subject.publish({
          topic: 'listReload',
          producerId: state.idUiConf!,
          data: undefined,
        });
      })
      .addCase(pageChange.pending, (state, action) => { })
      .addCase(pageChange.rejected, (state, action) => { })
      .addCase(pageChange.fulfilled, (state, action) => {
        if (!action.payload) {
          return;
        }
        const pageInfo = action.payload;
        state.pageIndex = pageInfo.pageInfoInput.pageIndex;
        state.pageSize = pageInfo.pageInfoInput.pageSize;
        state.totalCount = pageInfo.pageInfoInput.totalCount;
        state.tableData = pageInfo.dataList;
        // state.selectedTreeNode = selectedTreeNode;
        state.selectedRowKeys  = [];
        // state.searchData = undefined;
        subject.publish({
          topic: 'listReload',
          producerId: state.idUiConf!,
          data: undefined,
        });
      })
      .addCase(batchRemove.pending, (state, action) => { })
      .addCase(batchRemove.rejected, (state, action) => { })
      .addCase(batchRemove.fulfilled, (state, action) => {
        if (!action.payload) {
          return;
        }
        const pageInfo = action.payload;
        state.pageIndex = pageInfo.pageInfoInput.pageIndex;
        state.pageSize = pageInfo.pageInfoInput.pageSize;
        state.totalCount = pageInfo.pageInfoInput.totalCount;
        state.tableData = pageInfo.dataList;
        // state.selectedTreeNode = selectedTreeNode;
        state.selectedRowKeys  = [];
        // state.searchData = undefined;
        subject.publish({
          topic: 'listReload',
          producerId: state.idUiConf!,
          data: undefined,
        });
      });
  },
});

export const actions = tableSlice.actions;

const reducer:any = {};
reducer[tableSlice.name] = tableSlice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});
