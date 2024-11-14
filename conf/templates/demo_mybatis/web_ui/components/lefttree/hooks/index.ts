import { useLeftTreeStore } from '../store';

const { store } = useLeftTreeStore()

export const useIdUiConf = () => {
  return store.idUiConf;
};

export const useSelectedNode = () => {
  return store.selectedNode;
};

export const useLoadingStatus = () => {
  return store.status;
};

export const useTreeData = () => {
  return store.treeData;
};

export const useSelectedKeys = () => {
  return store.selectedKeys;
};

export const useExpandedKeys = () => {
  return store.expandedKeys;
};

export const useFoundKeys = () => {
  return store.foundKeys;
};
