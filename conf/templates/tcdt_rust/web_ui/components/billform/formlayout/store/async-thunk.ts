import { createAsyncThunk } from '@reduxjs/toolkit';
import FormAPI from '../api';
import {
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';
import { componentName } from '../conf';
import { TFormStore } from '../models';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
export const toEdit = createAsyncThunk(
  `/toEdit`,
  async (params: { nodeData: any; selectedRow: T{{ bt.tabClassName }}; }, thunkAPI) => {
    const { nodeData, selectedRow } = params;
    const detailData: T{{ bt.tabClassName }} = await FormAPI.getById(selectedRow.{{ bt.mainProperty }}!);
    return { nodeData, detailData };
  },
);

export const reflesh = createAsyncThunk(
  `/reflesh`,
  async (param: void, thunkAPI) => {
    const state: TFormStore = (thunkAPI.getState() as any)[componentName];
    const loadData: T{{ bt.tabClassName }} = await FormAPI.getById(state.selectedRow?.{{ bt.mainProperty }});
    return loadData;
  },
);

export const save = createAsyncThunk(
  `/save`,
  async (params: { actionType: 'add' | 'addAgain' | 'edit' }, thunkAPI) => {
    const { actionType } = params;
    const state: TFormStore = (thunkAPI.getState() as any)[componentName];
    if (actionType === 'add' || actionType === 'addAgain') {
      const saveData: T{{ bt.tabClassName }} = await FormAPI.add(state.formData);
      return {
        actionType,
        saveData,
      };
    }
    const saveData: T{{ bt.tabClassName }} = await FormAPI.update(state.formData);
    return {
      actionType,
      saveData,
    };
  },
);
  {%- endfor %}
{%- endif %}
