<template>
  <div class="flex">
    <el-button type="primary" :icon="Plus" @click="handleSave">保存</el-button>
    <el-button v-if="fgAddRef" type="primary" :icon="Edit" @click="handleAddAgain">保存并新增</el-button>
    <el-button type="primary" @click="handleReflesh">刷新</el-button>
    <el-button type="primary" @click="handleCancel">返回</el-button>
  </div>
</template>
<script setup lang="ts">
const props = defineProps<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}>();
import { Plus, Edit, } from '@element-plus/icons-vue'
import { ref, onMounted, onBeforeUnmount } from 'vue';
import { Observer, TMessage } from '~/util/observer';
import { subject } from '../../conf';

const compontId = 'formToolBar';
const fgAddRef = ref<boolean>(true);
const observers: Observer[] = [];
onMounted(() => {
  const toAddObserver: Observer = {
    topic: 'toAdd',
    consumerId: compontId,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(compontId)) {
        return;
      }
      fgAddRef.value = true;
    },
  };
  subject.subscribe(toAddObserver);
  observers.push(toAddObserver)

  const toEditObserver: Observer = {
    topic: 'toEdit',
    consumerId: compontId,
    update: function (message: TMessage): void {
      if (message.consumerIds.includes(compontId)) {
        return;
      }
      fgAddRef.value = false;
    },
  };
  subject.subscribe(toEditObserver);
  observers.push(toEditObserver)

});

onBeforeUnmount(() => {
  //销毁观察者
  observers.forEach(observer => subject.unsubsribe(observer))
})

const handleSave = () => {
  if (fgAddRef.value) {
    subject.publish({
      topic: 'add',
      data: undefined,
      producerId: props.idLayout,
    });
  } else {
    subject.publish({
      topic: 'edit',
      data: undefined,
      producerId: props.idLayout,
    });
  }
};

const handleAddAgain = () => {
  subject.publish({
    topic: 'addAgain',
    data: undefined,
    producerId: props.idLayout,
  });
};

const handleCancel = () => {
  subject.publish({
    topic: 'cancel',
    data: undefined,
    producerId: props.idLayout,
  });
};

const handleReflesh = () => {
  subject.publish({
    topic: 'detailReflesh',
    data: undefined,
    producerId: props.idLayout,
  });
};

</script>