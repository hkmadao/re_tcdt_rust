
import { useSelector, } from 'react-redux';
import { DOStatus } from '@/models';
import { TFormStore, } from '../models';
import { componentName } from '../conf';

export * from './columns';

export const useEditStatusInfo = () => {
  return useSelector((state: { [x: string]: TFormStore }) => {
    return state[componentName].editStatusInfo;
  });
}

const selectIdUiConf = (state: { [x: string]: TFormStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useFgDisabled = () => {
  return useSelector((state: { [x: string]: TFormStore }) => {
    return state[componentName].fgDisabled;
  });
}

export const useFgHidden = () => {
  return useSelector((state: { [x: string]: TFormStore }) => {
    return state[componentName].fgHidden;
  });
}

const selectStore = (state: { [x: string]: TFormStore }) => {
  return state[componentName].formData;
};

export const useFormData = () => {
  return useSelector(selectStore);
}

{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType == "Array" %}
/*=========={{ bt.firstUpperTabCode }}=============*/
export const use{{ bt.firstUpperTabCode }}Data = () => {
  return useSelector((state: { [x: string]: TFormStore }) => {
    if (!state[componentName].formData?.{{ bt.tabAttrName }}) {
      return [];
    }
    return state[componentName].formData.{{ bt.tabAttrName }}.filter({{ bt.firstLowerTabClassName }} => {{ bt.firstLowerTabClassName }}.action !== DOStatus.DELETED);
  });
}
/*=========={{ bt.firstUpperTabCode }}=============*/
      {%- endif %}
      {%- if bt.refType and bt.refType == "Single" %}
/*=========={{ bt.firstUpperTabCode }}=============*/
export const use{{ bt.firstUpperTabCode }}Data = () => {
  return useSelector((state: { [x: string]: TFormStore }) => {
    if (!state[componentName].formData?.{{ bt.tabAttrName }} || state[componentName].formData?.{{ bt.tabAttrName }}.action === DOStatus.DELETED) {
      return [];
    }
    return [ state[componentName].formData.{{ bt.tabAttrName }} ];
  });
}
/*=========={{ bt.firstUpperTabCode }}=============*/
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
