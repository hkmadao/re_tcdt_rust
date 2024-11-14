import { Ref } from "vue"
import { TDomainStore } from "./models"

const buildActions = (store: Ref<TDomainStore>) => {
  const changePageStatus = (pageCode: string) => {
    store.value.pageCode = pageCode
  }
  return { changePageStatus }
}

export default buildActions;