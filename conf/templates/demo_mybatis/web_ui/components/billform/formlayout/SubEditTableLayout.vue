<template>
  <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
{%- if rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType %}
        {%- if bt.refType is matching("Array") or bt.refType is matching("Single")%}
<!--=========={{ bt.tabClassName }}============-->
    <el-tab-pane label="{{ bt.tabName }}" name="{{ bt.tabCode }}">
      <div :style="{
        display: 'flex',
        flexDirection: 'column',
      }">
        <div class="table-action">
          <el-button @click="handleAdd{{ bt.tabClassName }}"
        {%- if bt.refType and bt.refType is matching("Single") %}
            :disabled="!!{{ bt.tabCode }} {%- if bt.refType and bt.refType is matching("Single") %}&& {{ bt.tabCode }}.length > 0{%- endif %}"
        {%- endif %}
          >新增</el-button>
        </div>
        <EditTable ref="{{ bt.tabCode }}TableRef" class="edit-table" key-prop="id{{ bt.tabClassName }}" :data-source="{{ bt.tabCode }}"
          :on-row-save="{{ bt.tabCode }}HandleRowSave">
        {%- for b in bt.billFormFields %}
          {%- if b.fgDisplay %}
            {%- if b.inputType %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-input v-model="row.{{ b.name }}" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Input") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-input v-model="row.{{ b.name }}" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("InputNumber") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-input-number v-model="row.{{ b.name }}" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Text") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-input v-model="row.{{ b.name }}" type="textarea"/>
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Checkbox") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <el-checkbox v-model="row.{{ b.name }}" disabled>{{ b.displayName }}</el-checkbox>
            </template>
            <template #edit="{ row }">
              <el-checkbox v-model="row.{{ b.name }}">{{ b.displayName }}</el-checkbox>
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("DateTime") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-date-picker v-model="row.{{ b.name }}" type="datetime" format="YYYY-MM-DD HH:mm:ss" placeholder="请选择{{ b.displayName }}" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Date") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-date-picker v-model="row.{{ b.name }}" type="date" format="YYYY-MM-DD" placeholder="请选择{{ b.displayName }}" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Time") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #edit="{ row }">
              <el-time-picker v-model="row.{{ b.name }}" placeholder="请选择{{ b.displayName }}" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Ref") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row, actions, $index }">
              <div>{{ "{{" }} row.{{ b.refAttributeName }}?.{{ b.refConfig.displayProp }} }}</div>
            </template>
            <template #edit="{ row }">
              <RefPickerInput v-model:refKey="row.{{ b.name }}" v-model:refObj="row.{{ b.refAttributeName }}"
                v-bind="getRefByAttr(EPartName.Body, '{{ bt.tabCode }}', '{{ b.name }}', billformConf)" />
            </template>
          </EditTableColumn>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Select") %}
          <EditTableColumn prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div v-if="!row.{{ b.name }}">---</div>
              {%- for enumColumn in b.enumConfig.enumColumns %}
              <div v-else-if="row.{{ b.name }} === '{{ enumColumn.enumValue }}'">{{ enumColumn.displayName }}</div>
              {%- endfor %}
            </template>
            <template #edit="{ row }">
              <el-select v-model="row.{{ b.name }}" placeholder="请选择{{ b.displayName }}" :clearable="true" :style="{ minWidth: '100px' }">
              {%- for enumColumn in b.enumConfig.enumColumns %}
                <el-option label='{{ enumColumn.displayName }}' value='{{ enumColumn.enumValue }}'></el-option>
              {%- endfor %}  
              </el-select>
            </template>
          </EditTableColumn>
            {%- endif %}
          {%- endif %}
        {%- endfor %}
          <EditTableColumn label="修改">
            <template #default="{ row, actions, $index }">
              <div :style="{
                display: 'flex',
                flexDirection: 'row',
              }">
                <el-button type="text" @click="handleToUpdate{{ bt.tabClassName }}(row.id{{ bt.tabClassName }})">修改</el-button>
                <el-button type="text" @click="handleDelete{{ bt.tabClassName }}(row.id{{ bt.tabClassName }})">删除</el-button>
              </div>
            </template>
            <template #edit="{ row, actions, $index }">
              <div :style="{
                display: 'flex',
                flexDirection: 'row',
              }">
                <el-button type="text" @click="handleUpdate{{ bt.tabClassName }}(row.id{{ bt.tabClassName }})">保存</el-button>
                <el-button type="text" @click="handleCancel{{ bt.tabClassName }}(row.id{{ bt.tabClassName }})">取消</el-button>
                <el-button type="text" @click="handleDelete{{ bt.tabClassName }}(row.id{{ bt.tabClassName }})">删除</el-button>
              </div>
            </template>
          </EditTableColumn>
        </EditTable>
        <div :style="{ alignSelf: 'end' }">
          <el-pagination :style="{ alignSelf: 'flex-end', }" :page-sizes="[10, 20, 50, 100]" :small="true"
            layout="total, sizes, prev, pager, next, jumper" :total="{{ bt.tabCode }}?.length" />
        </div>
      </div>
    </el-tab-pane>
