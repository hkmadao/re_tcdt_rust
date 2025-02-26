import { FC, ReactNode, useEffect, useRef, useState } from 'react';
import { Button, Checkbox, Form, Input, InputNumber, Select } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import RefPicker from '@/components/Ref';
import { Observer, TMessage } from '@/util/observer';
import CustomDatePick from '@/components/CustomDatePick';
import { subject, queryConf } from '../../../conf';
import { useFgDisabled, useFgHidden, useIdUiConf } from '../hooks';

const SearchAreaComponent: FC<{}> = ({ }) => {
  const idUiConf = useIdUiConf();
  const fgDisabled = useFgDisabled();
  const fgHidden = useFgHidden();
  const searcheRefs = queryConf?.searchRefs;
  const [form] = Form.useForm();
  const dispatch = useDispatch();
  const searchValuesRef = useRef<any>({});

  useEffect(() => {
    if (!idUiConf) {
      return;
    }
    
    const treeNodeObserver: Observer = {
      topic: 'treeNodeSelected',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf!)) {
            return;
          }
          // form.resetFields();
        })();
      },
    };
    subject.subscribe(treeNodeObserver);

    const treeNodeCancelObserver: Observer = {
      topic: 'treeSelectCancel',
      consumerId: idUiConf!,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idUiConf!)) {
            return;
          }
        })();
      },
    };
    subject.subscribe(treeNodeCancelObserver);

    //销毁观察者
    return () => {
      subject.unsubsribe(treeNodeObserver);
      subject.unsubsribe(treeNodeCancelObserver);
    };
  }, [idUiConf]);

  useEffect(() => {
    const newValues:any = {};
{%- if rootInfo.qJson and rootInfo.qJson.searchRefs is iterable %}
  {%- for b in rootInfo.qJson.searchRefs %}
    {%- if b.defaultValue == false or b.defaultValue %}
      {%- if b.htmlInputType and b.htmlInputType  == "Input" %}
    newValues.{{ b.attributeName }} = '{{ b.defaultValue }}';
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "InputNumber" %}
    newValues.{{ b.attributeName }} = {{ b.defaultValue }};
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Text" %}
    newValues.{{ b.attributeName }} = '{{ b.defaultValue }}';
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Checkbox" %}
    newValues.{{ b.attributeName }} = {{ b.defaultValue }};
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "DateTime" %}
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Date" %}
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Time" %}
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Ref" %}
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Select" %}
    newValues.{{ b.attributeName }} = '{{ b.defaultValue }}';
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
    form.setFieldsValue(newValues);
    searchValuesRef.current = newValues;
  }, [searcheRefs]);

  const handleValuesChange = (changedValues: any, values: any) => {
    const newValues = { ...values };
{%- if rootInfo.qJson and rootInfo.qJson.searchRefs is iterable %}
  {%- for b in rootInfo.qJson.searchRefs %}
    {%- if b.inputType and b.htmlInputType == "Ref" %}
    if (!values.{{ b.refAttributeName }}) {
      newValues.{{ b.attributeName }} = undefined;
    }
    if (changedValues.{{ b.refAttributeName }}) {
      newValues.{{ b.attributeName }} = changedValues.{{ b.refAttributeName }}.{{ b.refConfig.backWriteProp }};
    }
    {%- endif %}
  {%- endfor %}
{%- endif %}
    searchValuesRef.current = newValues;
  }

  const handleSearch = async () => {
    let searchValues: any = {};
    searcheRefs?.forEach((searchRef) => {
      let attributeName = searchRef.attributeName;
      if (!attributeName) {
        console.warn('searchRef attributeName is undefind');
        return;
      }
      if (searchRef.htmlInputType === 'Ref') {
        let refAttributeName = searchRef.refAttributeName;
        if (!refAttributeName) {
          console.warn('searchRef refAttributeName is undefind');
          return;
        }
        if (!searchValuesRef.current[refAttributeName]) {
          return;
        }
        let backWriteProp = searchRef.refConfig?.backWriteProp;
        if (!backWriteProp) {
          console.warn('searchRef refConfig backWriteProp is undefind');
          return;
        }
        searchValues[attributeName] =
          searchValuesRef.current[refAttributeName][backWriteProp];
        return;
      }
      searchValues[attributeName] = searchValuesRef.current[attributeName];
    });
    subject.publish({
      topic: 'search',
      producerId: idUiConf!,
      data: searchValues,
    });
  };

  return (
    <>
      <div
{%- raw %}
        style={{
          display: fgHidden ? 'none' : 'block',
        }}
{%- endraw %}
      >
        <Form form={form} layout={'inline'} onValuesChange={handleValuesChange}>
{%- if rootInfo.qJson and rootInfo.qJson.searchRefs  is iterable %}
  {%- for b in rootInfo.qJson.searchRefs %}
  {%- if not b.htmlInputType %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <Input
              allowClear
              placeholder={
                '请输入{{ b.label }}'
              }
            />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType  == "Input" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <Input
              allowClear
              placeholder={
                '请输入{{ b.label }}'
              }
            />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "InputNumber" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <InputNumber 
              placeholder={
                '请输入{{ b.label }}'
              } 
            />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Text" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <InputText 
              allowClear
              placeholder={
                '请输入{{ b.label }}'
              } 
            />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Checkbox" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
            valuePropName="checked"
          >
            <Checkbox />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "DateTime" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <CustomDatePick 
              format="YYYY-MM-DDTHH:mm:ssZ"
              displayFormat='YYYY-MM-DD HH:mm:ss'
            />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Date" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <CustomDatePick 
              format="YYYY-MM-DDTHH:mm:ssZ"
              displayFormat='YYYY-MM-DD HH:mm:ss' 
            />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Time" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            {/* <TimePicker format="HH:mm:ss" /> */}
            <CustomTimePicker format="HH:mm:ss" />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Ref" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.refAttributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <RefPicker {...getQueryAttributeRef('{{ b.attributeName }}', queryConf)!} />
          </Form.Item>
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "Select" %}
          <Form.Item
            label={'{{ b.label }}'}
            name={'{{ b.attributeName }}'}
{%- raw %}
            style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <Select placeholder={'请选择'} >
        {%- for enumColumn in b.enumConfig.enumColumns %}
              <Select.Option value={'{{ enumColumn.enumValue }}'}>{{ enumColumn.displayName }}</Select.Option>
        {%- endfor %}
            </Select>
          </Form.Item>
      {%- endif %}
  {%- endfor %}
          <Form.Item 
{%- raw %}
          style={{ padding: '5px 0px 5px 0px' }}
{%- endraw %}
          >
            <Button type="primary" htmlType="submit" onClick={handleSearch}
              disabled={fgDisabled}
            >
              查询
            </Button>
          </Form.Item>
{%- endif %}
        </Form>
      </div>
    </>
  );
};

export default SearchAreaComponent;
