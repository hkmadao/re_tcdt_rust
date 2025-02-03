import { useSelector, } from 'react-redux';
import { TTableStore } from '../models';
import { componentName } from '../conf';

export * from './columns';

{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
const selectStatus = (state: { [x: string]: TTableStore }) => {
  return state[componentName].status;
};

const selectStoreData = (state: { [x: string]: TTableStore }) => {
  return state[componentName];
};

const selectSelectRow = (state: { [x: string]: TTableStore }) => {
  if (!state[componentName].selectedRowKeys || state[componentName].selectedRowKeys.length !== 1) {
    return;
  }
  return state[componentName].tableData?.find(d => state[componentName].selectedRowKeys?.includes(d.{{ bt.mainProperty }}!));
};

const selectIdUiConf = (state: { [x: string]: TTableStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useFgDisabled = () => {
  return useSelector((state: { [x: string]: TTableStore }) => {
    return state[componentName].fgDisabled;
  });
}

export const useFgHidden = () => {
  return useSelector((state: { [x: string]: TTableStore }) => {
    return state[componentName].fgHidden;
  });
}

export const useStoreData = () => {
  return useSelector(selectStoreData);
}

export const useSelectRow = () => {
  return useSelector(selectSelectRow);
}
  {%- endfor %}
{%- endif %}

{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType == "Array" %}
/*=========={{ bt.firstUpperTabCode }}=============*/
export const use{{ bt.firstUpperTabCode }}Data = () => {
  return useSelector((state: { [x: string]: TTableStore }) => {
    if (!state[componentName].selectedRowKeys || state[componentName].selectedRowKeys.length !== 1) {
      return [];
    }
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
    const record = state[componentName].tableData?.find(d => state[componentName].selectedRowKeys?.includes(d.{{ ht.mainProperty }}!));
  {%- endfor %}
{%- endif %}
    if(!record?.{{ bt.tabAttrName }}){
      return []
    }
    return record.{{ bt.tabAttrName }};
  });
}
/*=========={{ bt.firstUpperTabCode }}=============*/
    {%- endif %}
    {%- if bt.refType and bt.refType == "Single" %}
/*=========={{ bt.firstUpperTabCode }}=============*/
export const use{{ bt.firstUpperTabCode }}Data = () => {
  return useSelector((state: { [x: string]: TTableStore }) => {
    if (!state[componentName].selectedRowKeys || state[componentName].selectedRowKeys.length !== 1) {
      return [];
    }
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
  const record = state[componentName].tableData?.find(d => state[componentName].selectedRowKeys?.includes(d.{{ ht.mainProperty }}!));
  {%- endfor %}
{%- endif %}
    if(!record?.{{ bt.tabAttrName }}){
      return []
    }
    return [ record.{{ bt.tabAttrName }} ];
  });
}
/*=========={{ bt.firstUpperTabCode }}=============*/
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