<!--=========={{ bt.tabClassName }}=============-->
      {%- endif %}
    {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
  </el-tabs>
</template>
<script lang="ts" setup>
import { ref, computed, } from 'vue'
import { TabsPaneContext } from 'element-plus'
import { nanoid } from 'nanoid';
import RefPickerInput from "~/components/ref/components/index.vue";
import { billformConf, } from '../../../conf';
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
} from '../../../models';
import { getRefByAttr } from '~/util';
import { DOStatus, EPartName, } from '~/models';
import { useFormData } from './hooks';
import { useFormStore } from './store';

const { ...actions } = useFormStore()
const formData = computed(() => useFormData())

const handleClick = (tab: TabsPaneContext, event: Event) => {
  // console.log(tab, event)
}

{%- if rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields and loop.first %}
  const activeName = '{{ bt.tabCode }}';
    {%- endif %}
  {%- endfor %}
{%- endif %}

{%- if rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType is matching("Array") %}
/*=========={{ bt.tabClassName }}=============*/
const {{ bt.tabCode }}TableRef = ref();
const {{ bt.tabCode }} = computed(() => formData.value.{{ bt.tabCode }}?.filter(c => c.action !== DOStatus.DELETED))

const {{ bt.tabCode }}HandleRowSave = (row: T{{ bt.tabClassName }}) => {
  actions.update{{ bt.tabClassName }}s(row)
}
const handleAdd{{ bt.tabClassName }} = () => {
  const {{ bt.tabCode }}: T{{ bt.tabClassName }} = { id{{ bt.tabClassName }}: nanoid(), }
  {{ bt.tabCode }}TableRef.value.editActions.addRow({{ bt.tabCode }})
  actions.add{{ bt.tabClassName }}s({{ bt.tabCode }})
}
const handleToUpdate{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.startEditable(key)
}
const handleUpdate{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.saveEditable(key)
}
const handleDelete{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.deleteRow(key)
  actions.delete{{ bt.tabClassName }}s({ id{{ bt.tabClassName }}: key })
}
const handleCancel{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.cancelEditable(key)
}
/*=========={{ bt.tabClassName }}=============*/
    {%- endif %}
    {%- if bt.refType and bt.refType is matching("Single") %}
/*=========={{ bt.tabClassName }}=============*/
const {{ bt.tabCode }}TableRef = ref();
const {{ bt.tabCode }} = computed(() => {
  if (formData.value.{{ bt.tabCode }} && formData.value.{{ bt.tabCode }}.action !== DOStatus.DELETED) {
    return [ formData.value.{{ bt.tabCode }} ]
  }
  return [];
})

const {{ bt.tabCode }}HandleRowSave = (row: T{{ bt.tabClassName }}) => {
  actions.update{{ bt.tabClassName }}(row)
}
const handleAdd{{ bt.tabClassName }} = () => {
  const {{ bt.tabClassName }}: T{{ bt.tabClassName }} = { id{{ bt.tabClassName }}: nanoid(), }
  {{ bt.tabCode }}TableRef.value.editActions.addRow({{ bt.tabClassName }})
  actions.add{{ bt.tabClassName }}({{ bt.tabClassName }})
}
const handleToUpdate{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.startEditable(key)
}
const handleUpdate{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.saveEditable(key)
}
const handleDelete{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.deleteRow(key)
  actions.delete{{ bt.tabClassName }}()
}
const handleCancel{{ bt.tabClassName }} = (key: string) => {
  {{ bt.tabCode }}TableRef.value.editActions.cancelEditable(key)
}
/*=========={{ bt.tabClassName }}=============*/
      {%- endif %}
    {%- endif %}
  {%- endfor %}
{%- endif %}
</script>
<style>
.demo-tabs>.el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}

.edit-table {
  width: 100%;
  min-height: '200px';
}

.table-action {
  align-self: self-start;
}
</style>