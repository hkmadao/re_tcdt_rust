import * as API from '~/api';
import { TPageInfo, TPageInfoInput } from '~/models';
import { 
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';

{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
const ListAPI = {
  pageList: (params: TPageInfoInput): Promise<TPageInfo<T{{ bt.tabClassName }}>> => {
    return API.POST(`{{ rootInfo.bTableJson.configList.uriConf.page }}`, params);
  },
  getById: (id?: string): Promise<T{{ bt.tabClassName }}> => {
    return API.GET(`{{ rootInfo.bTableJson.configList.uriConf.fetchById }}/${id}`);
  },
  batchRemove: (params: T{{ bt.tabClassName }}[]): Promise<void> => {
    return API.POST(`{{ rootInfo.bTableJson.configList.uriConf.batchRemove }}`, params);
  },
};

export default ListAPI;
  {%- endfor %}
{%- endif %}
