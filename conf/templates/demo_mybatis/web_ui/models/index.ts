import { TAudit } from '~/models';

/**{{ rootInfo.bMDJson.displayName }} */
export type T{{ rootInfo.bMDJson.entityInfo.className }} = {
{%- for field in rootInfo.bMDJson.children %}
  {%- if field.children %}
  /**{{ field.displayName }} */
  {{ field.attributeName }}?: {{ field.typeScriptType }};
  {%- endif %}
  {%- if field.children is iterable %}
  /**{{ field.displayName }} */
    {%- if field.fgPartner and field.refType %}
  {{ field.refAttributeName }}?: T{{ field.entityInfo.className }};
  {{ field.attributeName }}?: string;
    {%- endif %}
    {%- if field.fgPartner and field.refType and field.refType is matching("Array") %}
  {{ field.attributeName }}?: T{{ field.entityInfo.className }}[];
    {%- endif %}
    {%- if field.fgPartner and field.refType and field.refType is not matching("Array") %}
  {{ field.attributeName }}?: T{{ field.entityInfo.className }};
    {%- endif %}
    {%- if not field.fgPartner and field.refType and field.refType is matching("Array") %}
  {{ field.attributeName }}?: T{{ field.entityInfo.className }}[];
    {%- endif %}
    {%- if not field.fgPartner and field.refType and field.refType is not matching("Array") %}
  {{ field.refAttributeName }}?: T{{ field.entityInfo.className }};
  {{ field.attributeName }}?: string;
    {%- endif %}
  {%- endif %}
{%- endfor %}
} & TAudit;

{%- for field in rootInfo.bMDJson.children %}
  {%- if field.children and field.entityInfo.className != rootInfo.bMDJson.entityInfo.className %}
/**{{ field.entityInfo.displayName }} */
export type T{{ field.entityInfo.className }} = {
    {%- for field1 in field.children %}
      {%- if field1.children %}
  /**{{ field1.displayName }} */
  {{ field1.attributeName }}?: {{ field1.typeScriptType }};
      {%- endif %}
      {%- if not field1.children %}
  /**{{ field1.displayName }} */
  {{ field1.refAttributeName }}?: {%- if field1.entityInfo %}T{{ field1.entityInfo.className }}{%- endif %};
  {{ field1.attributeName }}?: string;
      {%- endif %}
    {%- endfor %}
} & TAudit;
  {%- endif %}
{%- endfor %}

{%- for field in rootInfo.bMDJson.children %}
  {%- if field.children is iterable %}
    {%- for field1 in field.children %}
      {%- if not field1.children %}
        {%- if field1.entityInfo and field1.entityInfo.className != rootInfo.bMDJson.entityInfo.className %}
/**{{ field1.entityInfo.displayName }} */
export type T{{ field1.entityInfo.className }} = {
          {%- for field2 in field1.children %}
            {%- if not field2.children %}
  /**{{ field2.displayName }} */
  {{ field2.attributeName }}?: {{ field2.typeScriptType }};
            {%- endif %}
            {%- if field2.children %}
              {%- for field3 in field2.children %}
                {%- if field3.fgPrimaryKey %}
  /**{{ field3.displayName }} */
  {{ field3.attributeName }}?: string;
                {%- endif %}
              {%- endfor %}
            {%- endif %}
          {%- endfor %}
} & TAudit;
        {%- endif %}
      {%- endif %}
    {%- endfor %}
  {%- endif %}
{%- endfor %}
