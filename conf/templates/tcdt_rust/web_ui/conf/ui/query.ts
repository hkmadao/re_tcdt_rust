import { TQueryContent } from '@/models';

const queryConf: TQueryContent | undefined = {% if rootInfo.qContent %}{{ rootInfo.qContent }}{%- else %}undefined{%- endif %};

export { queryConf }