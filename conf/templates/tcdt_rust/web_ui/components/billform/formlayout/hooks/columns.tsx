import { EPartName } from '@/models';
import { ProColumns } from '@ant-design/pro-table';
import { Checkbox, Popover, } from 'antd';
import moment from 'moment';
import RefPicker from '@/components/Ref';
import CustomDateText from '@/components/CustomDateText';
import CustomDatePick from '@/components/CustomDatePick';
import CustomTimePicker from '@/components/CustomTimePicker';
import { getRefByAttr } from '@/util';
import { billformConf } from '../../../../conf';
import { 
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
  T{{ bt.tabClassName }},
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from '../../../../models';

export * from '.';

{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
/**{{ bt.tabName }} */
export const use{{ bt.firstUpperTabCode }}Columns: () => ProColumns<T{{ bt.tabClassName }}>[] =
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
          render: (text, record, _, action) => {
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
          render: (text, record, _, action) => {
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
          render: (text, record, _, action) => {
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
          render: (text, record, _, action) => {
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
          valueType: 'checkbox',
          formItemProps: { valuePropName: 'checked' },
          render: (text, record, _, action) => {
            return <><Checkbox checked={record.{{ b.name }} ?? false} /></>;
          },
          renderFormItem: (_schema, config, form) => {
            return <Checkbox />;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "DateTime" %}
        {
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
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
          renderFormItem: (_schema, config, form) => {
            return <CustomDatePick 
                      format="YYYY-MM-DDTHH:mm:ssZ"
                      displayFormat="YYYY-MM-DD HH:mm:ss"
                    />;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Date" %}
        {
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
            return <>
              <CustomDateText
                value={{ "{ " }} record.{{b.name}} {{" }"}}
                format="YYYY-MM-DDTHH:mm:ssZ"
                displayFormat="YYYY-MM-DD HH:mm:ss"
              />
            </>
          },
          renderFormItem: (_schema, config, form) => {
            return <CustomDatePick 
                      format="YYYY-MM-DDTHH:mm:ssZ"
                      displayFormat="YYYY-MM-DD HH:mm:ss"
                    />;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Time" %}
        {
          width: {{ b.width }},
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
          renderFormItem: (_schema, config, form) => {
            return <CustomTimePicker format="HH:mm:ss" />;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Ref" %}
      {
        width: {{ b.width }},
        title: '{{ b.displayName }}',
        dataIndex: '{{ b.refAttributeName }}',
        key: '{{ b.refAttributeName }}',
        renderFormItem: (_schema, config, form) => {
          const refConf = getRefByAttr(
            EPartName.Body,
            "{{ bt.tabCode }}",
            '{{ b.name }}',
            billformConf,
          );
          if (refConf) {
            return (
              <RefPicker {...refConf!} />
            );
          }
        },
        render: (_dom, record) => {
          const refConf = getRefByAttr(
            EPartName.Body,
            "{{ bt.tabCode }}",
            '{{ b.name }}',
            billformConf,
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
        valueType: 'select',
        valueEnum: {
            {%- for enumColumn in b.enumConfig.enumColumns %}
          {{ enumColumn.enumValue }}: { 
            text: '{{ enumColumn.displayName }}', 
            status: '{{ enumColumn.enumValue }}',
          },
            {%- endfor %}
        },
      },
          {%- endif %}
        {%- endif %}
      {%- endfor %}
  ];
};
    {%- endif %}
  {%- endfor %}
{%- endif %}

