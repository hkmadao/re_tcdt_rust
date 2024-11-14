{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { Ref, } from "vue";
import { TFormStore, } from "../models";
import { DOStatus, } from "~/models";
import { subject, } from "../../../../conf";
import {   T{{ ht.tabClassName }} } from "../../../../models";
import { deepCopy } from "~/util";
import FormAPI from "../api";
import { nanoid } from "nanoid";
import { ElMessage } from "element-plus";

const buildAsyncThunks = (store: Ref<TFormStore>) => {

  const toEditThunks = (params: { nodeData: any; selectedRow:   T{{ ht.tabClassName }} }) => {
    const { nodeData, selectedRow } = params;
    getById(selectedRow.{{ ht.mainProperty }}).then((detailData) => {
      store.value.treeSelectedNode = nodeData;
      store.value.selectedRow = detailData;
      store.value.formData = detailData;
      store.value.newDataArr = [];
      store.value.editData = undefined;
      store.value.editStatusInfo = {
        id: nanoid(),
        editStatus: 'toEdit',
      };
    }).catch(err => {
      console.log(err)
    })
  }

  const refleshThunks = () => {
    getById(store.value.selectedRow?.{{ ht.mainProperty }}).then((res) => {
      store.value.formData = res;
      store.value.editStatusInfo = {
        id: nanoid(),
        editStatus: 'reflesh',
      };
    }).catch(err => {
      console.log(err)
    })
  }

  const saveThunks = (params: { actionType: 'add' | 'addAgain' | 'edit' }) => {
    save(store, params).then((res) => {
      const { actionType, saveData } = res;
      if (actionType === 'add') {
        store.value.newDataArr.push(saveData);
        store.value.formData = saveData;

        ElMessage.info('新增成功！');
        subject.publish({
          topic: 'addSuccess',
          producerId: store.value.idUiConf!,
          data: deepCopy(store.value.newDataArr),
        });
        store.value.editStatusInfo = {
          id: nanoid(),
          editStatus: 'added',
        };
        subject.publish({
          topic: '/page/change',
          producerId: store.value.idUiConf!,
          data: 'list',
        });
      }
      if (actionType === 'addAgain') {
        store.value.newDataArr.push(saveData);
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
        if (store.value.treeSelectedNode) {
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
    {%- if bt.billFormFields is iterable %}
      {%- for b in bt.billFormFields %}
        {%- if b.fgTreeAttr and b.refConfig %}
          store.value.formData.{{ b.name }} = store.value.treeSelectedNode.{{ b.refConfig.backWriteProp }};
          store.value.formData.{{ b.refAttributeName }} = deepCopy(store.value.treeSelectedNode);
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
        }

        ElMessage.info('新增成功！');
        store.value.editStatusInfo = {
          id: nanoid(),
          editStatus: 'toAdd',
        };
      }
      if (actionType === 'edit') {
        store.value.editData = saveData;
        store.value.formData = saveData;

        ElMessage.info('更新成功！');
        subject.publish({
          topic: 'updateSuccess',
          producerId: store.value.idUiConf!,
          data: deepCopy(saveData),
        });
        store.value.editStatusInfo = {
          id: nanoid(),
          editStatus: 'edited',
        };
        subject.publish({
          topic: '/page/change',
          producerId: store.value.idUiConf!,
          data: 'list',
        });
      }
    }).catch(err => {
      console.log(err)
    })
  }

  return { toEditThunks, refleshThunks, saveThunks, }
}

export default buildAsyncThunks;

const getById = async (id?: string) => {
  if (!id) {
    return new Promise<  T{{ ht.tabClassName }}>((resolve, reject) => {
      reject('没有参数')
    });
  }
  const view = await FormAPI.getById(id)
  return view;
};

const save = async (store: Ref<TFormStore>, params: { actionType: 'add' | 'addAgain' | 'edit' }) => {
  const { actionType } = params;
  if (actionType === 'add' || actionType === 'addAgain') {
    const saveData: T{{ ht.tabClassName }} = await FormAPI.add(store.value.formData);
    return {
      actionType,
      saveData,
    };
  }
  const saveData: T{{ ht.tabClassName }} = await FormAPI.update(store.value.formData);
  return {
    actionType,
    saveData,
  };
}
  {%- endfor %}
{%- endif %}
