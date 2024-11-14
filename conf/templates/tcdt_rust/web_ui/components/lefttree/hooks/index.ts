import { useSelector, } from 'react-redux';
import { getSelectedNodes } from '@/util';
import { TLeftTreeStore } from '../models';
import { componentName } from '../conf';

const selectStatus = (state: { [x: string]: TLeftTreeStore }) => {
  return state[componentName].status;
};

const selectTreeData = (state: { [x: string]: TLeftTreeStore }) => {
  return state[componentName].treeData;
};

const selectSelectedNode = (state: { [x: string]: TLeftTreeStore }) => {
  if (state[componentName].selectedKeys) {
    const selectNodes = getSelectedNodes(state[componentName].selectedKeys, state[componentName].treeData ?? []);
    if (selectNodes && selectNodes.length === 1) {
      return selectNodes[0];
    }
  }
  return undefined;
};

const selectIdUiConf = (state: { [x: string]: TLeftTreeStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useFgDisabled = () => {
  return useSelector((state: { [x: string]: TLeftTreeStore }) => {
    return state[componentName].fgDisabled;
  });
}

export const useSelectedNode = () => {
  return useSelector(selectSelectedNode);
}

export const useLoadingStatus = () => {
  return useSelector(selectStatus);
}

export const useTreeData = () => {
  return useSelector(selectTreeData);
}

export const useSelectedKeys = () => {
  return useSelector((state: { [x: string]: TLeftTreeStore }) => {
    return state[componentName].selectedKeys;
  });
}

export const useExpandedKeys = () => {
  return useSelector((state: { [x: string]: TLeftTreeStore }) => {
    return state[componentName].expandedKeys;
  });
}

export const useFoundKeys = () => {
  return useSelector((state: { [x: string]: TLeftTreeStore }) => {
    return state[componentName].foundKeys;
  });
}