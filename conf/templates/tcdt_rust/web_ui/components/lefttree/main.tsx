import { FC, useEffect, } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import LeftTreeLayout from './components';
import { actions, } from './store';

const LeftTree: FC<{
  idLayout: string
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
}> = ({ idLayout, fgDisabled }) => {
  const dispatch = useDispatch();

  useEffect(() => {
    dispatch(actions.setComponentInfo({ idUiConf: idLayout, fgDisabled }))
  }, [idLayout, fgDisabled]);

  return (
    <>
      <LeftTreeLayout />
    </>
  );
};

export default LeftTree;
