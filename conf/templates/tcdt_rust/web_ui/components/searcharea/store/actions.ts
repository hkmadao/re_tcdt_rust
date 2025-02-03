import { CaseReducer, PayloadAction } from "@reduxjs/toolkit";
import { TSearchAreaStore } from "../models";
import { TTree } from "@/models";
import { Key } from "react";
import { subject, treeConf } from "../../../conf";

export const setComponentInfo: CaseReducer<
  TSearchAreaStore,
  PayloadAction<{ idUiConf: string; fgDisabled: boolean; fgHidden: boolean; }>
> = (state, action) => {
  const { idUiConf, fgDisabled, fgHidden } = action.payload;
  state.idUiConf = idUiConf;
  state.fgDisabled = fgDisabled;
  state.fgHidden = fgHidden;
};
