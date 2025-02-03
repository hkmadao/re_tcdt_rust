import { FC, useEffect, } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import LeftTreeLayout from './components';
import { actions, } from './store';

const LeftTree: FC<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  fgHidden: boolean;
}> = ({ idLayout, fgDisabled, fgHidden }) => {
  const dispatch = useDispatch();

  useEffect(() => {
    dispatch(
      actions.setComponentInfo({ idUiConf: idLayout, fgDisabled, fgHidden }),
    );
  }, [idLayout, fgDisabled, fgHidden]);

  return (
    <>
      <LeftTreeLayout />
    </>
  );
};

export default LeftTree;
