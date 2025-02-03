{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
<template>
  <div :style="{
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'stretch'
  }">
    {%- if ht.billFormFields is iterable %}
    <el-form :model="form" :inline="true" ref="formRef" :style="{
      minHeight: '300px',
    }">
      <el-space direction="horizontal" alignment="start" :size="30" :style="{
        flexWrap: 'wrap',
      }">
      {%- for b in ht.billFormFields %}
        {%- if b.inputType %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-input 
            v-model="form.{{ b.name }}" 
            allowClear
            placeholder='请输入{{ b.displayName }}'
          />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Input") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-input 
            v-model="form.{{ b.name }}" 
            allowClear
            placeholder='请输入{{ b.displayName }}'
          />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("InputNumber") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-input-number v-model="form.{{ b.name }}" />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Text") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <input v-model="form.{{ b.name }}" type="textarea"/>
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Checkbox") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-checkbox 
            v-model="form.{{ b.name }}"
          >
            {{ b.displayName }}
          </el-checkbox>
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("DateTime") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-date-picker v-model="form.{{ b.name }}" type="datetime" format="YYYY-MM-DD HH:mm:ss" placeholder="请选择{{ b.displayName }}" />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Date") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-date-picker v-model="form.{{ b.name }}" type="date" format="YYYY-MM-DD" placeholder="请选择{{ b.displayName }}" />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Time") %}
        <el-form-item 
          label='{{ b.displayName }}'
          {%- if b.fgDisplay %}''{%- else %}'hidden'{%- endif %}
        >
          <el-time-picker v-model="form.{{ b.name }}" placeholder="请选择{{ b.displayName }}" />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Ref") %}
        <el-form-item label='{{ b.displayName }}'>
          <RefPickerInput 
            v-model:refKey="form.{{ b.name }}" 
            v-model:refObj="form.{{ b.refAttributeName }}" 
            v-bind="getRefByAttr(EPartName.Header,'{{ ht.tabCode }}','{{ b.name }}', billformConf!)"
          />
        </el-form-item>
        {%- endif %}
        {%- if b.inputType and b.inputType is matching("Select") %}
        <el-form-item label='{{ b.displayName }}'>
          <el-select v-model="form.{{ b.name }}" placeholder="请选择{{ b.displayName }}" :clearable="true" :style="{ minWidth: '100px' }">
          {%- for enumColumn in b.enumConfig.enumColumns %}
            <el-option label='{{ enumColumn.displayName }}' value='{{ enumColumn.enumValue }}'></el-option>
          {%- endfor %}
          </el-select>
        </el-form-item>
        {%- endif %}
      {%- endfor %}
      </el-space>
    </el-form>
    {%- endif %}
    <SubEditTableLayout />
  </div>
</template>
  
<script lang="ts" setup>

import { onMounted, onBeforeUnmount, ref, computed, watch, } from 'vue'
import { FormInstance } from 'element-plus'
import RefPickerInput from "~/components/ref/components/index.vue";
import { EPartName } from '~/models';
import { Observer, TMessage } from '~/util/observer';
import SubEditTableLayout from './SubEditTableLayout.vue'
import { billformConf, subject } from '../../../conf';
import { T{{ ht.tabClassName }} } from '../../../models';
import { getRefByAttr } from '~/util';
import { useFormStore } from './store';
import { useEditStatusInfo, useIdUiConf } from './hooks';

const props = defineProps<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}>();
const { store, ...actions } = useFormStore()
const idUiConfRef = computed(() => useIdUiConf())
const editStatusInfo = computed(() => useEditStatusInfo())
const formRef = ref<FormInstance>();

const form = ref<T{{ ht.tabClassName }}>({});
const observers: Observer[] = [];

watch(editStatusInfo, () => {
  form.value = store.formData
})

onMounted(() => {
  actions.setComponentInfo({
    idUiConf: props.idLayout,
    fgDisabled: props.fgDisabled,
  })
  if (!idUiConfRef.value) {
    return;
  }
  const cancleObserver: Observer = {
    topic: 'cancel',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(idUiConfRef.value)) {
        return;
      }
      actions.cancel();
    },
  };
  subject.subscribe(cancleObserver);

  const toAddObserver: Observer = {
    topic: 'toAdd',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(idUiConfRef.value)) {
        return;
      }
      actions.addFormData({ nodeData: message.data?.treeSelectedNode });
    },
  };
  subject.subscribe(toAddObserver);

  const addObserver: Observer = {
    topic: 'add',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        const fgValidate = await formRef.value?.validate();
        if (!fgValidate) {
          return;
        }
        actions.saveThunks({ actionType: 'add' });
      })();
    },
  };
  subject.subscribe(addObserver);

  const addAgainObserver: Observer = {
    topic: 'addAgain',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        const fgValidate = await formRef.value?.validate();
        if (!fgValidate) {
          return;
        }
        actions.saveThunks({ actionType: 'addAgain' });
      })();
    },
  };
  subject.subscribe(addAgainObserver);

  const toEditObserver: Observer = {
    topic: 'toEdit',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.toEditThunks({
          nodeData: message.data?.treeSelectedNode,
          selectedRow: message.data.selectedRow,
        });
      })();
    },
  };
  subject.subscribe(toEditObserver);

  const editObserver: Observer = {
    topic: 'edit',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        const fgValidate = await formRef.value?.validate();
        if (!fgValidate) {
          return;
        }
        actions.saveThunks({ actionType: 'edit' });
      })();
    },
  };
  subject.subscribe(editObserver);

  const detailRefleshObserver: Observer = {
    topic: 'detailReflesh',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.refleshThunks();
      })();
    },
  };
  subject.subscribe(detailRefleshObserver);

  observers.push(toAddObserver);
  observers.push(addObserver);
  observers.push(addAgainObserver);
  observers.push(toEditObserver);
  observers.push(editObserver);
  observers.push(detailRefleshObserver);
})

onBeforeUnmount(() => {
  //销毁观察者
  observers.forEach(observer => subject.unsubsribe(observer))
})
</script>
  {%- endfor %}
{%- endif %}
  