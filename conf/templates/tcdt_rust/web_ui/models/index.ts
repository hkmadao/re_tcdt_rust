import { TAudit } from '@/models';

/**{{ rootInfo.bMDJson.displayName }} */
export type T{{ rootInfo.bMDJson.entityInfo.className }} = {
{%- for field in rootInfo.bMDJson.children %}
  {%- if field.columnType %}
  /**{{ field.displayName }} */
  {{ field.attributeName }}?: {{ field.typeScriptType }};
  {%- endif %}
  {%- if not field.columnType %}
  /**{{ field.displayName }} */
    {%- if field.fgPartner and field.entityInfo and field.refType and field.refType == "Array" %}
  {{ field.attributeName }}?: T{{ field.entityInfo.className }}[];
    {%- endif %}
    {%- if field.fgPartner and field.entityInfo and field.refType and field.refType != "Array" %}
  {{ field.attributeName }}?: T{{ field.entityInfo.className }};
    {%- endif %}
    {%- if not field.fgPartner and field.entityInfo %}
  {{ field.attributeName }}?: T{{ field.entityInfo.className }};
      {%- if field.innerInfo and field.innerInfo.attributeName %}
  {{ field.innerInfo.attributeName }}?: string;
      {%- endif %}
    {%- endif %}
  {%- endif %}
{%- endfor %}
} & TAudit;

{%- for field in rootInfo.bMDJson.children %}
  {%- if field.children is iterable and field.entityInfo and field.entityInfo.className != rootInfo.bMDJson.entityInfo.className %}
/**{{ field.entityInfo.displayName }} */
export type T{{ field.entityInfo.className }} = {
    {%- for field1 in field.children %}
      {%- if field1.columnType %}
  /**{{ field1.displayName }} */
  {{ field1.attributeName }}?: {{ field1.typeScriptType }};
      {%- endif %}
      {%- if field1.children is iterable and field1.entityInfo %}
  /**{{ field1.displayName }} */
  {{ field1.attributeName }}?: T{{ field1.entityInfo.className }};
        {%- if field1.innerInfo and field1.innerInfo.attributeName %}
  {{ field1.innerInfo.attributeName }}?: string;
        {%- endif %}
      {%- endif %}
    {%- endfor %}
} & TAudit;
  {%- endif %}
{%- endfor %}

{%- for field in rootInfo.bMDJson.children %}
  {%- if field.children is iterable %}
    {%- for field1 in field.children %}
      {%- if field1.children is iterable %}
        {%- if field1.entityInfo and field1.entityInfo.className != rootInfo.bMDJson.entityInfo.className %}
/**{{ field1.entityInfo.displayName }} */
export type T{{ field1.entityInfo.className }} = {
          {%- for field2 in field1.children %}
            {%- if field2.columnType %}
  /**{{ field2.displayName }} */
  {{ field2.attributeName }}?: {{ field2.typeScriptType }};
            {%- endif %}
            {%- if field2.children is iterable %}
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
