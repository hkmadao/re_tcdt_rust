{%- if rootInfo.bTableJson and rootInfo.bTableJson.configForm.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configForm.header %}
import { Ref } from "vue";
import { TFormStore } from "../models";
import { DOStatus, TTree } from "~/models";
import { subject } from "../../../../conf";
import { T{{ ht.tabClassName }} } from "../../../../models";
import { deepCopy } from "~/util";
import { nanoid } from "nanoid";

const buildActions = (store: Ref<TFormStore>) => {
  const setComponentInfo = (params: { idUiConf: string; fgDisabled: boolean }) => {
    const { idUiConf, fgDisabled } = params;
    store.value.idUiConf = idUiConf;
    store.value.fgDisabled = fgDisabled;
  }

  const setSelectedTreeNode = (params?: TTree) => {
    store.value.treeSelectedNode = params;
  }

  const setFormData = (params: T{{ ht.tabClassName }}) => {
    store.value.formData = { ...params };
  }

  const addFormData = (params: { nodeData: any }) => {
    const { nodeData } = params;
    store.value.treeSelectedNode = nodeData;
    store.value.formData = {
      {{ ht.mainProperty }}: nanoid(),
      action: DOStatus.NEW,
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType is matching("Single") %}
      {{ bt.tabCode }}: undefined,
      {%- endif %}
      {%- if bt.refType and bt.refType is matching("Array") %}
      {{ bt.tabCode }}: [],
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
    };
    if (nodeData) {
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
    {%- if bt.billFormFields is iterable %}
      {%- for b in bt.billFormFields %}
        {%- if b.fgTreeAttr and b.refConfig %}
      store.value.formData.{{ b.name }} = nodeData.{{ b.name }};
      store.value.formData.{{ b.refAttributeName }} = deepCopy(nodeData);
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
    }
    store.value.newDataArr = [];
    store.value.editData = undefined;
    store.value.editStatusInfo = {
      id: nanoid(),
      editStatus: 'toAdd',
    };
  }

  const cancel = () => {
    if (store.value.editData) {
      subject.publish({
        topic: 'updateSuccess',
        producerId: store.value.idUiConf!,
        data: deepCopy(store.value.editData),
      });
    }
    if (store.value.newDataArr) {
      subject.publish({
        topic: 'addSuccess',
        producerId: store.value.idUiConf!,
        data: deepCopy(store.value.newDataArr),
      });
    }
    subject.publish({
      topic: '/page/change',
      producerId: store.value.idUiConf!,
      data: 'list',
    });
  }

  const updateFormData = (params: T{{ ht.tabClassName }}) => {
    store.value.formData = {
      ...store.value.formData,
      ...params,
      action: store.value.formData.action !== DOStatus.NEW ? DOStatus.UPDATED : store.value.formData.action,
      {{ ht.mainProperty }}: store.value.formData.{{ ht.mainProperty }},
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType is matching("Single") %}
      {{ bt.tabCode }}: store.value.formData.{{ bt.tabCode }},
      {%- endif %}
      {%- if bt.refType and bt.refType is matching("Array") %}
      {{ bt.tabCode }}: store.value.formData.{{ bt.tabCode }},
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
    };
  }

  return { setComponentInfo, setSelectedTreeNode, setFormData, addFormData, cancel, updateFormData }
}

export default buildActions;
  {%- endfor %}
{%- endif %}
