import { TActionContent, } from '~/models';

const actionTableConf: TActionContent = {%- if rootInfo.vButtonContent %}{{ rootInfo.vButtonContent }}{%- else %}undefined{%- endif %};

export { actionTableConf }