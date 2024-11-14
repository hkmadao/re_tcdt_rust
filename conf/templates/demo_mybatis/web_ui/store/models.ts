type TMessage = {
  id: string;
  created: string;
  customIds: string[];
};

type TMessageMap = {
  selectedRow?: TMessage;
  selectedRows?: TMessage;
  treeSelectedNode?: TMessage;
};

type TEventData = {
  selectedRow: any;
  selectedRows: any[];
  treeSelectedNode: any;
};

export type TDomainStore = {
  /**页面状态 */
  status: 'idle' | 'loading' | 'succeeded' | 'failed';
  /**页面配置id */
  idUiConf: string;
  /**当前页面编号 */
  pageCode?: string;
  messages: TMessageMap;
  data: TEventData;
};
