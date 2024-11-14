import { TQueryContent } from '~/models';

const queryConf: TQueryContent = {%- if rootInfo.qContent %}{{ rootInfo.qContent }}{%- else %}undefined{%- endif %};

export { queryConf }