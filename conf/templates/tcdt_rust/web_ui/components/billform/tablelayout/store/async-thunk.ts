import { createAsyncThunk } from '@reduxjs/toolkit';
import { TMessage } from '@/util';
import { componentName } from '../conf';
import { andLogicNode, buildFilterValueBySearchRef, equalFilterNode, EDirection, TFilterNode, TPageInfo, TPageInfoInput, TTree, stringFilterParam } from '@/models';
import { TTableStore } from '../models';
import { queryConf, tableConf, } from '../../../../conf';
import ListAPI from '../api';
import {
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
 } from '../../../../models';

{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
const searcheRefs = queryConf?.searchRefs;

export const fetchByTreeNode = createAsyncThunk(
  `${tableConf?.name}/fetchByTreeNode`,
  async (message: TMessage, thunkAPI) => {
    if (!message) {
      return;
    }
    const selectedTreeNode: TTree = message.data as TTree;
    const fns: TFilterNode[] = [];
    {%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
      {%- for bt in rootInfo.bTableJson.configList.header %}
        {%- if bt.billFormFields is iterable %}
          {%- for b in bt.billFormFields %}
            {%- if b.fgTreeAttr and b.refConfig %}
    if (selectedTreeNode) {
      const treeIdFn: TFilterNode = equalFilterNode('{{ b.name }}', stringFilterParam(selectedTreeNode['{{ b.refConfig.backWriteProp }}']));
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
      logicNode: andLogicNode(fns)(),
      orders: [
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.orderProperty %}
        {
          property: '{{ bt.orderProperty }}',
          direction: {%- if bt.orderType == 'ASC' %} EDirection.ASC{% else %} EDirection.DESC{%- endif %},
          ignoreCase: false,
        },
    {%- endif %}
  {%- endfor %}
{%- endif %}
      ],
    };
    const pageInfo: TPageInfo<T{{ bt.tabClassName }}> = await ListAPI.pageList(
      params,
    );
return {
  selectedTreeNode,
  pageInfo,
}
  },
);

export const search = createAsyncThunk(
  `${tableConf?.name}/search`,
  async (message: TMessage, thunkAPI) => {
    const state: TTableStore = (thunkAPI.getState() as any)[componentName];
    if (!message || message.consumerIds.includes(componentName)) {
      return;
    }
    const searchData = message.data;
    const fns: TFilterNode[] = [];
    if (searchData) {
      searcheRefs?.forEach((searcheRef) => {
        if (
          !(
            searcheRef.operatorCode === 'isNull' ||
            searcheRef.operatorCode === 'notNull'
          ) &&
          (searchData[searcheRef.attributeName!] === undefined ||
            searchData[searcheRef.attributeName!] === null)
        ) {
          return;
        }
        if (searcheRef.operatorCode) {
          let value = searchData[searcheRef.attributeName!];
          if (searcheRef.valueType === 'Bool') {
            if (!(value === true || value === false || value === 'true' || value === 'false')) {
              return;
            }
            value = value && value === 'true'
          } else if (!value && value !== 0) {
            return;
          }
          const fn: TFilterNode = {
            name: searcheRef.attributeName!,
            operatorCode: searcheRef.operatorCode,
            filterParams: buildFilterValueBySearchRef(searcheRef, value),
          };
          fns.push(fn);
        }
      });
    }
    const params: TPageInfoInput = {
      pageIndex: 1,
      pageSize: 10,
      logicNode: andLogicNode(fns)(),
      orders: [
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.orderProperty %}
        {
          property: '{{ bt.orderProperty }}',
          direction: {%- if bt.orderType == 'ASC' %} EDirection.ASC{% else %} EDirection.DESC{%- endif %},
          ignoreCase: false,
        },
    {%- endif %}
  {%- endfor %}
{%- endif %}
      ],
    };
    const pageInfo: TPageInfo<T{{ bt.tabClassName }}> = await ListAPI.pageList(
      params,
    );
return {
  searchData,
  pageInfo,
}
  },
);

