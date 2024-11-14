import { TTreeContent } from '@/models';

const treeConf: TTreeContent | undefined = {% if rootInfo.tContent %}{{ rootInfo.tContent }}{%- else %}undefined{%- endif %};

export { treeConf }