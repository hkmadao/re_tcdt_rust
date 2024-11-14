import { TActionContent, } from '@/models';

const actionFormConf: TActionContent | undefined = {% if rootInfo.buttonContent %}{{ rootInfo.buttonContent }}{%- else %}undefined{%- endif %};

export { actionFormConf }