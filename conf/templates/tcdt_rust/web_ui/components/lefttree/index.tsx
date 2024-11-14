import { FC, useEffect, useState, } from 'react';
import { Provider, } from 'react-redux';
import MainLayout from './main';
import store from './store';

const TableLayout: FC<{
  idLayout: string
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}> = ({ idLayout, fgDisabled }) => {
  const [myIdLayout, setMyIdLayout] = useState(idLayout)
  const [myFgDisabled, setMyFgDisabled] = useState(fgDisabled)

  useEffect(() => {
    setMyIdLayout(idLayout)
    setMyFgDisabled(fgDisabled)
  }, [idLayout, fgDisabled]);

  return (
    <>
      <Provider store={store}>
      <MainLayout idLayout={myIdLayout} fgDisabled={myFgDisabled} />
      </Provider>
    </>
  );
};

export default TableLayout;
