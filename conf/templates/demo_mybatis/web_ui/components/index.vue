<template>
    <div :style="{
      display: 'flex',
      flex: '1 1 auto',
      margin: '5px 0px 5px 0px',
    }">
      <Main :layouts="layoutsRef" />
    </div>
  </template>
  
  <script lang="ts" setup>
  import { computed, onMounted, onUnmounted, ref, watch } from "vue";
  import { layoutConf, subject, } from "../conf";
  import Main from "./main.vue";
  import { Observer, TMessage, getLayoutById } from "~/util";
  import { useModuleStore } from '../store';
  import { findVisableLayouts, useIdUiConf } from "../hooks";
  const pageObserverRef = ref<Observer>()
  const { changePageStatus, } = useModuleStore()
  
  const layoutsRef = computed(() => findVisableLayouts(layoutConf.layouts, layoutConf))
  const idUiConf = computed(() => useIdUiConf())
  
  onMounted(() => {
    const pageObserver: Observer = {
      topic: '/page/change',
      consumerId: idUiConf.value,
      update: function (message: TMessage): void {
        if (message.producerId) {
          const layout = getLayoutById(message.producerId, layoutConf);
          const pageMaps = layout?.pageMaps ?? [];
          const pageMap = pageMaps.find(
            (p) => p.componentStateCode === message.data,
          );
          if (pageMap && pageMap.pageCode) {
            changePageStatus(pageMap.pageCode);
          }
        }
      },
    };
    subject.subscribe(pageObserver);
    pageObserverRef.value = pageObserver
  })
  
  onUnmounted(() => {
    if (pageObserverRef.value) {
      subject.unsubsribe(pageObserverRef.value)
    }
  })
  
  </script>