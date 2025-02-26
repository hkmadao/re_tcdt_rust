import { CaseReducer, PayloadAction, nanoid, } from "@reduxjs/toolkit";
import { TFormStore } from "../models";
import { TTree, DOStatus, } from "@/models";
import { Key } from "react";
import {
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from "../../../../models";
import { subject } from "../../../../conf";
import { deepCopy } from "@/util";

{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
export const setComponentInfo: CaseReducer<
  TFormStore,
  PayloadAction<{ idUiConf: string; fgDisabled: boolean; fgHidden: boolean; }>
> = (state, action) => {
  const { idUiConf, fgDisabled, fgHidden } = action.payload;
  state.idUiConf = idUiConf;
  state.fgDisabled = fgDisabled;
  state.fgHidden = fgHidden;
};

export const setFormData: CaseReducer<
  TFormStore,
  PayloadAction<T{{ bt.tabClassName }}>
> = (state, action) => {
  state.formData = { ...action.payload };
};

export const addFormData: CaseReducer<
  TFormStore,
  PayloadAction<{ nodeData: any; }>
> = (state, action) => {
  const { nodeData, } = action.payload;
  state.treeSelectedNode = nodeData;
  state.formData = {
    {{ bt.mainProperty }}: nanoid(),
    action: DOStatus.NEW,
{%- for ht in rootInfo.bJson.configForm.header %}
  {%- if ht.billFormFields is iterable %}
    {%- for b in ht.billFormFields %}
      {%- if b.defaultValue %}
    {{ b.name }}: {{ b.defaultValue }},
      {%- endif %}
    {%- endfor %}
  {%- endif %}
{%- endfor %}
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType == "Single" %}
    {{ bt.tabCode }}: undefined,
      {%- endif %}
      {%- if bt.refType and bt.refType == "Array" %}
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
    state.formData.{{ b.name }} = nodeData.{{ b.name }};
    state.formData.{{ b.refAttributeName }} = deepCopy(nodeData);
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  }
  state.newDataArr = [];
  state.editData = undefined;
  state.editStatusInfo = {
    id: nanoid(),
    editStatus: 'toAdd',
  };
};

export const cancel: CaseReducer<
  TFormStore,
  PayloadAction<void>
> = (state, action) => {
  if (state.editData) {
    subject.publish({
      topic: 'updateSuccess',
      producerId: state.idUiConf!,
      data: deepCopy(state.editData),
    });
  }
  if (state.newDataArr) {
    subject.publish({
      topic: 'addSuccess',
      producerId: state.idUiConf!,
      data: deepCopy(state.newDataArr),
    });
  }
  subject.publish({
    topic: '/page/change',
    producerId: state.idUiConf!,
    data: 'list',
  });
};

export const updateFormData: CaseReducer<
  TFormStore,
  PayloadAction<T{{ bt.tabClassName }}>
> = (state, action) => {
  state.formData = {
    ...state.formData,
    ...action.payload,
    action: state.formData.action !== DOStatus.NEW ? DOStatus.UPDATED : state.formData.action,
    {{ bt.mainProperty }}: state.formData.{{ bt.mainProperty }},
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType == "Single" %}
    {{ bt.tabCode }}: state.formData.{{ bt.tabCode }},
      {%- endif %}
      {%- if bt.refType and bt.refType == "Array" %}
    {{ bt.tabCode }}: state.formData.{{ bt.tabCode }},
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  };
};
  {%- endfor %}
{%- endif %}
