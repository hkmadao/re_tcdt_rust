import { TAudit } from '@/models';

/**{{ rootInfo.displayName }} */
export type T{{ rootInfo.className }} = {
    /**{{ rootInfo.pkAttributeInfo.displayName }} */
    {{ rootInfo.pkAttributeInfo.camelCaseName }}: {{ rootInfo.pkAttributeInfo.ext1 }};
    {%- for attributeInfo in rootInfo.baseAttributeInfoList %}
        {%- if attributeInfo.attributeTypeCode is matching("xxxxxxx") %}
        {%- else %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
        {%- endif %}
    {%- endfor %}
    {%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
    {%- endfor %}
    {%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}
    {%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}
    {%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }};
    {%- endfor %}
    {%- for attributeInfo in rootInfo.downAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.outEntityInfo.className }}[];
    {%- endfor %}
} & TAudit;
