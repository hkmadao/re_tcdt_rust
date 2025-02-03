import { CaseReducer, PayloadAction } from "@reduxjs/toolkit";
import { TFormToolbarStore } from "../models";
import { TTree } from "@/models";
import { Key } from "react";
import { subject, } from "../../../../conf";

export const setComponentInfo: CaseReducer<
  TFormToolbarStore,
  PayloadAction<{ idUiConf: string; fgDisabled: boolean; fgHidden: boolean; }>
> = (state, action) => {
  const { idUiConf, fgDisabled, fgHidden } = action.payload;
  state.idUiConf = idUiConf;
  state.fgDisabled = fgDisabled;
  state.fgHidden = fgHidden;
};

export const setFgAdd: CaseReducer<
  TFormToolbarStore,
  PayloadAction<boolean>
> = (state, action) => {
  const fgAdd = action.payload;
  state.fgAdd = fgAdd;
};