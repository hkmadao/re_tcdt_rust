import { useSelector, } from 'react-redux';
import { TFormToolbarStore } from '../models';
import { componentName } from '../conf';

const selectStatus = (state: { [x: string]: TFormToolbarStore }) => {
  return state[componentName].status;
};

const selectIdUiConf = (state: { [x: string]: TFormToolbarStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useFgDisabled = () => {
  return useSelector((state: { [x: string]: TFormToolbarStore }) => {
    return state[componentName].fgDisabled;
  });
}

export const useFgAdd = () => {
  return useSelector((state: { [x: string]: TFormToolbarStore }) => {
    return state[componentName].fgAdd;
  });
}