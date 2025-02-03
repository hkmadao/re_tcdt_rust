import { TTree } from '@/models';
import { Key } from 'react';

export type TTableToolBarStore = {
  /**页面状态 */
  status: 'idle' | 'loading' | 'succeeded' | 'failed';
  /**页面配置id */
  idUiConf?: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  fgHidden: boolean;
  rowSelectionType: 'radio' | 'checkbox';
  selectRows: any[];
  nodeTreeData?: TTree;
};