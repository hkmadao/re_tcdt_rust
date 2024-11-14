import { TTreeContent } from '~/models';

const treeConf: TTreeContent = {%- if rootInfo.tContent %}{{ rootInfo.tContent }}{%- else %}undefined{%- endif %};

export { treeConf }