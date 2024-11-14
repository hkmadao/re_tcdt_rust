{%- if rootInfo.bTableJson and rootInfo.bTableJson.configList.header is iterable %}
  {%- for ht in rootInfo.bTableJson.configList.header %}
<template>
  <div :style="{ display: 'flex', flexDirection: 'column', flex: 'auto', gap: '10px', }">
    <div>
      <el-table v-if="selectionTypeRef === 'checkbox'" ref="tableRefCheckbox" :data="tableDataRef"
        :border="true" style="width: 100%" highlight-current-row @current-change="handleCurrentRowChange"
        @row-click="handleRowClick">
        <el-table-column type='selection' />
    {%- for b in ht.billFormFields %}
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
      <el-table v-if="selectionTypeRef !== 'checkbox'" ref="tableRef" :data="tableDataRef" :border="true"
        style="width: 100%" highlight-current-row @current-change="handleCurrentRowChange" @row-click="handleRowClick">
    {%- for b in ht.billFormFields %}
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
    </div>
    <div :style="{ alignSelf: 'end' }">
      <el-pagination v-model:current-page="pageInfoRef.pageIndex"
        v-model:page-size="pageInfoRef.pageSize" :page-sizes="[10, 20, 50, 100]" :small="true"
        layout="total, sizes, prev, pager, next, jumper" :total="pageInfoRef.totalCount"
        @size-change="handleSizeChange" @current-change="handleCurrentChange" />
    </div>
    <SubTableLayout />
  </div>
</template>
  
<script lang="ts" setup>
const props = defineProps<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}>();
import { onMounted, onBeforeUnmount, ref, computed, } from 'vue'
import SubTableLayout from './SubTableLayout.vue'
import { T{{ ht.tabClassName }}, } from '../../../models';
import { Observer, TMessage } from '~/util/observer';
import { subject, treeConf } from '../../../conf';
import { useIdUiConf,  useSelectRowKeys, useTableData,usePageInfo, } from './hooks';
import { useTableStore } from './store';
const { store, ...actions } = useTableStore()
const selectionTypeRef = ref<'checkbox' | 'radio'>('radio');
const tableRefCheckbox = ref()
const tableRef = ref()

const idUiConfRef = computed(() => useIdUiConf())
const tableDataRef = computed(() => useTableData())
const selectedRowKeys = computed(() => useSelectRowKeys())
const pageInfoRef = computed(() => usePageInfo())

const observers: Observer[] = [];

const handleSizeChange = (pageSize: number) => {
}

const handleCurrentChange = (currentPage: number) => {
}

const handleCurrentRowChange = (record: T{{ ht.tabClassName }}) => {
  if (selectionTypeRef.value === 'radio') {
  }
}

const handleRowClick = (row: T{{ ht.tabClassName }}, column: any,) => {
  const tableData = store.tableData
  if (selectionTypeRef.value === 'radio') {
    subject.publish({
      topic: 'selectRow',
      data: row,
      producerId: props.idLayout,
    });
    const rowKeys = [row.{{ ht.mainProperty }}!]
    actions.setSelectedRowKeys(rowKeys)
    tableRef.value.setCurrentRow(row)
    return;
  }
  const findRowKey = selectedRowKeys.value.find(k => k === row.{{ ht.mainProperty }})
  if (findRowKey) {
    const rowKeys = selectedRowKeys.value.filter(k => k !== row.{{ ht.mainProperty }})
    actions.setSelectedRowKeys(rowKeys)

    tableRefCheckbox.value.clearSelection();
    const rows = tableData?.filter(t => rowKeys.includes(t.{{ ht.mainProperty }}!)) ?? []
    rows.forEach(r => tableRefCheckbox.value.toggleRowSelection(r))
  } else {
    const rowKeys = [...selectedRowKeys.value, row.{{ ht.mainProperty }}!]
    actions.setSelectedRowKeys(rowKeys)

    tableRefCheckbox.value.clearSelection();
    const rows = tableData?.filter(t => rowKeys.includes(t.{{ ht.mainProperty }}!)) ?? []
    rows.forEach(r => tableRefCheckbox.value.toggleRowSelection(r))
  }
}

onMounted(() => {
  actions.setComponentInfo({
    idUiConf: props.idLayout,
    fgDisabled: props.fgDisabled,
  })
  if (!idUiConfRef.value) {
    return;
  }
  if (!treeConf) {
    actions.refleshThunks();
  }
  const treeNodeObserver: Observer = {
    topic: 'treeNodeSelected',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.fetchByTreeNodeThunks(message)
      })();
    },
  };
  subject.subscribe(treeNodeObserver);

  const treeNodeCancelObserver: Observer = {
    topic: 'treeSelectCancel',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.setSelectedTreeNode(undefined)
      })();
    },
  };
  subject.subscribe(treeNodeCancelObserver);

  const searchObserver: Observer = {
    topic: 'search',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.searchThunks(message);
      })();
    },
  };
  subject.subscribe(searchObserver);

  const checkboxObserver: Observer = {
    topic: 'checkbox',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        console.log('checkbox')
        actions.setSelectedRowKeys([])
        selectionTypeRef.value = 'checkbox'
      })();
    },
  };
  subject.subscribe(checkboxObserver);

  const radioObserver: Observer = {
    topic: 'radio',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.setSelectedRowKeys([])
        selectionTypeRef.value = 'radio'
      })();
    },
  };
  subject.subscribe(radioObserver);

  const addSuccessObserver: Observer = {
    topic: 'addSuccess',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        const addAfterData = message.data;
        actions.addNewRecords(addAfterData)
        selectionTypeRef.value = 'radio'
      })();
    },
  };
  subject.subscribe(addSuccessObserver);

  const updateSuccessObserver: Observer = {
    topic: 'updateSuccess',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        const updateAfterData = message.data;
        actions.updateRecord(updateAfterData)
        selectionTypeRef.value = 'radio'
      })();
    },
  };
  subject.subscribe(updateSuccessObserver);

  const deletesObserver: Observer = {
    topic: 'deletes',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.batchRemoveThunks(message)
      })();
    },
  };
  subject.subscribe(deletesObserver);

  const refleshObserver: Observer = {
    topic: 'reflesh',
    consumerId: idUiConfRef.value,
    update: function (message: TMessage): void {
      (async () => {
        if (!message || message.consumerIds.includes(idUiConfRef.value)) {
          return;
        }
        actions.refleshThunks()
      })();
    },
  };
  subject.subscribe(refleshObserver);

  observers.push(treeNodeObserver);
  observers.push(searchObserver);
  observers.push(checkboxObserver);
  observers.push(radioObserver);
  observers.push(addSuccessObserver);
  observers.push(updateSuccessObserver);
  observers.push(deletesObserver);
  observers.push(refleshObserver);
})

onBeforeUnmount(() => {
  observers.forEach(observer => subject.unsubsribe(observer))
})

</script>
  {%- endfor %}
{%- endif %}
