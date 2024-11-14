import { TDomainStore } from './models';

export const initialState: TDomainStore = {
  status: 'idle',
  pageCode: 'index',
  idUiConf: 'root',
  messages: {
    selectedRow: undefined,
    selectedRows: undefined,
    treeSelectedNode: undefined
  },
  data: {
    selectedRow: undefined,
    selectedRows: [],
    treeSelectedNode: undefined
  }
};
