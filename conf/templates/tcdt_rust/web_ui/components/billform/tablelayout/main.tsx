import { FC, memo, useEffect, } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import SubTableLayout from './SubTableLayout';
import MainTableLayout from './MainTableLayout';
import { actions, } from './store';

const TableLayout: FC<{
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
