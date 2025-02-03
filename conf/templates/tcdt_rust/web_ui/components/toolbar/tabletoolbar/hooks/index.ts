import { useSelector, } from 'react-redux';
import { TTableToolBarStore } from '../models';
import { componentName } from '../conf';

const selectStatus = (state: { [x: string]: TTableToolBarStore }) => {
  return state[componentName].status;
};

const selectIdUiConf = (state: { [x: string]: TTableToolBarStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useFgDisabled = () => {
  return useSelector((state: { [x: string]: TTableToolBarStore }) => {
    return state[componentName].fgDisabled;
  });
}

export const useTreeNodeData = () => {
  return useSelector((state: { [x: string]: TTableToolBarStore }) => {
    return state[componentName].nodeTreeData;
  });
}

export const useRowSelectionType = () => {
  return useSelector((state: { [x: string]: TTableToolBarStore }) => {
    return state[componentName].rowSelectionType;
  });
}

export const useSelectRows = () => {
  return useSelector((state: { [x: string]: TTableToolBarStore }) => {
    return state[componentName].selectRows;
  });
}