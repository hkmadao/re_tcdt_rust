<template>
  <Layout v-for="l in layouts" :key="l.id" :id="l.id" :flex-str="l.flexStr" :direction="l.direction">
    <SelfComponent v-if="l.children && l.children.length > 0" :layouts="l.children" />
    <FormLayout v-if="l.component && l.component.componentType === 'editBillform'" :id-layout="l.id"
      :fg-disabled="l.fgDisable" />
    <TableLayout v-if="l.component && l.component.componentType === 'viewBillform'" :id-layout="l.id"
      :fg-disabled="l.fgDisable" />
    <FormToolBar v-if="l.component && l.component.componentType === 'editButton'" :id-layout="l.id"
      :fg-disabled="l.fgDisable" />
    <TableToolBar v-if="l.component && l.component.componentType === 'viewButton'" :id-layout="l.id"
      :fg-disabled="l.fgDisable" />
    <SearchArea v-if="l.component && l.component.componentType === 'search'" :id-layout="l.id"
      :fg-disabled="l.fgDisable" />
    <LeftTree v-if="l.component && l.component.componentType === 'tree'" :id-layout="l.id" :fg-disabled="l.fgDisable" />
    <div v-if="l.component && l.component.componentType === 'custom'">自定义组件</div>
  </Layout>
</template>

<script lang="ts" setup>
import { TLayout } from "~/models";
const props = defineProps<{ layouts: TLayout[]; }>();

import FormLayout from "./billform/formlayout/index.vue";
import TableLayout from "./billform/tablelayout/index.vue";
import FormToolBar from "./toolbar/FormToolBar.vue";
import TableToolBar from "./toolbar/TableToolBar.vue";
import SearchArea from './searcharea/index.vue';
import LeftTree from "./lefttree/index.vue";
import Layout from "./layout.vue";
import SelfComponent from "./main.vue";
import { computed, } from "vue";
import { findVisableLayouts, } from "../hooks";
import { layoutConf } from "../conf";

const layouts = computed(() => { return findVisableLayouts(props.layouts, layoutConf); })

</script>