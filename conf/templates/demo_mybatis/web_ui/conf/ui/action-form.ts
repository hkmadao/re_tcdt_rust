import { TActionContent, } from '~/models';

const actionFormConf: TActionContent = {%- if rootInfo.buttonContent %}{{ rootInfo.buttonContent }}{%- else %}undefined{%- endif %};

export { actionFormConf }