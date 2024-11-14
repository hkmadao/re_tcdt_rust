<template>
  <el-tabs v-model="activeName" class="demo-tabs" @tab-click="handleClick">
{%- if rootInfo.bTableJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType %}
        {%- if bt.refType is matching("Array") or bt.refType is matching("Single") %}
<!--=========={{ bt.tabClassName }}=============-->
    <el-tab-pane label="{{ bt.tabName }}" name="{{ bt.tabCode }}">
      <div :style="{
        display: 'flex',
        flexDirection: 'column',
      }">
        <el-table :data="{{ bt.tabCode }}" :style="{
          width: '100%',
          minHeight: '200px'
        }" highlight-current-row>
        {%- for b in bt.billFormFields %}
          {%- if b.fgDisplay %}
            {%- if b.inputType %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Input") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("InputNumber") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Text") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Checkbox") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <el-checkbox v-model="row.{{ b.name }}" disabled>{{ b.displayName }}</el-checkbox>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("DateTime") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Date") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Time") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.name }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Ref") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div>{{ "{{" }} row.{{ b.refAttributeName }}?.{{ b.refConfig.displayProp }} {{ "}}" }}</div>
            </template>
          </el-table-column>
            {%- endif %}
            {%- if b.inputType and b.inputType is matching("Select") %}
          <el-table-column prop="{{ b.name }}" label="{{ b.displayName }}">
            <template #default="{ row }">
              <div v-if="!row.{{ b.name }}">---</div>
              {%- for enumColumn in b.enumConfig.enumColumns %}
              <div v-else-if="row.{{ b.name }} === '{{ enumColumn.enumValue }}'">{{ enumColumn.displayName }}</div>
              {%- endfor %}
            </template>
          </el-table-column>
            {%- endif %}
          {%- endif %}
        {%- endfor %}
        </el-table>
        <div :style="{ alignSelf: 'end' }">
          <el-pagination :page-sizes="[10, 20, 50, 100]" :small="true"
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
import type { TabsPaneContext } from 'element-plus'
import { DOStatus } from '~/models';
{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
import { 
  T{{ ht.tabClassName }},
} from '../../../models';
import { useSelectRow } from './hooks';
  {%- endfor %}
{%- endif %}

const selectedRowRef = computed(() => useSelectRow())

const handleClick = (tab: TabsPaneContext, event: Event) => {
  // console.log(tab, event)
}

{%- if rootInfo.bTableJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configForm.body %}
    {%- if loop.first and bt.billFormFields %}
const activeName = '{{ bt.tabCode }}';
    {%- endif %}
  {%- endfor %}
{%- endif %}

{%- if rootInfo.bTableJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bTableJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
      {%- if bt.refType and bt.refType is matching("Array") %}
/*=========={{ bt.tabClassName }}=============*/
const {{ bt.tabCode }} = computed(() => selectedRowRef.value?.{{ bt.tabCode }}?.filter(c => c.action !== DOStatus.DELETED))
/*=========={{ bt.tabClassName }}=============*/
      {%- endif %}
      {%- if bt.refType and bt.refType is matching("Single") %}
/*=========={{ bt.tabClassName }}=============*/
const {{ bt.tabCode }} = computed(() => {
  if (selectedRowRef.value?.{{ bt.tabCode }} && selectedRowRef.value?.{{ bt.tabCode }}.action !== DOStatus.DELETED) {
    return [ selectedRowRef.value.{{ bt.tabCode }} ]
  }
  return [];
})
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
</style>
  