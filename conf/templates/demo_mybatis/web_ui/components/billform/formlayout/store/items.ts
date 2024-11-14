import { DOStatus } from '~/models';
import { Ref } from "vue";
import { TFormStore } from "../models";
import { 
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.body %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';

const buildItemActions = (store: Ref<TFormStore>) => {

{%- if rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType is matching("Array") %}
/*=========={{ bt.tabClassName }}=============*/
  const add{{ bt.tabClassName }}s = (subItem: T{{ bt.tabClassName }}) => {
    const new{{ bt.tabClassName }}: T{{ bt.tabClassName }} = { ...subItem, action: DOStatus.NEW }
    store.value.formData.{{ bt.firstLowerTabClassName }}s?.push(new{{ bt.tabClassName }})
    store.value.formData.action === DOStatus.UNCHANGED ? store.value.formData.action = DOStatus.UPDATED : undefined
  }

  const update{{ bt.tabClassName }}s = (subItem: T{{ bt.tabClassName }}) => {
    const new{{ bt.tabClassName }}s = store.value.formData.{{ bt.firstLowerTabClassName }}s?.map(c => {
      if (c.{{ bt.mainProperty }} === subItem.{{ bt.mainProperty }} && c.action !== DOStatus.DELETED) {
        const new{{ bt.tabClassName }} = { ...c, ...subItem }
        if (c.action === DOStatus.UNCHANGED) {
          new{{ bt.tabClassName }}.action = DOStatus.UPDATED
        }
        return new{{ bt.tabClassName }}
      }
      return c
    })
    store.value.formData.{{ bt.firstLowerTabClassName }}s = new{{ bt.tabClassName }}s
    store.value.formData.action === DOStatus.UNCHANGED ? store.value.formData.action = DOStatus.UPDATED : undefined
  }

  const delete{{ bt.tabClassName }}s = (subItem: T{{ bt.tabClassName }}) => {
    const filters = store.value.formData.{{ bt.firstLowerTabClassName }}s?.filter(c => {
      if (c.{{ bt.mainProperty }} !== subItem.{{ bt.mainProperty }}) {
        return true;
      }
      if (c.action !== DOStatus.NEW) {
        return true;
      }
      return false;
    }) || []
    const r = filters.map(c => {
      if (c.{{ bt.mainProperty }} === subItem.{{ bt.mainProperty }}) {
        return { ...c, action: DOStatus.DELETED }
      }
      return c;
    })
    store.value.formData.{{ bt.firstLowerTabClassName }}s = r;
    store.value.formData.action === DOStatus.UNCHANGED ? store.value.formData.action = DOStatus.UPDATED : undefined
  }

/*=========={{ bt.tabClassName }}=============*/
      {%- endif %}
      {%- if bt.refType and bt.refType is matching("Single") %}
/*=========={{ bt.tabClassName }}=============*/
  const add{{ bt.tabClassName }} = (subItem: T{{ bt.tabClassName }}) => {
    store.value.formData.{{ bt.firstLowerTabClassName }} = { ...subItem, action: DOStatus.NEW }
    store.value.formData.action === DOStatus.UNCHANGED ? store.value.formData.action = DOStatus.UPDATED : undefined
  }

  const update{{ bt.tabClassName }} = (subItem: T{{ bt.tabClassName }}) => {
    const nwe{{ bt.tabClassName }} = { ...store.value.formData.{{ bt.firstLowerTabClassName }}, ...subItem }
    if (store.value.formData.{{ bt.firstLowerTabClassName }}?.action === DOStatus.UNCHANGED) {
      nwe{{ bt.tabClassName }}.action = DOStatus.UPDATED
    }
    store.value.formData.{{ bt.firstLowerTabClassName }} = nwe{{ bt.tabClassName }}
    store.value.formData.action === DOStatus.UNCHANGED ? store.value.formData.action = DOStatus.UPDATED : undefined
  }

  const delete{{ bt.tabClassName }} = () => {
    if (!store.value.formData.{{ bt.firstLowerTabClassName }}) {
      return
    }
    if (store.value.formData.{{ bt.firstLowerTabClassName }}.action === DOStatus.NEW) {
      store.value.formData.{{ bt.firstLowerTabClassName }} = undefined
    } else {
      store.value.formData.{{ bt.firstLowerTabClassName }}.action = DOStatus.DELETED
    }
    store.value.formData.action === DOStatus.UNCHANGED ? store.value.formData.action = DOStatus.UPDATED : undefined
  }
/*=========={{ bt.tabClassName }}=============*/
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  return {
{%- if rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType is matching("Array") %}
    add{{ bt.tabClassName }}s,
    update{{ bt.tabClassName }}s,
    delete{{ bt.tabClassName }}s,
        {%- endif %}
        {%- if bt.refType and bt.refType is matching("Single") %}
    add{{ bt.tabClassName }},
    update{{ bt.tabClassName }},
    delete{{ bt.tabClassName }},
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  }

}

export default buildItemActions
