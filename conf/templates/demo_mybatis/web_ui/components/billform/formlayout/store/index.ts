import { ref } from 'vue';
import { initialState } from './initial-state';
import { TFormStore } from '../models';

import { defineStore } from 'pinia'
import buildActions from './actions';
import buildItemActions from './items';
import buildAsyncThunks from './async-thunk';
import { componentName } from '../conf';
import { moduleName } from '../../../../conf';

export const useFormStore = defineStore(moduleName + '_' + componentName, () => {
  const store = ref<TFormStore>(initialState)

  // const setComponentInfo = (params: { idUiConf: string; fgDisabled: boolean }) => {
  //   const { idUiConf, fgDisabled } = params;
  //   store.value.idUiConf = idUiConf;
  //   store.value.fgDisabled = fgDisabled;
  // }

  const actions = buildActions(store)
  const itemActions = buildItemActions(store)

  const thunks = buildAsyncThunks(store)

  return { store, ...actions, ...itemActions, ...thunks, }
})