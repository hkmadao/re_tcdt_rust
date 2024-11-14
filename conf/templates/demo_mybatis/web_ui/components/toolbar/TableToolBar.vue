<template>
  <div class="flex">
    <el-button type="primary" :icon="Plus" @click="toAdd" />
    <el-button type="primary" :icon="Edit" @click="toEdit" :disabled="!currentRef" />
    <el-button type="primary" @click="multiSelect">{{ "{{" }} multiButtonContentRef {{ "}}" }}</el-button>
    <el-button type="primary" @click="handleToDelete" :disabled="!currentRef">删除</el-button>
  </div>
</template>
<script setup lang="ts">
const props = defineProps<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}>();
import { Plus, Edit, } from '@element-plus/icons-vue'
import { subject } from '../../conf';
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { TTree } from '~/models';
import { Observer, TMessage } from '~/util/observer';

const isModalVisibleRef = ref<boolean>(false);
const multiButtonContentRef = ref<string>('多选');
const nodeTreeDataRef = ref<TTree>();
const currentRef = ref<any>();
const currentsRef = ref<any[]>([]);
const rowSelectionTypeRef = ref<
  'checkbox' | 'radio'
>('radio');

const compontId = 'tableToolBar';

const observers: Observer[] = [];

onMounted(() => {
  const treeNodeObserver: Observer = {
    topic: 'treeNodeSelected',
    consumerId: compontId,
    update: function (message: TMessage): void {
      (async () => {
        if (!message) {
          return;
        }
        const nodeData: TTree = message?.data as TTree;
        nodeTreeDataRef.value = nodeData
      })();
    },
  };
  subject.subscribe(treeNodeObserver);
  observers.push(treeNodeObserver)

  const selectRowObserver: Observer = {
    topic: 'selectRow',
    consumerId: compontId,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(compontId)) {
        return;
      }
      currentRef.value = message.data
    },
  };
  subject.subscribe(selectRowObserver);
  observers.push(selectRowObserver)

  const selectRowsObserver: Observer = {
    topic: 'selectRows',
    consumerId: compontId,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(compontId)) {
        return;
      }
      currentsRef.value = message.data;
    },
  };
  subject.subscribe(selectRowsObserver);
  observers.push(selectRowsObserver)

  const listReloadObserver: Observer = {
    topic: 'listReload',
    consumerId: compontId,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(compontId)) {
        return;
      }
      currentRef.value = undefined;
      currentsRef.value = [];
    },
  };
  subject.subscribe(listReloadObserver);
  observers.push(listReloadObserver)

});

onBeforeUnmount(() => {
  //销毁观察者
  observers.forEach(observer => subject.unsubsribe(observer))
})

const toAdd = () => {
  subject.publish({
    topic: 'toAdd',
    producerId: props.idLayout,
    data: { treeSelectedNode: nodeTreeDataRef.value },
  });
  subject.publish({
    topic: '/page/change',
    producerId: props.idLayout,
    data: 'form',
  });
};

const toEdit = () => {
  subject.publish({
    topic: 'toEdit',
    producerId: props.idLayout,
    data: { treeSelectedNode: nodeTreeDataRef.value, selectedRow: currentRef.value },
  });
  subject.publish({
    topic: '/page/change',
    producerId: props.idLayout,
    data: 'form',
  });
};

const handleToDelete = () => {
  handleOk()
}

const multiSelect = () => {
  if (rowSelectionTypeRef.value !== 'checkbox') {
    multiButtonContentRef.value = '取消多选';
    subject.publish({
      topic: 'checkbox',
      producerId: props.idLayout,
      data: undefined,
    });
    rowSelectionTypeRef.value = 'checkbox'
    currentRef.value = undefined
    return;
  }
  subject.publish({
    topic: 'radio',
    producerId: props.idLayout,
    data: undefined,
  });
  rowSelectionTypeRef.value = 'radio'
  multiButtonContentRef.value = '多选'
  currentsRef.value = []
};

const handleOk = () => {
  subject.publish({
    topic: 'deletes',
    producerId: props.idLayout,
    data: [currentsRef.value],
  });
  isModalVisibleRef.value = false
};

const handleCancel = () => {
  isModalVisibleRef.value = false
};

const handleReflesh = () => {
  subject.publish({
    topic: 'reflesh',
    producerId: props.idLayout,
    data: undefined,
  });
};
</script>