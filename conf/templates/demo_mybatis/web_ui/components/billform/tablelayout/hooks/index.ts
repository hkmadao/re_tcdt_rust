{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { useTableStore } from '../store';

const { store } = useTableStore()

export const useIdUiConf = () => {
  return store.idUiConf!
};


export const useFgDisabled = () => {
  return store.fgDisabled
};

export const useTableData = () => {
  return store.tableData
};

export const usePageInfo = () => {
  return {
    pageIndex: store.pageIndex,
    pageSize: store.pageSize,
    totalCount: store.totalCount,
  }
};

export const useSelectRow = () => {
  if (
    !store.selectedRowKeys || store.selectedRowKeys.length !== 1
  ) {
    return;
  }
  return store.tableData?.find((d) =>
    store.selectedRowKeys?.includes(d.{{ ht.mainProperty }}!),
  );
};

export const useSelectRowKeys = () => {
  return store.selectedRowKeys ?? []
};
  {%- endfor %}
{%- endif %}
