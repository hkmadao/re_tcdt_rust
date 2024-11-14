{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { Ref, } from "vue";
import { TTableStore } from "../models";
import { EDirection, ELogicOperatorCode, EOperatorCode, TFilterNode, TPageInfo, TPageInfoInput, TTree, } from "~/models";
import { componentName, queryConf, subject, } from "../../../../conf";
import { T{{ ht.tabClassName }} } from "../../../../models";
import { TMessage } from "~/util";
import ListAPI from "../api";
const searcheRefs = queryConf?.searchRefs;

const oneParamConds = [
  EOperatorCode.LIKE_FULL,
  EOperatorCode.LIKE_RIGHT,
  EOperatorCode.LIKE_LEFT,
  EOperatorCode.EQUAL,
  EOperatorCode.GREATER_THAN,
  EOperatorCode.LESS_THAN,
];

const buildAsyncThunks = (store: Ref<TTableStore>) => {

  const fetchByTreeNodeThunks = (message: TMessage) => {
    fetchByTreeNode(message).then((res) => {
      const { selectedTreeNode, pageInfo } = res;
      store.value.pageIndex = pageInfo.pageInfoInput.pageIndex;
      store.value.pageSize = pageInfo.pageInfoInput.pageSize;
      store.value.totalCount = pageInfo.pageInfoInput.totalCount;
      store.value.tableData = pageInfo.dataList;
      store.value.selectedTreeNode = selectedTreeNode;
      store.value.selectedRowKeys = [];
      store.value.searchData = undefined;
      subject.publish({
        topic: 'listReload',
        producerId: store.value.idUiConf!,
        data: undefined,
      });
    }).catch(err => {
      console.log(err)
    })
  }

  const searchThunks = (message: TMessage) => {
    search(message).then((res) => {
      const { searchData, pageInfo } = res;
      store.value.pageIndex = pageInfo.pageInfoInput.pageIndex;
      store.value.pageSize = pageInfo.pageInfoInput.pageSize;
      store.value.totalCount = pageInfo.pageInfoInput.totalCount;
      store.value.tableData = pageInfo.dataList;
      // state.selectedTreeNode = selectedTreeNode;
      store.value.selectedRowKeys = [];
      store.value.searchData = searchData;
      subject.publish({
        topic: 'listReload',
        producerId: store.value.idUiConf!,
        data: undefined,
      });
    }).catch(err => {
      console.log(err)
    })
  }

  const refleshThunks = () => {
    reflesh(store).then((res) => {
      const pageInfo = res;
      store.value.pageIndex = pageInfo.pageInfoInput.pageIndex;
      store.value.pageSize = pageInfo.pageInfoInput.pageSize;
      store.value.totalCount = pageInfo.pageInfoInput.totalCount;
      store.value.tableData = pageInfo.dataList;
      // state.selectedTreeNode = selectedTreeNode;
      store.value.selectedRowKeys = [];
      // state.searchData = undefined;
      subject.publish({
        topic: 'listReload',
        producerId: store.value.idUiConf!,
        data: undefined,
      });
    }).catch(err => {
      console.log(err)
    })
  }

  const pageChangeThunks = (params: { page: number; pageSize: number }) => {
    pageChange(store, params).then((res) => {
      const pageInfo = res;
      store.value.pageIndex = pageInfo.pageInfoInput.pageIndex;
      store.value.pageSize = pageInfo.pageInfoInput.pageSize;
      store.value.totalCount = pageInfo.pageInfoInput.totalCount;
      store.value.tableData = pageInfo.dataList;
      // store.value.selectedTreeNode = selectedTreeNode;
      store.value.selectedRowKeys = [];
      // store.value.searchData = undefined;
      subject.publish({
        topic: 'listReload',
        producerId: store.value.idUiConf!,
        data: undefined,
      });
    }).catch(err => {
      console.log(err)
    })
  }

  const batchRemoveThunks = (message: TMessage) => {
    batchRemove(store, message).then((res) => {
      const pageInfo = res;
      store.value.pageIndex = pageInfo.pageInfoInput.pageIndex;
      store.value.pageSize = pageInfo.pageInfoInput.pageSize;
      store.value.totalCount = pageInfo.pageInfoInput.totalCount;
      store.value.tableData = pageInfo.dataList;
      // store.value.selectedTreeNode = selectedTreeNode;
      store.value.selectedRowKeys = [];
      // store.value.searchData = undefined;
      subject.publish({
        topic: 'listReload',
        producerId: store.value.idUiConf!,
        data: undefined,
      });
    }).catch(err => {
      console.log(err)
    })
  }

  return { fetchByTreeNodeThunks, searchThunks, refleshThunks, pageChangeThunks, batchRemoveThunks }
}

export default buildAsyncThunks;

type TFetchByTreeNodeResult = {
  selectedTreeNode: any;
  pageInfo: TPageInfo<T{{ ht.tabClassName }}>
}

type TSearchResult = {
  searchData: any;
  pageInfo: TPageInfo<T{{ ht.tabClassName }}>
}

const fetchByTreeNode = async (message: TMessage) => {
  if (!message) {
    return new Promise<TFetchByTreeNodeResult>((resolve, reject) => {
      reject('没有参数，或消息已被消费')
    });
  }
  const selectedTreeNode: TTree = message.data as TTree;
  const fns: TFilterNode[] = [];
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.billFormFields is iterable %}
      {%- for b in bt.billFormFields %}
        {%- if b.fgTreeAttr and b.refConfig %}
        if(selectedTreeNode){
          const treeIdFn: TFilterNode = {
            name: '{{ b.name }}',
            operatorCode: EOperatorCode.EQUAL,
            filterParams: [selectedTreeNode['{{ b.refConfig.backWriteProp }}']],
          };
          fns.push(treeIdFn);
        }
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  const params: TPageInfoInput = {
    pageIndex: 1,
    pageSize: 10,
    logicNode: {
      logicOperatorCode: ELogicOperatorCode.AND,
      filterNodes: fns,
    },
    orders: [
      {
        property: '{{ ht.mainProperty }}',
        direction: EDirection.ASC,
        ignoreCase: false,
      },
    ],
  };
  const pageInfo: TPageInfo<T{{ ht.tabClassName }}> = await ListAPI.pageList(params);
  return {
    selectedTreeNode,
    pageInfo,
  };
};

