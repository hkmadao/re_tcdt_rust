import { createAsyncThunk } from '@reduxjs/toolkit';
import BaseAPI from '@/api';
import { treeConf, } from '../../../conf';

export const fetchTree = createAsyncThunk(
  `${treeConf?.name}/fetchTree`,
  async (param: void, thunkAPI) => {
    const tree: any = await BaseAPI.POST(`${treeConf?.firstTreeRef?.uri!}`, {})
    return tree;
  },
);

export const remove = createAsyncThunk(
  `${treeConf?.name}/remove`,
  async (param: any, thunkAPI) => {
    await BaseAPI.POST(`/menu/remove`, param)
    const tree: any = await BaseAPI.POST(`${treeConf?.firstTreeRef?.uri!}`, {})
    return tree;
  },
);