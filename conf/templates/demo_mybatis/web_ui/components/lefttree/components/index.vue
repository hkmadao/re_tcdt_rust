<template>
  <div :style="{ display: 'flex', flex: 'auto', minWidth: 200, }">
    <div :style="{ display: 'flex', flex: 'auto', flexDirection: 'column', backgroundColor: 'white', gap: '5px', }">
      <div :style="{ display: 'flex', flex: '0 1 auto', }">
        <el-input v-model="searchValue" placeholder="Please input" />
        <el-button :icon="Search" circle />
        <el-button :icon="Refresh" circle @click="handleRefresh" />
      </div>
      <!-- <NodeAction /> -->
      <el-tree :data="treeDataRef" :props="defaultProps" :highlight-current="true" @node-click="handleNodeClick" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import {
  Search,
  Refresh,
} from '@element-plus/icons-vue'
import { TTree } from '~/models';
import { computed, onMounted, ref } from 'vue';
import { useLeftTreeStore } from '../store';
import { useTreeData } from '../hooks';

const { store, ...actions } = useLeftTreeStore()

const treeDataRef = computed(() => useTreeData())
const searchValue = ref<string>()

const defaultProps = {
  children: 'children',
  label: 'displayName',
}

const handleNodeClick = (data: TTree, _1: any, _2: any, e: MouseEvent) => {
  // console.log(data, _1, _2, e)
  actions.setSelectedNode({ keys: [data.key], node: data })
}

const handleRefresh = () => {
  actions.fetchTreeThunks()
}

onMounted(() => {
  actions.fetchTreeThunks()
})

</script>
