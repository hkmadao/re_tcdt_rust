import { FC, memo, useEffect, } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import SubTableLayout from './components/SubTableLayout';
import MainTableLayout from './components/MainTableLayout';
import { actions, } from './store';

const TableLayout: FC<{
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
      <div
{%- raw %}
        style={{
          display: 'flex',
          flex: 'auto',
          flexDirection: 'column',
          backgroundColor: 'white',
          overflow: 'auto',
        }}
{%- endraw %}
      >
        <div 
{%- raw %}
          style={{ flex: 'auto', overflow: 'auto', }}
{%- endraw %}
        >
          <MainTableLayout />
        </div>
        <div 
{%- raw %}
          style={{ flex: 'auto', overflow: 'auto', }}
{%- endraw %}
        >
          <SubTableLayout />
        </div>
      </div>
    </>
  );
};

export default TableLayout;
