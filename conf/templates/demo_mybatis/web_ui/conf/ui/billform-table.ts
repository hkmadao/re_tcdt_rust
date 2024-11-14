import { TBillFormContent } from '~/models';

const tableConf: TBillFormContent = {%- if rootInfo.bTableContent %}{{ rootInfo.bTableContent }}{%- else %}undefined{%- endif %};

export { tableConf }