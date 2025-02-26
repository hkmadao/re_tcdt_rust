import { TTableStore } from '../models';

export const initialState: TTableStore = {
  status: 'idle',
  tableData: [],
  selectedRowKeys: [],
  selectedTreeNode: undefined,
  searchData: {},
  pageIndex: 1,
  pageSize: 10,
  totalCount: 0,
  fgDisabled: false,
  fgHidden: false
};