export const reflesh = createAsyncThunk(
  `${tableConf?.name}/reflesh`,
  async (params: void, thunkAPI) => {
    const state: TTableStore = (thunkAPI.getState() as any)[componentName];
    const searchData = state.searchData;
    const fns: TFilterNode[] = [];
    {%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
      {%- for bt in rootInfo.bTableJson.configList.header %}
        {%- if bt.billFormFields is iterable %}
          {%- for b in bt.billFormFields %}
            {%- if b.fgTreeAttr and b.refConfig %}
    if (state.selectedTreeNode) {
      const treeIdFn: TFilterNode = equalFilterNode('{{ b.name }}', stringFilterParam(state.selectedTreeNode['{{ b.refConfig.backWriteProp }}']));
      fns.push(treeIdFn);
    }
            {%- endif %}
          {%- endfor %}
        {%- endif %}
      {%- endfor %}
    {%- endif %}
    if (searcheRefs && searchData) {
      searcheRefs.forEach((searcheRef) => {
        if (
          !(
            searcheRef.operatorCode === 'isNull' ||
            searcheRef.operatorCode === 'notNull'
          ) &&
          (searchData[searcheRef.attributeName!] === undefined ||
            searchData[searcheRef.attributeName!] === null)
        ) {
          return;
        }
        if (searcheRef.operatorCode) {
          let value = searchData[searcheRef.attributeName!];
          if (searcheRef.valueType === 'Bool') {
            if (!(value === true || value === false || value === 'true' || value === 'false')) {
              return;
            }
            value = value && value === 'true'
          } else if (!value && value !== 0) {
            return;
          }
          const fn: TFilterNode = {
            name: searcheRef.attributeName!,
            operatorCode: searcheRef.operatorCode,
            filterParams: buildFilterValueBySearchRef(searcheRef, value),
          };
          fns.push(fn);
        }
      });
    }
    const searchParam: TPageInfoInput = {
      pageIndex: 1,
      pageSize: 10,
      logicNode: andLogicNode(fns)(),
      orders: [
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.orderProperty %}
        {
          property: '{{ bt.orderProperty }}',
          direction: {%- if bt.orderType == 'ASC' %} EDirection.ASC{% else %} EDirection.DESC{%- endif %},
          ignoreCase: false,
        },
    {%- endif %}
  {%- endfor %}
{%- endif %}
      ],
    };
    const pageInfo: TPageInfo<T{{ bt.tabClassName }}> = await ListAPI.pageList(
      searchParam,
    );
return pageInfo
  },
);

export const pageChange = createAsyncThunk(
  `${tableConf?.name}/pageChange`,
  async (params: { page: number, pageSize: number }, thunkAPI) => {
    const { page, pageSize } = params;
    const state: TTableStore = (thunkAPI.getState() as any)[componentName];
    const fns: TFilterNode[] = [];
    {%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
      {%- for bt in rootInfo.bTableJson.configList.header %}
        {%- if bt.billFormFields is iterable %}
          {%- for b in bt.billFormFields %}
            {%- if b.fgTreeAttr and b.refConfig %}
    if (state.selectedTreeNode) {
      const treeIdFn: TFilterNode = equalFilterNode('{{ b.name }}', stringFilterParam(state.selectedTreeNode['{{ b.refConfig.backWriteProp }}']));
      fns.push(treeIdFn);
    }
            {%- endif %}
          {%- endfor %}
        {%- endif %}
      {%- endfor %}
    {%- endif %}
    const searchData = state.searchData;
    if (searcheRefs && searchData) {
      searcheRefs.forEach((searcheRef) => {
        if (searchData[searcheRef.attributeName!]) {
          if (
            !(
              searcheRef.operatorCode === 'isNull' ||
              searcheRef.operatorCode === 'notNull'
            ) &&
            (searchData[searcheRef.attributeName!] === undefined ||
              searchData[searcheRef.attributeName!] === null)
          ) {
            return;
          }
          let value = searchData[searcheRef.attributeName!];
          if (searcheRef.valueType === 'Bool') {
            if (!(value === true || value === false || value === 'true' || value === 'false')) {
              return;
            }
            value = value && value === 'true'
          } else if (!value && value !== 0) {
            return;
          }
          const fn: TFilterNode = {
            name: searcheRef.attributeName!,
            operatorCode: searcheRef.operatorCode,
            filterParams: buildFilterValueBySearchRef(searcheRef, value),
          };
          fns.push(fn);
        }
      });
    }
    const queyrParams: TPageInfoInput = {
      pageIndex: page,
      pageSize: pageSize,
      logicNode: andLogicNode(fns)(),
      orders: [
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.orderProperty %}
        {
          property: '{{ bt.orderProperty }}',
          direction: {%- if bt.orderType == 'ASC' %} EDirection.ASC{% else %} EDirection.DESC{%- endif %},
          ignoreCase: false,
        },
    {%- endif %}
  {%- endfor %}
{%- endif %}
      ],
    };
    const pageInfo: TPageInfo<T{{ bt.tabClassName }}> = await ListAPI.pageList(
      queyrParams,
    );
return pageInfo
  },
);

export const batchRemove = createAsyncThunk(
  `${tableConf?.name}/batchRemove`,
  async (message: TMessage, thunkAPI) => {
    if (!message || message.consumerIds.includes(componentName)) {
      return;
    }
    const state: TTableStore = (thunkAPI.getState() as any)[componentName];
    const deleteDatas = state.tableData?.filter(d => state.selectedRowKeys?.includes(d.{{ bt.mainProperty }}!));
if (!deleteDatas || deleteDatas.length === 0) {
  return;
}
await ListAPI.batchRemove(deleteDatas);
const fns: TFilterNode[] = [];
const params: TPageInfoInput = {
  pageIndex: 1,
  pageSize: 10,
  logicNode: andLogicNode(fns)(),
  orders: [
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
    {%- if bt.orderProperty %}
        {
          property: '{{ bt.orderProperty }}',
          direction: {%- if bt.orderType == 'ASC' %} EDirection.ASC{% else %} EDirection.DESC{%- endif %},
          ignoreCase: false,
        },
    {%- endif %}
  {%- endfor %}
{%- endif %}
  ],
};
const pageInfo: TPageInfo<T{{ bt.tabClassName }}> = await ListAPI.pageList(
  params,
);
return pageInfo;
  },
);
  {%- endfor %}
{%- endif %}
