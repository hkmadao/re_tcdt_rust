import { Ref, } from "vue";
import { TLeftTreeStore } from "../models";
import { treeConf, } from "../../../conf";
import BaseAPI from "~/api";
import { arrToTree } from "~/util";

const buildAsyncThunks = (store: Ref<TLeftTreeStore>) => {

  const fetchTreeThunks = () => {
    fetchTree().then((tree) => {
      const treeData = arrToTree(
        treeConf?.firstTreeRef?.parentIdAttr ?? 'idParent',
        treeConf?.firstTreeRef?.keyAttr ?? 'id',
        treeConf?.firstTreeRef?.labelAttr ?? 'displayName',
        tree,
        true,
        null,
      );
      store.value.sourchTreeData = treeData;
      store.value.treeData = treeData;
      if (
        treeData &&
        treeData.length > 0 &&
        store.value.expandedKeys.length === 0
      ) {
        store.value.expandedKeys = [treeData[0].key];
      }
    }).catch(err => {
      console.log(err)
    })
  }

  return { fetchTreeThunks, }
}

export default buildAsyncThunks;

const fetchTree = async () => {
  const tree: any = await BaseAPI.POST(`${treeConf?.firstTreeRef?.uri!}`, {});
  return tree;
};