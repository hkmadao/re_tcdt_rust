import { useSelector, } from 'react-redux';
import { TUiFactoryContent } from '@/models';
import { getAsso } from '@/util';
import { TDomainStore } from '../store/slice/models';
import { componentName } from '../conf';

const selectStatus = (state: { [x: string]: TDomainStore }) => {
  return state[componentName].status;
};

const selectPageCode = (state: { [x: string]: TDomainStore }) => {
  return state[componentName].pageCode;
};

const selectIdUiConf = (state: { [x: string]: TDomainStore }) => {
  return state[componentName].idUiConf;
};

export const useIdUiConf = () => {
  return useSelector(selectIdUiConf);
}

export const useLoadingStatus = () => {
  return useSelector(selectStatus);
}

export const usePageCode = () => {
  return useSelector(selectPageCode);
}

export const useAsso = (idLayout: string,
  layoutConf: TUiFactoryContent) => {
  return useSelector((state: { [x: string]: TDomainStore }) => {
    if (!state[componentName].pageCode) {
      return;
    }
    const asso = getAsso(state[componentName].pageCode, idLayout, layoutConf);
    return asso;
  });
}