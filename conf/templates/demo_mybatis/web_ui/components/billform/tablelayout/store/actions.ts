{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { Ref } from "vue";
import { TTableStore } from "../models";
import { TTree } from "~/models";
import { subject } from "../../../../conf";
import { T{{ ht.tabClassName }} } from "../../../../models";

const buildActions = (store: Ref<TTableStore>) => {
  const setComponentInfo = (params: { idUiConf: string; fgDisabled: boolean }) => {
    const { idUiConf, fgDisabled } = params;
    store.value.idUiConf = idUiConf;
    store.value.fgDisabled = fgDisabled;
  }

  const setSelectedTreeNode = (params?: TTree) => {
    store.value.selectedTreeNode = params;
  }

  const setSelectedRowKeys = (params: string[]) => {
    store.value.selectedRowKeys = [...params];
    if (!store.value.selectedRowKeys || store.value.selectedRowKeys.length < 1) {
      return;
    }
    const selectRows =
      store.value.tableData?.filter((d) =>
        store.value.selectedRowKeys?.includes(d.{{ ht.mainProperty }}!),
      ) || [];
    if (selectRows) {
      subject.publish({
        topic: 'selectRows',
        producerId: store.value.idUiConf!,
        data: JSON.parse(JSON.stringify(selectRows)),
      });
    }
  }

  const addNewRecords = (params: T{{ ht.tabClassName }}[]) => {
    if (store.value.tableData) {
      store.value.tableData.unshift(...params);
    }
    const lastKey = params
      .map((entity) => entity.{{ ht.mainProperty }})
      .find((t) => true);
    store.value.selectedRowKeys = [lastKey!];
    if (store.value.totalCount) {
      store.value.totalCount = store.value.totalCount + params.length;
    }
  }

  const updateRecord = (params: T{{ ht.tabClassName }}) => {
    if (store.value.tableData) {
      store.value.tableData = store.value.tableData.map((entity) => {
        if (entity.{{ ht.mainProperty }} === params.{{ ht.mainProperty }}) {
          return { ...entity, ...params };
        }
        return entity;
      });
    }
    const lastKey = params.{{ ht.mainProperty }};
    store.value.selectedRowKeys = [lastKey!];
  }

  return { setComponentInfo, setSelectedTreeNode, setSelectedRowKeys, addNewRecords, updateRecord }
}

export default buildActions;
  {%- endfor %}
{%- endif %}
