import { TLayout, TUiFactoryContent } from '~/models';
import { getAsso } from '~/util';
import { useModuleStore } from '../store';

const { store } = useModuleStore()

export const useIdUiConf = () => {
  return store?.idUiConf;
};

export const useLoadingStatus = () => {
  return store?.status;
};

export const usePageCode = () => {
  return store?.pageCode;
};

export const useAsso = (idLayout: string, layoutConf: TUiFactoryContent) => {
  if (!store?.pageCode) {
    return;
  }
  const asso = getAsso(store?.pageCode, idLayout, layoutConf);
  return asso;
};

export const findVisableLayouts = (layouts: TLayout[], layoutConf: TUiFactoryContent) => {
  const layoutsCurrent = layouts.filter(layout => {
    const asso = getAsso(store.pageCode!, layout.id, layoutConf);
    if (!asso || asso.hidden) {
      return false;
    }
    return true;
  }).map(layout => {
    const asso = getAsso(store.pageCode!, layout.id, layoutConf);
    return { ...layout, fgDisable: !!asso?.disabled }
  });
  return layoutsCurrent;
}
