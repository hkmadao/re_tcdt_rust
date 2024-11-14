import { ref } from 'vue';
import { initialState } from './initial-state';
import { TDomainStore } from './models';
import { defineStore } from 'pinia'
import buildActions from './actions';
import { componentName, moduleName } from '../conf';

export const useModuleStore = defineStore(moduleName + '_' + componentName, () => {
  const store = ref<TDomainStore>(initialState)

  // const changePageStatus = (pageCode: string) => {
  //   store.value.pageCode = pageCode
  // }
  const actions = buildActions(store)
  return { store, ...actions }
})
