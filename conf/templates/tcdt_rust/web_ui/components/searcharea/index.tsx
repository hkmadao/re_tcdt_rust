import { FC, useEffect, useState } from 'react';
import { Provider } from 'react-redux';
import MainLayout from './main';
import store from './store';

const SearchAreaLayout: FC<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  fgHidden: boolean;
}> = ({ idLayout, fgDisabled, fgHidden }) => {
  const [myIdLayout, setMyIdLayout] = useState(idLayout);
  const [myFgDisabled, setMyFgDisabled] = useState(fgDisabled);
  const [myHidden, setMyHidden] = useState(fgHidden);

  useEffect(() => {
    setMyIdLayout(idLayout);
    setMyFgDisabled(fgDisabled);
    setMyHidden(fgHidden);
  }, [idLayout, fgDisabled, fgHidden]);

  return (
    <>
      <Provider store={store}>
        <MainLayout
          idLayout={myIdLayout}
          fgDisabled={myFgDisabled}
          fgHidden={myHidden}
        />
      </Provider>
    </>
  );
};

export default SearchAreaLayout;
