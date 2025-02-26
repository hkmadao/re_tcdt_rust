import { Checkbox, Dropdown, Menu, TableColumnType, message, Popover, } from 'antd';
import moment from 'moment';
import { DownOutlined } from '@ant-design/icons';
import { useSelector, useDispatch } from 'react-redux';
import { EPartName } from '@/models';
import { getRefByAttr } from '@/util';
import { billformConf } from '../../../../conf';
import CustomDateText from '@/components/CustomDateText';
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
        width: {{ b.width }},
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            const content = record.{{ b.name }} ? record.{{ b.name }} : '--';
            return (
              <div
                style={{ "{{" }}
                  overflow: 'hidden',
                  width: '{{ b.textLen }}px',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                {{ "}}" }}
              >
                <Popover content={content} trigger="hover">
                  {content}
                </Popover>
              </div>
            );
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "InputNumber" %}
        {
          width: {{ b.width }},
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            const content = record.{{ b.name }} ? record.{{ b.name }} : '--';
            return (
              <div
                style={{ "{{" }}
                  overflow: 'hidden',
                  width: '{{ b.textLen }}px',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                {{ "}}" }}
              >
                <Popover content={content} trigger="hover">
                  {content}
                </Popover>
              </div>
            );
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Checkbox" %}
        {
          width: {{ b.width }},
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return (
              <>
                <CustomDateText
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return (
              <>
                <CustomDateText
                  value={{ "{ " }} record.{{b.name}} {{" }"}}
                  format="YYYY-MM-DDTHH:mm:ssZ"
                  displayFormat="YYYY-MM-DD HH:mm:ss"
                />
              </>
            );
          },
        },
      {%- endif %}
      {%- if b.inputType and b.inputType == "Time" %}
        {
          width: {{ b.width }},
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
          width: {{ b.width }},
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
          width: {{ b.width }},
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
        width: {{ b.width }},
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
          width: {{ b.width }},
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            const content = record.{{ b.name }} ? record.{{ b.name }} : '--';
            return (
              <div
                style={{ "{{" }}
                  overflow: 'hidden',
                  width: '{{ b.textLen }}px',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                {{ "}}" }}
              >
                <Popover content={content} trigger="hover">
                  {content}
                </Popover>
              </div>
            );
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "InputNumber" %}
        {
          width: {{ b.width }},
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            const content = record.{{ b.name }} ? record.{{ b.name }} : '--';
            return (
              <div
                style={{ "{{" }}
                  overflow: 'hidden',
                  width: '{{ b.textLen }}px',
                  textOverflow: 'ellipsis',
                  whiteSpace: 'nowrap',
                {{ "}}" }}
              >
                <Popover content={content} trigger="hover">
                  {content}
                </Popover>
              </div>
            );
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Checkbox" %}
        {
          width: {{ b.width }},
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return (
              <>
                <CustomDateText
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
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (_dom: any, record: any) => {
            return (
              <>
                <CustomDateText
                  value={{ "{ " }} record.{{b.name}} {{" }"}}
                  format="YYYY-MM-DDTHH:mm:ssZ"
                  displayFormat="YYYY-MM-DD HH:mm:ss"
                />
              </>
            );
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Time" %}
        {
          width: {{ b.width }},
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
          width: {{ b.width }},
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
          width: {{ b.width }},
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