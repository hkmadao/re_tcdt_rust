<template>
  <div :id="id" :style="{
    display: 'flex',
    flexDirection: direction,
    margin: '5px',
    flex: flexStr ?? '1 1 auto',
    cursor: asso && asso.disabled ? 'not-allowed' : undefined,
    pointerEvents: asso && asso.disabled ? 'none' : undefined,
    opacity: asso && asso.disabled ? '0.5' : undefined,
  }">
    <slot></slot>
  </div>
</template>

<script lang="ts" setup>
const props = defineProps<{ id: string; flexStr: string; direction: 'row' | 'column'; }>();
import { computed } from 'vue';
import { layoutConf } from '../conf';
import { useAsso } from '../hooks';
const direction = props.direction;
const flexStr = props.flexStr;
const id = props.id;
const asso = computed(() => useAsso(id, layoutConf))
</script>