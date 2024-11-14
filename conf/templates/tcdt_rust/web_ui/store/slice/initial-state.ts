import { TDomainStore } from './models';

export const initialState: TDomainStore = {
  status: 'idle',
{%- if rootInfo.uiJson.pages is iterable %}
  {%- for page in rootInfo.uiJson.pages %}
    {%- if loop.first %}
  pageCode: '{{ page.code }}',
    {%- endif %}
  {%- endfor %}
{%- endif %}
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
