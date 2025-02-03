import { FC, useEffect, useRef, useState } from 'react';
import {
  Form,
  Input,
  InputNumber,
  Checkbox,
  Space,
  Select,
} from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { PlusOutlined } from '@ant-design/icons';
import moment from 'moment';
import { EPartName, TTree } from '@/models';
import { Observer, TMessage } from '@/util/observer';
import RefPicker from '@/components/Ref';
import CustomDatePick from '@/components/CustomDatePick';
import CustomTimePicker from '@/components/CustomTimePicker';
import {
{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';
import { getRefByAttr } from '@/util';
import { billformConf, subject } from '../../../../conf';
import {
  actions,
  toEdit,
  save,
  reflesh,
} from '../store';
import { useEditStatusInfo, useFormData, useIdUiConf, useFgDisabled, } from '../hooks';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.header is iterable %}
  {%- for bt in rootInfo.bJson.configForm.header %}
const MainFormLayout: FC = () => {
  const [form] = Form.useForm();
  const dispatch = useDispatch();
  const idUiConf = useIdUiConf();
  const fgDisabled = useFgDisabled();
  const moduleData = useFormData();
  const editStatus = useEditStatusInfo();

  useEffect(() => {
    if (editStatus) {
      form.resetFields();
      form.setFieldsValue(moduleData);
    }
  }, [editStatus]);

  useEffect(() => {
    if (!idUiConf) {
      return;
    }

    const cancleObserver: Observer = {
      topic: 'cancel',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idUiConf)) {
          return;
        }
        dispatch(actions.cancel());
      },
    };
    subject.subscribe(cancleObserver);

    const toAddObserver: Observer = {
      topic: 'toAdd',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        if (message.consumerIds.includes(idUiConf)) {
          return;
        }
        dispatch(actions.addFormData({ nodeData: message.data.treeSelectedNode }));
      },
    };
    subject.subscribe(toAddObserver);

    const addObserver: Observer = {
      topic: 'add',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (message.consumerIds.includes(idUiConf)) {
            return;
          }
          const data = await form.validateFields();
          dispatch(save({ actionType: 'add' }));
        })();
      },
    };
    subject.subscribe(addObserver);

    const addAgainObserver: Observer = {
      topic: 'addAgain',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (message.consumerIds.includes(idUiConf)) {
            return;
          }
          const data = await form.validateFields();
          dispatch(save({ actionType: 'addAgain' }));
        })();
      },
    };
    subject.subscribe(addAgainObserver);

    const toEditObserver: Observer = {
      topic: 'toEdit',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(toEdit({ nodeData: message.data.treeSelectedNode, selectedRow: message.data.selectedRow }));
        })();
      },
    };
    subject.subscribe(toEditObserver);

    const editObserver: Observer = {
      topic: 'edit',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (message.consumerIds.includes(idUiConf)) {
            return;
          }
          const data = await form.validateFields();
          dispatch(save({ actionType: 'edit' }));
        })();
      },
    };
    subject.subscribe(editObserver);

    const detailRefleshObserver: Observer = {
      topic: 'detailReflesh',
      consumerId: idUiConf,
      update: function (message: TMessage): void {
        (async () => {
          if (message.consumerIds.includes(idUiConf)) {
            return;
          }
          dispatch(reflesh());
        })();
      },
    };
    subject.subscribe(detailRefleshObserver);

    //销毁观察者
    return () => {
      subject.unsubsribe(cancleObserver);
      subject.unsubsribe(toAddObserver);
      subject.unsubsribe(addObserver);
      subject.unsubsribe(addAgainObserver);
      subject.unsubsribe(toEditObserver);
      subject.unsubsribe(editObserver);
      subject.unsubsribe(detailRefleshObserver);
    };
  }, [idUiConf]);

  const handleValuesChange = (changedValues: any, values: T{{ bt.tabClassName }}) => {
    const newValues = { ...values };
{%- for ht in rootInfo.bJson.configForm.header %}
  {%- if ht.billFormFields is iterable %}
    {%- for b in ht.billFormFields %}
      {%- if b.inputType and b.inputType == "Ref" %}
    if (!newValues.{{ b.refAttributeName }}) {
      newValues.{{ b.name }} = undefined;
    } else {
      newValues.{{ b.name }} = newValues.{{ b.refAttributeName }}.{{ b.refConfig.backWriteProp }};
    }
      {%- endif %}
    {%- endfor %}
  {%- endif %}
{%- endfor %}
    dispatch(actions.updateFormData(newValues));
  }

  return (
    <>
{%- for ht in rootInfo.bJson.configForm.header %}
  {%- if ht.billFormFields is iterable %}
      <Form form={form} layout={'inline'} onValuesChange={handleValuesChange}>
        <Space direction="horizontal" size={2} wrap={true}>
    {%- for b in ht.billFormFields %}
      {%- if not b.inputType %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
            {%- if b.fgDisplay %}{%- else %}hidden{%- endif %}
          >
            <Input
              readOnly={fgDisabled {%- if b.readonly %} || true {%- else %}{%- endif %}}
              allowClear
              placeholder={
                '请输入{{ b.displayName }}'
              }
            />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Input" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
            {%- if b.fgDisplay %}{%- else %}hidden{%- endif %}
          >
            <Input
              readOnly={fgDisabled {%- if b.readonly %} || true {%- else %}{%- endif %} }
              allowClear
              placeholder={
                '请输入{{ b.displayName }}'
              }
            />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "InputNumber" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <InputNumber 
              readOnly={fgDisabled {%- if b.readonly %} || true {%- else %}{%- endif %} }
              placeholder={
                '请输入{{ b.displayName }}'
              } 
            />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Text" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <InputText 
              readOnly={fgDisabled {%- if b.readonly %} || true {%- else %}{%- endif %} }
              allowClear
              placeholder={
                '请输入{{ b.displayName }}'
              } 
            />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Checkbox" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
            valuePropName="checked"
          >
            <Checkbox disabled={fgDisabled {%- if b.readonly %} || true {%- else %}{%- endif %} }/>
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "DateTime" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <CustomDatePick 
              format='YYYY-MM-DDTHH:mm:ssZ'
              displayFormat='YYYY-MM-DD HH:mm:ss'
            />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Date" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <CustomDatePick displayFormat='YYYY-MM-DDHH:mm:ss' format="YYYY-MM-DD" />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Time" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <CustomTimePicker format="HH:mm:ss" />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Ref" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.refAttributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <RefPicker
              {...getRefByAttr(
                EPartName.Header,
                '{{ ht.tabCode }}',
                '{{ b.name }}',
                billformConf!,
              )!}
            />
          </Form.Item>
      {%- endif %}
      {%- if b.inputType and b.inputType == "Select" %}
          <Form.Item
            label={'{{ b.displayName }}'}
            name={'{{ b.name }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <Select placeholder={'请选择'} disabled={fgDisabled {%- if b.readonly %} || true {%- else %}{%- endif %} }>
        {%- for enumColumn in b.enumConfig.enumColumns %}
              <Select.Option value={'{{ enumColumn.enumValue }}'}>{{ enumColumn.displayName }}</Select.Option>
        {%- endfor %}
            </Select>
          </Form.Item>
      {%- endif %}
    {%- endfor %}
        </Space>
      </Form>
  {%- endif %}
{%- endfor %}
    </>
  );
};

export default MainFormLayout;
  {%- endfor %}
{%- endif %}
