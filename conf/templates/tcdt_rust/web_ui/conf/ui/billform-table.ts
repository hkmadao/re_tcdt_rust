import { TBillFormContent } from '@/models';

const tableConf: TBillFormContent | undefined = {% if rootInfo.bTableContent %}{{ rootInfo.bTableContent }}{%- else %}undefined{%- endif %};

export { tableConf }