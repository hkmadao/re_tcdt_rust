import { TTree } from '@/models';
import { 
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
export type TFormStore = {
  /**页面状态 */
  status: 'idle' | 'loading' | 'succeeded' | 'failed';
  /**页面配置id */
  idUiConf?: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  fgHidden: boolean;
  editStatusInfo?: {
    id: string;
    editStatus: 'toAdd' | 'added' | 'toEdit' | 'edited' | 'reflesh' | 'addItem' | 'removeItem';
  };
  /**树选中节点 */
  treeSelectedNode?: TTree;
  /**列表页选中的记录 */
  selectedRow?: T{{ bt.tabClassName }};
  /**添加成功的记录 */
  newDataArr: T{{ bt.tabClassName }}[];
  /**编辑成功的记录 */
  editData?: T{{ bt.tabClassName }};
  /**编辑表单数据 */
  formData: T{{ bt.tabClassName }};
}
  {%- endfor %}
{%- endif %}
