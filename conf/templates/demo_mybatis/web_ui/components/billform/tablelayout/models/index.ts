{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { TTree } from '~/models';
import { T{{ ht.tabClassName }} } from '../../../../models';
export type TTableStore = {
  /**页面状态 */
  status: 'idle' | 'loading' | 'succeeded' | 'failed';
  /**页面配置id */
  idUiConf?: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  /**当前页面编号 */
  tableData?: T{{ ht.tabClassName }}[];
  selectedRowKeys?: string[];
  selectedTreeNode?: TTree;
  searchData?: { [x in string]: any };
  pageIndex?: number;
  pageSize?: number;
  totalCount?: number;
};
  {%- endfor %}
{%- endif %}
