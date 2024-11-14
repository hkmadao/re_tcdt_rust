import { DOStatus } from '@/models';
import { CaseReducer, nanoid, PayloadAction } from '@reduxjs/toolkit';
import { 
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
    T{{ bt.tabClassName }},
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from '../../../../models';
import { TFormStore } from '../models';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType == "Array" %}
/*=========={{ bt.tabClassName }}=============*/
export const addFormData{{ bt.tabClassName }}: CaseReducer<
  TFormStore,
  PayloadAction<Pick<T{{ bt.tabClassName }}, '{{ bt.mainProperty }}'>>
> = (state, action) => {
  const {{ bt.firstLowerTabClassName }}New: T{{ bt.tabClassName }} = {
    ...action.payload,
    {{ bt.mainProperty }}: nanoid(),
  };
  {{ bt.firstLowerTabClassName }}New.action = DOStatus.NEW;
  if (!state.formData.{{ bt.tabCode }}) {
    state.formData.{{ bt.tabCode }} = [];
  }
  state.formData.{{ bt.tabCode }}.push({{ bt.firstLowerTabClassName }}New);
  if (state.formData.action !== DOStatus.NEW) {
    state.formData.action = DOStatus.UPDATED;
  }
};

export const updateFormData{{ bt.tabClassName }}: CaseReducer<
  TFormStore,
  PayloadAction<T{{ bt.tabClassName }}>
> = (state, action) => {
  const {{ bt.firstLowerTabClassName }}News = state.formData.{{ bt.tabCode }}?.map((t) => {
    if (
      t.{{ bt.mainProperty }} === action.payload.{{ bt.mainProperty }}
    ) {
      t = { ...t, ...action.payload };
      if (t.action !== DOStatus.NEW) {
        t.action = DOStatus.UPDATED;
      }
    }
    return t;
  });
  if (state) {
    if (state.formData.action !== DOStatus.NEW) {
      state.formData.action = DOStatus.UPDATED;
    }
    state.formData.{{ bt.tabCode }} = {{ bt.firstLowerTabClassName }}News;
  }
};

export const deleteFormData{{ bt.tabClassName }}: CaseReducer<
  TFormStore,
  PayloadAction<T{{ bt.tabClassName }}>
> = (state, action) => {
  let {{ bt.firstLowerTabClassName }}News = state.formData.{{ bt.tabCode }}?.slice();
  {{ bt.firstLowerTabClassName }}News = {{ bt.firstLowerTabClassName }}News?.filter((t) => {
    if (
      t.{{ bt.mainProperty }} === action.payload.{{ bt.mainProperty }}
    ) {
      if (t.action === DOStatus.NEW) {
        return false;
      }
      t.action = DOStatus.DELETED;
    }
    return true;
  });
  if (state) {
    state.formData.{{ bt.tabCode }} = {{ bt.firstLowerTabClassName }}News;
    if (state.formData.action !== DOStatus.NEW) {
      state.formData.action = DOStatus.UPDATED;
    }
  }
};
/*=========={{ bt.tabClassName }}=============*/
        {%- endif %}
        {%- if bt.refType and bt.refType == "Single" %}
/*=========={{ bt.tabClassName }}=============*/
export const addFormData{{ bt.tabClassName }}: CaseReducer<
  TFormStore,
  PayloadAction<Pick<T{{ bt.tabClassName }}, '{{ bt.mainProperty }}'>>
> = (state, action) => {
  const {{ bt.firstLowerTabClassName }}: T{{ bt.tabClassName }} = {
    ...action.payload,
    {{ bt.mainProperty }}: nanoid(),
  };
  {{ bt.firstLowerTabClassName }}.action = DOStatus.NEW;
  if (state) {
    state.formData.{{ bt.tabCode }} = {{ bt.firstLowerTabClassName }};
    if (state.formData.action !== DOStatus.NEW) {
      state.formData.action = DOStatus.UPDATED;
    }
  }
};

export const updateFormData{{ bt.tabClassName }}: CaseReducer<
  TFormStore,
  PayloadAction<T{{ bt.tabClassName }}>
> = (state, action) => {
  let {{ bt.firstLowerTabClassName }} = action.payload;
  if ({{ bt.firstLowerTabClassName }}.action !== DOStatus.NEW) {
    {{ bt.firstLowerTabClassName }}.action = DOStatus.UPDATED;
  }
  if (state) {
    state.formData.{{ bt.tabCode }} = { ...state.formData.{{ bt.tabCode }}, ...{{ bt.firstLowerTabClassName }} };
    if (state.formData.action !== DOStatus.NEW) {
      state.formData.action = DOStatus.UPDATED;
    }
  }
};

export const deleteFormData{{ bt.tabClassName }}: CaseReducer<
  TFormStore,
  PayloadAction<T{{ bt.tabClassName }}>
> = (state, action) => {
  const delete{{ bt.tabClassName }} = action.payload;
  let {{ bt.firstLowerTabClassName }} = state.formData.{{ bt.tabCode }};
  if (!{{ bt.firstLowerTabClassName }}) {
    return;
  }
  if ({{ bt.firstLowerTabClassName }}?.{{ bt.mainProperty }} === delete{{ bt.tabClassName }}.{{ bt.mainProperty }}) {
    if (state) {
      if ({{ bt.firstLowerTabClassName }}?.action === DOStatus.NEW) {
        state.formData.{{ bt.tabCode }} = undefined;
        return;
      }
      {{ bt.firstLowerTabClassName }}.action = DOStatus.DELETED;
      state.formData.{{ bt.tabCode }} = {{ bt.firstLowerTabClassName }};
      if (state.formData.action !== DOStatus.NEW) {
        state.formData.action = DOStatus.UPDATED;
      }
    }
  }
};
/*=========={{ bt.tabClassName }}=============*/
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
