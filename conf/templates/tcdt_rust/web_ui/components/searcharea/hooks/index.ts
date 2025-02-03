import { useSelector, } from 'react-redux';
import { TSearchAreaStore } from '../models';
import { componentName } from '../conf';

const selectStatus = (state: { [x: string]: TSearchAreaStore }) => {
  return state[componentName].status;
};

const selectIdUiConf = (state: { [x: string]: TSearchAreaStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useFgDisabled = () => {
  return useSelector((state: { [x: string]: TSearchAreaStore }) => {
    return state[componentName].fgDisabled;
  });
}

export const useFgHidden = () => {
  return useSelector((state: { [x: string]: TSearchAreaStore }) => {
    return state[componentName].fgHidden;
  });
}