import { TBillFormContent } from '~/models';

const billformConf: TBillFormContent = {%- if rootInfo.bContent %}{{ rootInfo.bContent }}{%- else %}undefined{%- endif %};

export { billformConf }