import { Checkbox, Dropdown, Menu, TableColumnType, message } from 'antd';
import moment from 'moment';
import { DownOutlined } from '@ant-design/icons';
import { useSelector, useDispatch } from 'react-redux';
import { EPartName } from '@/models';
import { getRefByAttr } from '@/util';
import { billformConf } from '../../../../conf';
import CustomDateTimeText from '@/components/CustomDateTimeText';
import { 
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.header %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configList.body %}
  T{{ bt.tabClassName }},
  {%- endfor %}
{%- endif %}
} from '../../../../models';
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
{%- for ht in rootInfo.bTableJson.configList.header %}
export const useMainTableColumns: () => TableColumnType<T{{ ht.tabClassName }}>[] =
  () => {
  const dispatch = useDispatch();
  const toEdit = () => {
    message.error("to be complate");
  };

  const detail = () => {
    message.error("to be complate");
  };

  const remove = () => {
    message.error("to be complate");
  };

  const menu = (
    <Menu>
      <Menu.Item key="1" onClick={toEdit}>
        编辑
      </Menu.Item>
      <Menu.Item key="2" onClick={detail}>
        详情
      </Menu.Item>
      <Menu.Item key="3" onClick={remove}>
        删除
      </Menu.Item>
    </Menu>
  );

    return [
  {%- for b in ht.billFormFields %}
    {%- if b.fgDisplay %}
      {%- if not b.inputType %}
      {
        width: 150,
        title: '{{ b.displayName }}',
        dataIndex: '{{ b.name }}',
        key: '{{ b.name }}',
        render: (_dom: any, record: any) => {
          return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
        },
      },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Input" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "InputNumber" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Text" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Checkbox" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <><Checkbox checked={record.{{ b.name }} ?? false} /></>;
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "DateTime" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return (
              <>
                <CustomDateTimeText
                  value={{ "{ " }} record.{{b.name}} {{" }"}}
                  format="YYYY-MM-DDTHH:mm:ssZ"
                  displayFormat="YYYY-MM-DD HH:mm:ss"
                />
              </>
            );
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Date" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Time" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Ref" %}
        {
          width: 150,
          title: '{{ b.displayName }}',
          dataIndex: [
            '{{ b.name }}',
            '{{ b.refAttributeName }}',
          ],
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            const refConf = getRefByAttr(
              EPartName.Header,
              "{{ ht.tabCode }}",
              '{{ b.name }}',
              billformConf!,
            );
            if (refConf) {
              const refData = (record as any).{{ b.refAttributeName }};
              if (refData) {
                return refData[refConf.displayProp!];
              }
            }
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Select" %}
        {
          width: 150,
          fixed: 'right',
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
        {%- for enumColumn in b.enumConfig.enumColumns %}
            if (record['{{ b.name }}'] === '{{ enumColumn.enumValue }}') {
              return '{{ enumColumn.displayName }}';
            }
        {%- endfor %}
            return '--';
          },
        },
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{#
      {
        width: 150,
        fixed: 'right',
        title: '操作',
        key: 'action',
        sorter: true,
        render: () => (
          <Dropdown overlay={menu} trigger={['click']}>
            <a
              className="ant-dropdown-link"
              onClick={(e) => e.preventDefault()}
            >
              更多 <DownOutlined />
            </a>
          </Dropdown>
        ),
      },
#}
    ];
  };

  {%- for bt in rootInfo.bTableJson.configList.body %}
    {%- if bt.billFormFields is iterable %}
/**{{ bt.tabName }} */
export const use{{ bt.firstUpperTabCode }}Columns: () => TableColumnType<T{{ bt.tabClassName }}>[] =
  () => {
    return [
      {%- for b in bt.billFormFields %}
        {%- if b.fgDisplay %}
          {%- if not b.inputType %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Input" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "InputNumber" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Text" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Checkbox" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <><Checkbox checked={record.{{ b.name }} ?? false} /></>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "DateTime" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Date" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Time" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Ref" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: [
            '{{ b.name }}',
            '{{ b.refAttributeName }}',
          ],
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            const refConf = getRefByAttr(
              EPartName.Body,
              "{{ bt.tabCode }}",
              '{{ b.name }}',
              billformConf!,
            );
            if (refConf) {
              const refData = (record as any).{{ b.refAttributeName }};
              if (refData) {
                return refData[refConf.displayProp!];
              }
            }
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Select" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
          {%- for enumColumn in b.enumConfig.enumColumns %}
            if (record['{{ b.name }}'] === '{{ enumColumn.enumValue }}') {
              return '{{ enumColumn.displayName }}';
            }
          {%- endfor %}
            return '--';
          },
        },
          {%- endif %}
        {%- endif %}
      {%- endfor %}
  ];
};
    {%- endif %}
  {%- endfor %}
{%- endfor %}
{%- endif %}