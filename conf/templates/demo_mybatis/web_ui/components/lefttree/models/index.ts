import { TTree } from '~/models';

export type TLeftTreeStore = {
  /**页面状态 */
  status: 'idle' | 'loading' | 'succeeded' | 'failed';
  /**页面配置id */
  idUiConf?: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  /**树的原始数据 */
  sourchTreeData?: TTree[];
  /**当前树的数据 */
  treeData?: TTree[];
  selectedNode?: TTree;
  selectedKeys: string[];
  expandedKeys: string[];
  foundKeys: string[];
};
