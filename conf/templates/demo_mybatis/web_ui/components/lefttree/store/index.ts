import { ref } from 'vue';
import { initialState } from './initial-state';
import { TLeftTreeStore } from '../models';

import { defineStore } from 'pinia'
import buildActions from './actions';
import buildAsyncThunks from './async-thunk';
import { componentName } from '../conf';
import { moduleName } from '../../../conf';

export const useLeftTreeStore = defineStore(moduleName + '_' + componentName, () => {
  const store = ref<TLeftTreeStore>(initialState)

  // const setComponentInfo = (params: { idUiConf: string; fgDisabled: boolean }) => {
  //   const { idUiConf, fgDisabled } = params;
  //   store.value.idUiConf = idUiConf;
  //   store.value.fgDisabled = fgDisabled;
  // }

  const actions = buildActions(store)

  const thunks = buildAsyncThunks(store)

  return { store, ...actions, ...thunks, }
})