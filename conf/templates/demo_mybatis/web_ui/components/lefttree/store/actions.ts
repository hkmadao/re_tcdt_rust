import { Ref } from "vue";
import { TLeftTreeStore } from "../models";
import { subject, treeConf } from "../../../conf";
import { deepCopy, getMatchKeys, getTreeByKeys, getTreeKeys } from "~/util";
import { TTree } from "~/models";

const buildActions = (store: Ref<TLeftTreeStore>) => {
  const setComponentInfo = (params: { idUiConf: string; fgDisabled: boolean }) => {
    const { idUiConf, fgDisabled } = params;
    store.value.idUiConf = idUiConf;
    store.value.fgDisabled = fgDisabled;
  }

  const setSelectedNode = (params: { keys: string[]; node: TTree }) => {
    const { keys, node } = params;
    store.value.selectedNode = node;
    store.value.selectedKeys = keys;
    subject.publish({
      topic: 'treeNodeSelected',
      producerId: store.value.idUiConf!,
      data: deepCopy(node),
    });
  }

  const cancelSelectedNode = () => {
    store.value.selectedKeys = [];
    store.value.selectedNode = undefined;
    subject.publish({
      topic: 'treeSelectCancel',
      producerId: store.value.idUiConf!,
      data: undefined,
    });
  }

  const toggleExpand = (key: string) => {
    if (store.value.expandedKeys.includes(key)) {
      store.value.expandedKeys = store.value.expandedKeys.filter((k) => k !== key);
      return;
    }
    store.value.expandedKeys = store.value.expandedKeys.concat([key]);
  }

  const setExpandedKeys = (keys: string[]) => {
    store.value.expandedKeys = keys;
  }

  const searchTreeNode = (keys: string) => {
    const searchValue = keys;
    const foundKeys = getMatchKeys(
      treeConf.searchAttrs,
      searchValue,
      store.value.sourchTreeData || [],
    );
    const foundTree = getTreeByKeys(foundKeys, store.value.sourchTreeData || []);
    store.value.expandedKeys = getTreeKeys(foundTree);
    store.value.foundKeys = foundKeys;
    store.value.treeData = foundTree;
  }

  return { setComponentInfo, setSelectedNode, cancelSelectedNode, toggleExpand, setExpandedKeys, searchTreeNode, }
}

export default buildActions;