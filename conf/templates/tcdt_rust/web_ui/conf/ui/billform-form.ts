import { TBillFormContent } from '@/models';

const billformConf: TBillFormContent | undefined = {% if rootInfo.bContent %}{{ rootInfo.bContent }}{%- else %}undefined{%- endif %};

export { billformConf }