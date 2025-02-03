import { DOStatus } from '@/models';
import { ActionType, EditableProTable } from '@ant-design/pro-table';
import { nanoid } from '@reduxjs/toolkit';
import { PlusOutlined } from '@ant-design/icons';
import { useSelector, useDispatch } from 'react-redux';
import { Form, Space, Button } from 'antd';
import { FC, useRef, useState, useEffect } from 'react';
import styles from './styles.less';
import { actions, } from '../store';
import { 
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
  T{{ bt.tabClassName }},
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from '../../../../models';
import { 
  useFgDisabled,
  useFormData,
  useEditStatusInfo,
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
  use{{ bt.firstUpperTabCode }}Columns,
  use{{ bt.firstUpperTabCode }}Data,
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from '../hooks';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType == "Array" %}
/*=========={{ bt.firstUpperTabCode }}=============*/
export const {{ bt.firstUpperTabAttrName }}: FC = () => {
  const fgDisabled = useFgDisabled();
  const actionRef = useRef<ActionType>();
  const [editableKeys, setEditableRowKeys] = useState<React.Key[]>([]);
  const [form] = Form.useForm<T{{ bt.tabClassName }}>();
  const dispatch = useDispatch();
  const tableData = use{{ bt.firstUpperTabCode }}Data();
  const moduleData = useFormData();
  const [ {{ bt.tabCode }}, set{{ bt.firstUpperTabCode }}] = useState<T{{ bt.tabClassName }}[]>([]);
  const editStatus = useEditStatusInfo();

  useEffect(() => {
    if (editStatus) {
      set{{ bt.firstUpperTabCode }}(tableData);
    }
  }, [editStatus, tableData.length]);

  const {{ bt.tabCode }}Columns = use{{ bt.firstUpperTabCode }}Columns();

  /**编辑行内容改变处理 */
  const handleFormChange: (
    record: T{{ bt.tabClassName }},
    dataSource: T{{ bt.tabClassName }}[],
  ) => void = (record, dataSource) => {
    const newRecord = { ...record };
  {%- if bt.billFormFields is iterable %}
    {%- for b in bt.billFormFields %}
      {%- if b.inputType and b.inputType == "Ref" %}
    newRecord.{{ b.name }} = undefined;  
    if (newRecord.{{ b.refAttributeName }}) {
      newRecord.{{ b.name }} = newRecord.{{ b.refAttributeName }}.{{ b.refConfig.backWriteProp }};
    }
      {%- endif %}
    {%- endfor %}
  {%- endif %}
    dispatch(actions.updateFormData{{ bt.tabClassName }}(newRecord));
  };

  /**行操作 */
  const handleRow = (record: T{{ bt.tabClassName }}) => {
    return {
      onClick: async (_event: any) => {
        if(fgDisabled){
          return;
        }
        editableKeys.forEach((editableKey) =>
          actionRef.current?.cancelEditable(editableKey),
        );
        actionRef.current?.startEditable(record.{{ bt.mainProperty }}!);
      }, // 点击行
      onDoubleClick: (_event: any) => {},
      onContextMenu: (_event: any) => {},
      onMouseEnter: (_event: any) => {}, // 鼠标移入行
      onMouseLeave: (_event: any) => {},
    };
  };

  /**添加行 */
  const handleAddRow = () => {
    const {{ bt.firstLowerTabClassName }}New: T{{ bt.tabClassName }} = {
      {{ rootInfo.bMDJson.entityInfo.pkAttributeInfo.attributeName }}: moduleData.{{ rootInfo.bMDJson.entityInfo.pkAttributeInfo.attributeName }},
      {{ bt.mainProperty }}: nanoid(),
      action: DOStatus.NEW,
    };
    dispatch(actions.addFormData{{ bt.tabClassName }}({{ bt.firstLowerTabClassName }}New));
    editableKeys.forEach((editableKey) =>
      actionRef.current?.cancelEditable(editableKey),
    );
    actionRef.current?.startEditable(
      {{ bt.firstLowerTabClassName }}New.{{ bt.mainProperty }} as React.Key,
    );
  };
  /**删除行 */
  const handleDelete = () => {
    if (editableKeys && editableKeys.length === 1) {
      const {{ bt.firstLowerTabClassName }} = {{ bt.tabCode }}?.find(
        (t) =>
          t.action !== DOStatus.DELETED &&
          editableKeys[0] === t.{{ bt.mainProperty }},
      );
      if ({{ bt.firstLowerTabClassName }}) {
        dispatch(actions.deleteFormData{{ bt.tabClassName }}({{ bt.firstLowerTabClassName }}));
      }
    }
  };

  return (
    <>
      <Space size={2}>
        <Button size={'small'} onClick={handleAddRow} icon={<PlusOutlined />} disabled={fgDisabled}>
          添加
        </Button>
        <Button
          size={'small'}
          onClick={handleDelete}
          disabled={!editableKeys || editableKeys.length == 0 || fgDisabled}
        >
          删除
        </Button>
      </Space>
      <EditableProTable<T{{ bt.tabClassName }}>
        className={styles['my-ant-card-body']}
{%- raw %}
        style={{ padding: '0px' }}
{%- endraw %}
        actionRef={actionRef}
        rowKey={'{{ bt.mainProperty }}'}
        headerTitle={false}
        bordered={true}
        size={'small'}
        maxLength={5}
        recordCreatorProps={false}
        value={{ "{" }}{{ bt.tabCode }}{{ "}" }}
        columns={{ "{" }}{{ bt.tabCode }}Columns}
{%- raw %}
        editable={{
          type: 'multiple',
          form,
          editableKeys: editableKeys,
          onChange: setEditableRowKeys,
          onValuesChange: handleFormChange,
        }}
{%- endraw %}
        onRow={handleRow}
      />
    </>
  );
};
/*=========={{ bt.firstUpperTabCode }}=============*/
      {%- endif %}
      {%- if bt.refType and bt.refType == "Single" %}
