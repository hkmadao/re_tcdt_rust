import { TTableStore } from '../models';

export const initialState: TTableStore = {
  status: 'idle',
  fgDisabled: false,
  tableData: [],
  selectedRowKeys: [],
  selectedTreeNode: undefined,
  searchData: undefined,
  pageIndex: 1,
  pageSize: 10,
  totalCount: 0,
};
