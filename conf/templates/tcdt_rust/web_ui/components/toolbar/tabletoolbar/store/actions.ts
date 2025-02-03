import { CaseReducer, PayloadAction } from "@reduxjs/toolkit";
import { TTableToolBarStore } from "../models";
import { TTree } from "@/models";
import { Key } from "react";
import { subject, treeConf } from "../../../../conf";
import { deepCopy, getMatchKeys, getTreeByKeys, getTreeKeys, } from "@/util";

export const setComponentInfo: CaseReducer<
  TFormToolbarStore,
  PayloadAction<{ idUiConf: string; fgDisabled: boolean; fgHidden: boolean; }>
> = (state, action) => {
  const { idUiConf, fgDisabled, fgHidden } = action.payload;
  state.idUiConf = idUiConf;
  state.fgDisabled = fgDisabled;
  state.fgHidden = fgHidden;
};

export const setTreeNodeData: CaseReducer<
  TTableToolBarStore,
  PayloadAction<TTree>
> = (state, action) => {
  const treeNodeData = action.payload;
  state.nodeTreeData = treeNodeData;
};

export const cancelTreeNodeData: CaseReducer<
  TTableToolBarStore,
  PayloadAction<void>
> = (state, action) => {
  state.nodeTreeData = undefined;
};

export const setSelectRows: CaseReducer<
  TTableToolBarStore,
  PayloadAction<any[]>
> = (state, action) => {
  const selectRows = action.payload;
  state.selectRows = selectRows;
};

export const setRowSelectionType: CaseReducer<
  TTableToolBarStore,
  PayloadAction<'checkbox' | 'radio'>
> = (state, action) => {
  const rowSelectionType = action.payload;
  state.rowSelectionType = rowSelectionType;
  state.selectRows = [];
};

