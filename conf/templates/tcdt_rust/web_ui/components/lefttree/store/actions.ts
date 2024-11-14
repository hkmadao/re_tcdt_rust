import { CaseReducer, PayloadAction } from "@reduxjs/toolkit";
import { TLeftTreeStore } from "../models";
import { TTree } from "@/models";
import { Key } from "react";
import { subject, treeConf } from "../../../conf";
import { deepCopy, getMatchKeys, getTreeByKeys, getTreeKeys, } from "@/util";

export const setComponentInfo: CaseReducer<
  TLeftTreeStore,
  PayloadAction<{ idUiConf: string; fgDisabled: boolean; }>
> = (state, action) => {
  const { idUiConf, fgDisabled } = action.payload;
  state.idUiConf = idUiConf;
  state.fgDisabled = fgDisabled;
};

export const setSelectedNode: CaseReducer<
  TLeftTreeStore,
  PayloadAction<{ keys: Key[], node: TTree }>
> = (state, action) => {
  const { keys, node } = action.payload;
  state.selectedNode = node;
  state.selectedKeys = keys;
  subject.publish({
    topic: 'treeNodeSelected',
    producerId: state.idUiConf!,
    data: deepCopy(node),
  });
};

export const cancelSelectedNode: CaseReducer<
  TLeftTreeStore,
  PayloadAction<void>
> = (state, action) => {
  state.selectedKeys = [];
  state.selectedNode = undefined;
  subject.publish({
    topic: 'treeSelectCancel',
    producerId: state.idUiConf!,
    data: undefined,
  });
};

export const toggleExpand: CaseReducer<
  TLeftTreeStore,
  PayloadAction<Key>
> = (state, action) => {
  if (state.expandedKeys.includes(action.payload)) {
    state.expandedKeys = state.expandedKeys.filter((k) => k !== action.payload);
    return;
  }
  state.expandedKeys = state.expandedKeys.concat([action.payload]);
};

export const setExpandedKeys: CaseReducer<
  TLeftTreeStore,
  PayloadAction<Key[]>
> = (state, action) => {
  state.expandedKeys = action.payload;
};

export const searchTreeNode: CaseReducer<
  TLeftTreeStore,
  PayloadAction<string>
> = (state, action) => {
  const searchValue = action.payload;
  const foundKeys = getMatchKeys(treeConf?.searchAttrs || [], searchValue, state.sourchTreeData || []);
  const foundTree = getTreeByKeys(foundKeys, state.sourchTreeData || []);
  state.expandedKeys = getTreeKeys(foundTree);
  state.foundKeys = foundKeys;
  state.treeData = foundTree;
};