import { TAudit } from '@/models';

{%- for entityInfo in rootInfo.entities %}
/**{{ entityInfo.displayName }} */
export type T{{ entityInfo.className }}VO = {
    /**{{ entityInfo.pkAttributeInfo.displayName }} */
    {{ entityInfo.pkAttributeInfo.camelCaseName }}: {{ entityInfo.pkAttributeInfo.ext1 }};
    {%- for attributeInfo in entityInfo.baseAttributeInfoList %}
        {%- if attributeInfo.attributeTypeCode is matching("xxxxxxx") %}
        {%- else %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
        {%- endif %}
    {%- endfor %}
    {%- for attributeInfo in entityInfo.fkAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: {{ attributeInfo.ext1 }};
    {%- endfor %}
    {%- for attributeInfo in entityInfo.upAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.objectType }}VO;
    {%- endfor %}
    {%- for attributeInfo in entityInfo.upSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.objectType }}VO;
    {%- endfor %}
    {%- for attributeInfo in entityInfo.downAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.objectType }}VO[];
    {%- endfor %}
    {%- for attributeInfo in entityInfo.downSingleAttributeInfoList %}
    /**{{ attributeInfo.displayName }} */
    {{ attributeInfo.camelCaseName }}: T{{ attributeInfo.objectType }}VO;
    {%- endfor %}
} & TAudit;

{%- endfor %}