const search = async (message: TMessage) => {
  if (!message || message.consumerIds.includes(componentName)) {
    return new Promise<TSearchResult>((resolve, reject) => {
      reject('没有参数，或消息已被消费')
    });
  }
  const searchData = message.data;
  const fns: TFilterNode[] = [];
  if (searchData) {
    searcheRefs.forEach((searcheRef) => {
      if (searchData[searcheRef.attributeName!]) {
        if (
          searcheRef.operatorCode &&
          oneParamConds.includes(searcheRef.operatorCode)
        ) {
          const fn: TFilterNode = {
            name: searcheRef.attributeName!,
            operatorCode: searcheRef.operatorCode,
            filterParams: [searchData[searcheRef.attributeName!]],
          };
          fns.push(fn);
        }
      }
    });
  }
  const params: TPageInfoInput = {
    pageIndex: 1,
    pageSize: 10,
    logicNode: {
      logicOperatorCode: ELogicOperatorCode.AND,
      filterNodes: fns,
    },
    orders: [
      {
        property: '{{ ht.mainProperty }}',
        direction: EDirection.ASC,
        ignoreCase: false,
      },
    ],
  };
  const pageInfo: TPageInfo<T{{ ht.tabClassName }}> = await ListAPI.pageList(params);
  return { searchData, pageInfo, };
}

