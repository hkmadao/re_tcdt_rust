import * as API from '~/api';
import { TPageInfo, TPageInfoInput } from '~/models';
import { 
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
const FormAPI = {
  getById: (id?: string): Promise<T{{ bt.tabClassName }}> => {
    return API.GET(`{{ rootInfo.bJson.configForm.uriConf.fetchById }}/${id}`);
  },
  add: (params: T{{ bt.tabClassName }}): Promise<T{{ bt.tabClassName }}> => {
    {%- if rootInfo.bJson.billFormType and rootInfo.bJson.billFormType is matching("Single") %}
    return API.POST(`{{ rootInfo.bJson.configForm.uriConf.save }}`, params);
    {%- endif %}
    {%- if rootInfo.bJson.billFormType and rootInfo.bJson.billFormType is matching("Combination") %}
    return API.POST(`{{ rootInfo.bJson.configForm.uriConf.save }}`, params);
    {%- endif %}
  },
  update: (params: T{{ bt.tabClassName }}): Promise<T{{ bt.tabClassName }}> => {
    {%- if rootInfo.bJson.billFormType and rootInfo.bJson.billFormType is matching("Single") %}
    return API.POST(`{{ rootInfo.bJson.configForm.uriConf.update }}`, params);
    {%- endif %}
    {%- if rootInfo.bJson.billFormType and rootInfo.bJson.billFormType is matching("Combination") %}
    return API.POST(`{{ rootInfo.bJson.configForm.uriConf.update }}`, params);
    {%- endif %}
  },
  remove: (params: T{{ bt.tabClassName }}): Promise<void> => {
    return API.POST(`{{ rootInfo.bJson.configForm.uriConf.dataRemove }}`, params);
  },
};

export default FormAPI;
  {%- endfor %}
{%- endif %}
