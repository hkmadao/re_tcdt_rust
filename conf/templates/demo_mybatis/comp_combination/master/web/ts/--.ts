import { TAudit } from '~/models';

/**{{ rootInfo.displayName }} */
export type T{{ rootInfo.className }} = {
    /**{{ rootInfo.pkAttributeInfo.displayName }} */
    {{ rootInfo.pkAttributeInfo.camelCaseName }}: {{ rootInfo.pkAttributeInfo.ext1 }};

    {%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggFkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggUpAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggUpSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }}[];
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}
} & TAudit;

{%- for entityInfo in rootInfo.childEntityInfoList %}
/**{{ entityInfo.displayName }} */
export type T{{ entityInfo.className }} = {
    /**{{ entityInfo.pkAttributeInfo.displayName }} */
    {{ entityInfo.pkAttributeInfo.camelCaseName }}: {{ entityInfo.pkAttributeInfo.ext1 }};

    {%- for attributeInfo in entityInfo.baseAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggFkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggUpAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggUpSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }}[];
    {%- endfor %}

    {%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}
} & TAudit;
{%- endfor %}

{%- for entityInfo in rootInfo.compInfo.upEntityInfoList %}
/**{{ entityInfo.displayName }} */
export type T{{ entityInfo.className }} = {
    /**{{ entityInfo.pkAttributeInfo.displayName }} */
    {{ entityInfo.pkAttributeInfo.camelCaseName }}: {{ entityInfo.pkAttributeInfo.ext1 }};
    
    {%- for attributeInfo in entityInfo.baseAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
    {%- endfor %}

    {%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: string;
    {%- endfor %}
} & TAudit;
{%- endfor %}
