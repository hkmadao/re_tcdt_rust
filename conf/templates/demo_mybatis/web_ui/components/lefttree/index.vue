<template>
  <LeftTree />
</template>

<script lang="ts" setup>
import { onMounted } from "vue";
import LeftTree from "./components/index.vue";
import { useLeftTreeStore } from './store';

const props = defineProps<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}>();
const { store, ...actions } = useLeftTreeStore()

onMounted(() => {
  actions.setComponentInfo({
    idUiConf: props.idLayout,
    fgDisabled: props.fgDisabled,
  })
})

</script>