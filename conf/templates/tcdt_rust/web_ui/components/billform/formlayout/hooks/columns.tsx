import { EPartName } from '@/models';
import { ProColumns } from '@ant-design/pro-table';
import { Checkbox } from 'antd';
import moment from 'moment';
import RefPicker from '@/components/Ref';
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
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "InputNumber" %}
        {
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
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Checkbox" %}
        {
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
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
          renderFormItem: (_schema, config, form) => {
            return <CustomDatePick 
                      format="YYYY-MM-DD HH:mm:ss"
                      showTime={true} 
                    />;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Date" %}
        {
          title: '{{ b.displayName }}',
          dataIndex: '{{ b.name }}',
          key: '{{ b.name }}',
          render: (text, record, _, action) => {
            return <>{record.{{ b.name }} ? record.{{ b.name }} : '--'}</>;
          },
          renderFormItem: (_schema, config, form) => {
            return <CustomDatePick format="YYYY-MM-DD" />;
          },
        },
          {%- endif %}
          {%- if b.inputType and b.inputType == "Time" %}
        {
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
