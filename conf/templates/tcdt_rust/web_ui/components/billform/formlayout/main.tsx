import { FC, useEffect } from 'react';
import { Tabs } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { 
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
    {{ bt.firstUpperTabCode }},
    {%- endif %}
  {%- endfor %}
{%- endif %}
} from './components/SubForm';
import MainFormLayout from './components/MainFormLayout';
import { actions, } from './store';

const FormLayout: FC<{
  idLayout: string;
  /**组件是否是禁用状态 */
  fgDisabled: boolean;
  fgHidden: boolean;
}> = ({ idLayout, fgDisabled, fgHidden }) => {
  const { TabPane } = Tabs;
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
        }}
{%- endraw %}
      >
        <div 
{%- raw %}
          style={{ flex: 'auto', }}
{%- endraw %}
        >
          <MainFormLayout />
        </div>
        <div 
{%- raw %}
          style={{ flex: 'auto', }}
{%- endraw %}
        >
{%- if rootInfo.bJson and rootInfo.bJson.configForm.body is iterable %}
        <Tabs defaultActiveKey={ 
  {%- if rootInfo.bJson.configForm.body | length > 0 -%}
    {%- for bt in rootInfo.bJson.configForm.body -%}
      {%- if bt.fgDefaultTab -%}
'{{ bt.tabCode }}' 
      {%- endif -%}
    {%- endfor -%}
  {%- else -%}
''
  {%- endif -%}
  }>
  {%- for bt in rootInfo.bJson.configForm.body %}
    {%- if bt.billFormFields is iterable %}
            <TabPane
              key={'{{ bt.tabCode }}'}
              tabKey={'{{ bt.tabCode }}'}
              tab={'{{ bt.tabName }}'}
            >
              <{{ bt.firstUpperTabCode }} />
            </TabPane>
    {%- endif %}
  {%- endfor %}
         </Tabs>
{%- endif %}
        </div>
      </div>
    </>
  );
};

export default FormLayout;