/*=========={{ bt.firstUpperTabCode }}=============*/
export const {{ bt.firstUpperTabAttrName }}: FC = () => {
  const fgDisabled = useFgDisabled();
  const actionRef = useRef<ActionType>();
  const [editableKeys, setEditableRowKeys] = useState<React.Key[]>([]);
  const [form] = Form.useForm<T{{ bt.tabClassName }}>();
  const dispatch = useDispatch();
  const tableData = use{{ bt.firstUpperTabCode }}Data();
  const moduleData = useFormData();
  const [ {{ bt.tabCode }}, set{{ bt.firstUpperTabCode }}] = useState<T{{ bt.tabClassName }}[]>([]);
  const editStatus = useEditStatusInfo();

  useEffect(() => {
    if (editStatus) {
      set{{ bt.firstUpperTabCode }}(tableData);
    }
  }, [editStatus, tableData.length]);

  const {{ bt.tabCode }}Columns = use{{ bt.firstUpperTabCode }}Columns();

  /**编辑行内容改变处理 */
  const handleFormChange: (
    record: T{{ bt.tabClassName }},
    dataSource: T{{ bt.tabClassName }}[],
  ) => void = (record, dataSource) => {
    const newRecord = { ...record };
  {%- if bt.billFormFields is iterable %}
    {%- for b in bt.billFormFields %}
      {%- if b.inputType and b.inputType == "Ref" %}
    newRecord.{{ b.name }} = undefined;  
    if (newRecord.{{ b.refAttributeName }}) {
      newRecord.{{ b.name }} = newRecord.{{ b.refAttributeName }}.{{ b.refConfig.backWriteProp }};
    }
      {%- endif %}
    {%- endfor %}
  {%- endif %}
    dispatch(actions.updateFormData{{ bt.tabClassName }}(newRecord));
  };

  /**行操作 */
  const handleRow = (record: T{{ bt.tabClassName }}) => {
    return {
      onClick: async (_event: any) => {
        if(fgDisabled){
          return;
        }
        editableKeys.forEach((editableKey) =>
          actionRef.current?.cancelEditable(editableKey),
        );
        actionRef.current?.startEditable(record.{{ bt.mainProperty }}!);
      }, // 点击行
      onDoubleClick: (_event: any) => {},
      onContextMenu: (_event: any) => {},
      onMouseEnter: (_event: any) => {}, // 鼠标移入行
      onMouseLeave: (_event: any) => {},
    };
  };

  /**添加行 */
  const handleAddRow = () => {
    const {{ bt.firstLowerTabClassName }}New: T{{ bt.tabClassName }} = {
      {{ rootInfo.bMDJson.entityInfo.pkAttributeInfo.attributeName }}: moduleData.{{ rootInfo.bMDJson.entityInfo.pkAttributeInfo.attributeName }},
      {{ bt.mainProperty }}: nanoid(),
      action: DOStatus.NEW,
    };
    dispatch(actions.addFormData{{ bt.tabClassName }}({{ bt.firstLowerTabClassName }}New));
    editableKeys.forEach((editableKey) =>
      actionRef.current?.cancelEditable(editableKey),
    );
    actionRef.current?.startEditable(
      {{ bt.firstLowerTabClassName }}New.{{ bt.mainProperty }} as React.Key,
    );
  };
  /**删除行 */
  const handleDelete = () => {
    if (editableKeys && editableKeys.length === 1) {
      const {{ bt.firstLowerTabClassName }}New = {{ bt.tabAttrName }}[0];
      if ({{ bt.firstLowerTabClassName }}New) {
        dispatch(actions.deleteFormData{{ bt.tabClassName }}({{ bt.firstLowerTabClassName }}New));
      }
    }
  };

  return (
    <>
      <Space size={2}>
        <Button 
          size={'small'} 
          onClick={handleAddRow} 
          icon={<PlusOutlined />}
          disabled={{ "{" }}{{ bt.tabCode }} && {{ bt.tabCode }}.length === 1 || fgDisabled}
        >
          添加
        </Button>
        <Button
          size={'small'}
          onClick={handleDelete}
          disabled={!editableKeys || editableKeys.length == 0 || fgDisabled}
        >
          删除
        </Button>
      </Space>
      <EditableProTable<T{{ bt.tabClassName }}>
        className={styles['my-ant-card-body']}
{%- raw %}
        style={{ padding: '0px' }}
{%- endraw %}
        actionRef={actionRef}
        rowKey={'{{ bt.mainProperty }}'}
        headerTitle={false}
        bordered={true}
        size={'small'}
        maxLength={5}
        recordCreatorProps={false}
        value={{ "{" }}{{ bt.tabCode }}{{ "}" }}
        columns={{ "{" }}{{ bt.tabCode }}Columns}
{%- raw %}
        editable={{
          type: 'multiple',
          form,
          editableKeys: editableKeys,
          onChange: setEditableRowKeys,
          onValuesChange: handleFormChange,
        }}
{%- endraw %}
        onRow={handleRow}
      />
    </>
  );
};
/*=========={{ bt.firstUpperTabCode }}=============*/
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
