import { FC, ReactNode, useEffect, useRef, useState } from 'react';
import { Button, Checkbox, Form, Input, InputNumber, Select } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import RefPicker from '@/components/Ref';
import { Observer, TMessage } from '@/util/observer';
import { subject, queryConf, } from '../../conf';
import { usePageCode } from '../../hooks';

const SearchArea: FC<{
  idLayout: string
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}> = ({ idLayout, fgDisabled }) => {
  const [componentFgDiabled, setComponentFgDiabled] = useState<boolean>(fgDisabled);
  const pageCode = usePageCode();
  const searcheRefs = queryConf?.searchRefs;
  const [itemNodes, setItemNodes] = useState<ReactNode[]>([]);
  const [form] = Form.useForm();
  const dispatch = useDispatch();
  const searchValuesRef = useRef<any>({});

  useEffect(() => {
    setComponentFgDiabled(fgDisabled);
  }, [fgDisabled]);

  useEffect(() => {
    const treeNodeObserver: Observer = {
      topic: 'treeNodeSelected',
      consumerId: idLayout,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idLayout)) {
            return;
          }
          form.resetFields();
        })();
      },
    };
    subject.subscribe(treeNodeObserver);

    const treeNodeCancelObserver: Observer = {
      topic: 'treeSelectCancel',
      consumerId: idLayout,
      update: function (message: TMessage): void {
        (async () => {
          if (!message || message.consumerIds.includes(idLayout)) {
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
  }, []);

  useEffect(() => {
    const newValues:any = {};
{%- if rootInfo.qJson and rootInfo.qJson.searchRefs is iterable %}
  {%- for b in rootInfo.qJson.searchRefs %}
    {%- if b.defaultValue == false or b.defaultValue | defined %}
      {%- if b.htmlInputType and b.htmlInputType  == "Input" %}
    newValues.{{ b.attributeName }} = '{{ b.defaultValue }}';
      {%- endif %}
      {%- if b.htmlInputType and b.htmlInputType == "InputNumber" %}
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
    subject.publish({
      topic: 'search',
      producerId: idLayout,
      data: searchValuesRef.current,
    });
  };

  return (
    <>
      <div
{%- raw %}
        style={{
          display: pageCode === 'form' ? 'none' : 'block',
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
            {/* <DatePicker
              format="YYYY-MM-DD HH:mm:ss"
              showTime={true}
            /> */}
            <CustomDatePick 
              format="YYYY-MM-DD HH:mm:ss"
              showTime={true} 
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
            {/* <DatePicker format="YYYY-MM-DD" /> */}
            <CustomDatePick format="YYYY-MM-DD" />
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
            <RefPicker
              {...getRefByAttr(
                EPartName.Header,
                '{{ ht.tabCode }}',
                '{{ b.attributeName }}',
                billformConf!,
              )!}
            />
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
              disabled={componentFgDiabled}
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

export default SearchArea;
