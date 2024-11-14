-- {{ rootInfo.displayName }}
DROP TABLE IF EXISTS `{{ rootInfo.tableName }}`;
CREATE TABLE `{{ rootInfo.tableName }}`
(
    {%- for attributeInfo in rootInfo.baseAttributeInfoList %}
        {%- if attributeInfo.columnName %}
`{{ attributeInfo.columnName }}` {{ attributeInfo.columnType }} 
{%- if attributeInfo.len %}{%- if attributeInfo.pcs %}({{ attributeInfo.len }},{{ attributeInfo.pcs }}){%- else %}({{ attributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if attributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if attributeInfo.defaultValue %} DEFAULT {{ attributeInfo.defaultValue }}{%- else %}{%- endif %} 
COMMENT '{{ attributeInfo.displayName }}{%- if attributeInfo.note %}: {{ attributeInfo.note }}{%- else %}{%- endif %}' 
,
        {%- endif %}
    {%- endfor %}
    {%- for attributeInfo in rootInfo.fkAttributeInfoList %}
        {%- if attributeInfo.columnName %}
`{{ attributeInfo.columnName }}` {{ attributeInfo.columnType }} 
{%- if attributeInfo.len %}{%- if attributeInfo.pcs %}({{ attributeInfo.len }},{{ attributeInfo.pcs }}){%- else %}({{ attributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if attributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if attributeInfo.defaultValue %} DEFAULT {{ attributeInfo.defaultValue }}{%- else %}{%- endif %} 
COMMENT '{{ attributeInfo.displayName }}{%- if attributeInfo.note %}: {{ attributeInfo.note }}{%- else %}{%- endif %}' 
,
        {%- endif %}
    {%- endfor %}
`{{ rootInfo.pkAttributeInfo.columnName }}` {{ rootInfo.pkAttributeInfo.columnType }} 
{%- if rootInfo.pkAttributeInfo.len %}{%- if rootInfo.pkAttributeInfo.pcs %}({{ rootInfo.pkAttributeInfo.len }},{{ rootInfo.pkAttributeInfo.pcs }}){%- else %}({{ rootInfo.pkAttributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if rootInfo.pkAttributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if rootInfo.pkAttributeInfo.defaultValue %} DEFAULT {{ rootInfo.pkAttributeInfo.defaultValue }}{%- else %}{%- endif %}  
COMMENT '{{ rootInfo.pkAttributeInfo.displayName }}{%- if rootInfo.pkAttributeInfo.note %}: {{ rootInfo.pkAttributeInfo.note }}{%- else %}{%- endif %}' 
);
   
ALTER TABLE `{{ rootInfo.tableName }}` ADD CONSTRAINT `pk_{{ rootInfo.tableName }}` PRIMARY KEY(
    {{ rootInfo.pkAttributeInfo.columnName }} 
);

ALTER TABLE `{{ rootInfo.tableName }}` COMMENT='{{ rootInfo.displayName }}';
{%- endfor %}