const reflesh = async (store: Ref<TTableStore>) => {
  const searchData = store.value.searchData;
  const fns: TFilterNode[] = [];
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.billFormFields is iterable %}
      {%- for b in bt.billFormFields %}
        {%- if b.fgTreeAttr and b.refConfig %}
        if(store.value.selectedTreeNode){
          const treeIdFn: TFilterNode = {
            name: '{{ b.name }}',
            operatorCode: EOperatorCode.EQUAL,
            filterParams: [store.value.selectedTreeNode['{{ b.refConfig.backWriteProp }}']],
          };
          fns.push(treeIdFn);
        }
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  if (searcheRefs && searchData) {
    searcheRefs.forEach((searcheRef) => {
      if (searchData[searcheRef.attributeName!]) {
        if (
          searcheRef.operatorCode &&
          oneParamConds.includes(searcheRef.operatorCode)
        ) {
          const fn: TFilterNode = {
            name: searcheRef.attributeName!,
            operatorCode: searcheRef.operatorCode,
            filterParams: [searchData[searcheRef.attributeName!]],
          };
          fns.push(fn);
        }
      }
    });
  }
  const searchParam: TPageInfoInput = {
    pageIndex: 1,
    pageSize: 10,
    logicNode: {
      logicOperatorCode: ELogicOperatorCode.AND,
      filterNodes: fns,
    },
    orders: [
      {
        property: '{{ ht.mainProperty }}',
        direction: EDirection.ASC,
        ignoreCase: false,
      },
    ],
  };
  const pageInfo: TPageInfo<T{{ ht.tabClassName }}> = await ListAPI.pageList(searchParam);
  return pageInfo;
};

const pageChange = async (store: Ref<TTableStore>, params: { page: number; pageSize: number }) => {
  const { page, pageSize } = params;
  const fns: TFilterNode[] = [];
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.billFormFields is iterable %}
      {%- for b in bt.billFormFields %}
        {%- if b.fgTreeAttr and b.refConfig %}
        if(store.value.selectedTreeNode){
          const treeIdFn: TFilterNode = {
            name: '{{ b.name }}',
            operatorCode: EOperatorCode.EQUAL,
            filterParams: [store.value.selectedTreeNode['{{ b.refConfig.backWriteProp }}']],
          };
          fns.push(treeIdFn);
        }
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  const searchData = store.value.searchData;
  if (searcheRefs && searchData) {
    searcheRefs.forEach((searcheRef) => {
      if (searchData[searcheRef.attributeName!]) {
        if (
          searcheRef.operatorCode &&
          oneParamConds.includes(searcheRef.operatorCode)
        ) {
          const fn: TFilterNode = {
            name: searcheRef.attributeName!,
            operatorCode: searcheRef.operatorCode,
            filterParams: [searchData[searcheRef.attributeName!]],
          };
          fns.push(fn);
        }
      }
    });
  }
  const queyrParams: TPageInfoInput = {
    pageIndex: page,
    pageSize: pageSize,
    logicNode: {
      logicOperatorCode: ELogicOperatorCode.AND,
      filterNodes: fns,
    },
    orders: [
      {
        property: '{{ ht.mainProperty }}',
        direction: EDirection.ASC,
        ignoreCase: false,
      },
    ],
  };
  const pageInfo: TPageInfo<T{{ ht.tabClassName }}> = await ListAPI.pageList(queyrParams);
  return pageInfo;
}

const batchRemove = async (store: Ref<TTableStore>, message: TMessage) => {
  if (!message || message.consumerIds.includes(componentName)) {
    return new Promise<TPageInfo<T{{ ht.tabClassName }}>>((resolve, reject) => {
      reject('没有参数，或消息已被消费')
    });
  }
  const deleteDatas = store.value.tableData?.filter((d) =>
    store.value.selectedRowKeys?.includes(d.{{ ht.mainProperty }}!),
  );
  if (!deleteDatas || deleteDatas.length === 0) {
    return new Promise<TPageInfo<T{{ ht.tabClassName }}>>((resolve, reject) => {
      reject('没有选中待删除数据')
    });
  }
  await ListAPI.batchRemove(deleteDatas);
  const fns: TFilterNode[] = [];
  const params: TPageInfoInput = {
    pageIndex: 1,
    pageSize: 10,
    logicNode: {
      logicOperatorCode: ELogicOperatorCode.AND,
      filterNodes: fns,
    },
    orders: [
      {
        property: '{{ ht.mainProperty }}',
        direction: EDirection.ASC,
        ignoreCase: false,
      },
    ],
  };
  const pageInfo: TPageInfo<T{{ ht.tabClassName }}> = await ListAPI.pageList(params);
  return pageInfo;
};

  {%- endfor %}
{%- endif %}