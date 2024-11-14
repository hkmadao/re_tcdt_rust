<template>
  <el-form :inline="true" :model="formInline" class="demo-form-inline">
    <el-form-item v-for="searcheRef in searcheRefs" :label="searcheRef.label">
      <el-select v-if="searcheRef.valueType === 'Select'" v-model="formInline[searcheRef.attributeName!]"
        :placeholder="searcheRef.label" clearable :style="{ minWidth: '100px' }">
        <el-option v-for="valueEnum in searcheRef.enumConfig?.enumColumns" :label="valueEnum.displayName"
          :value="valueEnum.enumValue ?? ''" />
      </el-select>
      <el-input v-if="searcheRef.valueType === 'Input'" v-model="formInline[searcheRef.attributeName!]"
        :placeholder="searcheRef.label" clearable />
      <el-input-number v-if="searcheRef.valueType === 'InputNumber'" v-model="formInline[searcheRef.attributeName!]"
       :placeholder="searcheRef.label" clearable />
      <el-date-picker v-if="searcheRef.valueType === 'DateTime'" v-model="formInline[searcheRef.attributeName!]"
       type="datetime" format="YYYY-MM-DD HH:mm:ss" placeholder="请选择" clearable/>
      <el-date-picker v-if="searcheRef.valueType === 'Date'" v-model="formInline[searcheRef.attributeName!]"
       type="date" format="YYYY-MM-DD" placeholder="请选择" clearable/>
      <el-time-picker v-if="searcheRef.valueType === 'Time'" v-model="formInline[searcheRef.attributeName!]"
       placeholder="请选择" clearable/>
      <RefPickerInput v-if="searcheRef.valueType === 'Ref'"
        v-model:refKey="searcheRef.attributeName" 
        v-model:refObj="searcheRef.refAttributeName" 
        v-bind="searcheRef.refConfig"
      />
    </el-form-item>
    <el-form-item>
      <el-button type="primary" @click="handleSearch">查询</el-button>
    </el-form-item>
  </el-form>
</template>

<script lang="ts" setup>
import { reactive } from 'vue'
import RefPickerInput from "~/components/ref/components/index.vue";
import { queryConf, subject } from '../../conf';

const props = defineProps<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}>();
const searcheRefs = queryConf?.searchRefs;

const handleSearch = async () => {
  subject.publish({
    topic: 'search',
    producerId: props.idLayout,
    data: formInline,
  });
};

const formInline = reactive<{ [x in string]: any }>({})

const onSubmit = () => {
  console.log('submit!')
}
</script>

<style>
.demo-form-inline {
  text-align: left;
}

.demo-form-inline .el-input {
  --el-input-width: 220px;
}
</style>