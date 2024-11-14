{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { TTree } from '~/models';
import { T{{ ht.tabClassName }} } from '../../../../models';
export type TFormStore = {
  /**页面状态 */
  status: 'idle' | 'loading' | 'succeeded' | 'failed';
  /**页面配置id */
  idUiConf?: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  editStatusInfo?: {
    id: string;
    editStatus:
      | 'toAdd'
      | 'added'
      | 'toEdit'
      | 'edited'
      | 'reflesh'
      | 'addItem'
      | 'removeItem';
  };
  /**树选中节点 */
  treeSelectedNode?: TTree;
  /**列表页选中的记录 */
  selectedRow?: T{{ ht.tabClassName }};
  /**添加成功的记录 */
  newDataArr: T{{ ht.tabClassName }}[];
  /**编辑成功的记录 */
  editData?: T{{ ht.tabClassName }};
  /**编辑表单数据 */
  formData: T{{ ht.tabClassName }};
};
  {%- endfor %}
{%- endif %}
