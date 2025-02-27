import {
  configureStore,
  createSlice,
  CaseReducer,
  PayloadAction,
  nanoid,
} from '@reduxjs/toolkit';
import {
  message as antdMessage,
} from 'antd';
import { DOStatus } from '@/models';
import { reflesh, toEdit, save } from './async-thunk';
import { componentName } from '../conf';
import { initialState } from './initial-state';
import * as reducers from './actions';
import * as itemReducers from './items';
import { subject } from '../../../../conf';
import { deepCopy } from '@/util';

export * from './async-thunk';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
export const formSlice = createSlice({
  name: componentName,
  initialState: initialState,
  reducers: {
    ...reducers,
    ...itemReducers,
  },
  extraReducers: (builder) => {
    builder
      .addCase(toEdit.pending, (state, action) => {
        subject.publish({
          topic: '/page/addLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(toEdit.rejected, (state, action) => { 
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
      })
      .addCase(toEdit.fulfilled, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
        const { nodeData, detailData } = action.payload;
        state.treeSelectedNode = nodeData;
        state.selectedRow = detailData;
        state.formData = detailData;
        state.newDataArr = [];
        state.editData = undefined;
        state.editStatusInfo = {
          id: nanoid(),
          editStatus: 'toEdit',
        };
      })
      .addCase(reflesh.pending, (state, action) => {
        subject.publish({
          topic: '/page/addLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(reflesh.rejected, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(reflesh.fulfilled, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
        state.formData = action.payload;
        state.editStatusInfo = {
          id: nanoid(),
          editStatus: 'reflesh',
        };
      })
      .addCase(save.pending, (state, action) => {
        subject.publish({
          topic: '/page/addLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
       })
      .addCase(save.rejected, (state, action) => { 
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
      })
      .addCase(save.fulfilled, (state, action) => {
        subject.publish({
          topic: '/page/reduceLoadingCount',
          producerId: state.idUiConf!,
          data: undefined,
        });
        const { actionType, saveData } = action.payload;
        if (actionType === 'add') {
          state.newDataArr.push(saveData);
          state.formData = saveData;

          antdMessage.info('新增成功！');
          subject.publish({
            topic: 'addSuccess',
            producerId: state.idUiConf!,
            data: deepCopy(state.newDataArr),
          });
          state.editStatusInfo = {
            id: nanoid(),
            editStatus: 'added',
          };
          subject.publish({
            topic: '/page/change',
            producerId: state.idUiConf!,
            data: 'list',
          });
        }
        if (actionType === 'addAgain') {
          state.newDataArr.push(saveData);
          state.formData = {
            {{ bt.mainProperty }}: nanoid(),
            action: DOStatus.NEW,
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
          if(state.treeSelectedNode){
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
    {%- if bt.billFormFields is iterable %}
      {%- for b in bt.billFormFields %}
        {%- if b.fgTreeAttr and b.refConfig %}
            state.formData.{{ b.name }} = state.treeSelectedNode.{{ b.name }};
            state.formData.{{ b.refAttributeName }} = deepCopy(state.treeSelectedNode);
        {%- endif %}
      {%- endfor %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
          }

          antdMessage.info('新增成功！');
          state.editStatusInfo = {
            id: nanoid(),
            editStatus: 'toAdd',
          };
        }
        if (actionType === 'edit') {
          state.editData = saveData;
          state.formData = saveData;

          antdMessage.info('更新成功！');
          subject.publish({
            topic: 'updateSuccess',
            producerId: state.idUiConf!,
            data: deepCopy(saveData),
          });
          state.editStatusInfo = {
            id: nanoid(),
            editStatus: 'edited',
          };
          subject.publish({
            topic: '/page/change',
            producerId: state.idUiConf!,
            data: 'list',
          });
        }
      });
  },
});

export const actions = formSlice.actions;
  {%- endfor %}
{%- endif %}

const reducer:any = {};
reducer[formSlice.name] = formSlice.reducer;

export default configureStore({
  reducer: {
    ...reducer,
  },
});
