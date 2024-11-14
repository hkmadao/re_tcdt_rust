import { TActionContent, } from '@/models';

const actionTableConf: TActionContent | undefined = {% if rootInfo.vButtonContent %}{{ rootInfo.vButtonContent }}{%- else %}undefined{%- endif %};

export { actionTableConf }