import { FC, Key, memo, useEffect, useState } from 'react';
import { Table, Tabs } from 'antd';
import { DownOutlined } from '@ant-design/icons';
import moment from 'moment';
import styles from './styles.less';
import { 
  useSelectRow,
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.body %}
    {%- if bt.billFormFields is iterable %}
  use{{ bt.firstUpperTabCode }}Columns,
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from '../hooks';
import { 
{%- if rootInfo.bMDJson and rootInfo.bMDJson.children is iterable %}
  {%- for field in rootInfo.bMDJson.children %}
    {%- if field.children is iterable and field.entityInfo and field.entityInfo.className != rootInfo.bMDJson.entityInfo.className %}
  T{{ field.entityInfo.className }},
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from '../../../../models';

const SubTableLayout: FC = () => {
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.body is iterable %}
  const selectRow = useSelectRow();
  {%- for bt in rootInfo.bTableJson.configList.body %}
    {%- if bt.billFormFields is iterable %}
  const {{ bt.tabCode }}Columns = use{{ bt.firstUpperTabCode }}Columns();
    {%- endif %}
  {%- endfor %}

  const { TabPane } = Tabs;

{%- endif %}
  return (
    <>
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.body is iterable %}
      <Tabs defaultActiveKey={ 
  {%- if rootInfo.bTableJson.configList.body | length > 0 -%}
    {%- for bt in rootInfo.bTableJson.configList.body -%}
      {%- if bt.fgDefaultTab -%}
'{{ bt.tabCode }}' 
      {%- endif -%}
    {%- endfor -%}
  {%- else -%}
''
  {%- endif -%}
  }>
  {%- for bt in rootInfo.bTableJson.configList.body %}
    {%- if bt.billFormFields is iterable %}
        <TabPane
          key={'{{ bt.tabCode }}'}
          tabKey={'{{ bt.tabCode }}'}
          tab={'{{ bt.tabName }}'}
        >
          <Table
            className={styles['my-ant-card-body']}
            rowKey={'{{ bt.mainProperty }}'}
            columns={{ "{" }}{{ bt.tabCode }}Columns}
      {%- if bt.refType and bt.refType == "Array" %}
            dataSource={selectRow && selectRow.{{ bt.tabAttrName }} ? selectRow.{{ bt.tabAttrName }} :[] }
      {%- endif %}
      {%- if bt.refType and bt.refType == "Single" %}
            dataSource={selectRow && selectRow.{{ bt.tabAttrName }} ? [selectRow.{{ bt.tabAttrName }}] :[] }
      {%- endif %}
            pagination={false}
          />
        </TabPane>
    {%- endif %}
  {%- endfor %}
      </Tabs>
{%- endif %}
    </>
  );
};

export default SubTableLayout;
