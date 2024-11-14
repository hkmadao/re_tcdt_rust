import { useFormStore } from '../store';

const { store } = useFormStore()

export const useEditStatusInfo = () => {
  return store.editStatusInfo;
};

export const useIdUiConf = () => {
  return store.idUiConf!
};


export const useFgDisabled = () => {
  return store.fgDisabled
};

export const useStoreData = () => {
  return store
};

export const useFormData = () => {
  return store.formData
};
