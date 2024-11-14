import { TUiFactoryContent } from '@/models';

const layoutConf: TUiFactoryContent = {% if rootInfo.uiContent %}{{ rootInfo.uiContent }}{%- else %}undefined{%- endif %};

export { layoutConf }