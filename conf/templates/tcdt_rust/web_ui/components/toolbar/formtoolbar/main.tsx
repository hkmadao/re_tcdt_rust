import { FC, useEffect, } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import FormToolBarComponent from './components';
import { actions, } from './store';

const FormToolBar: FC<{
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
      <FormToolBarComponent />
    </>
  );
};

export default FormToolBar;
