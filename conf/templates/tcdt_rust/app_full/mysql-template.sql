{%- for entityInfo in rootInfo.entities %}

-- {{ entityInfo.displayName }}
DROP TABLE IF EXISTS `{{ entityInfo.tableName }}`;
CREATE TABLE `{{ entityInfo.tableName }}`
(
    {%- for attributeInfo in entityInfo.baseAttributeInfoList %}
        {%- if attributeInfo.columnName %}
`{{ attributeInfo.columnName }}` {{ attributeInfo.columnType }} 
{%- if attributeInfo.len %}{%- if attributeInfo.pcs %}({{ attributeInfo.len }},{{ attributeInfo.pcs }}){%- else %}({{ attributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if attributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if attributeInfo.defaultValue %} DEFAULT {{ attributeInfo.defaultValue }}{%- else %}{%- endif %} 
COMMENT '{{ attributeInfo.displayName }}{%- if attributeInfo.note %}: {{ attributeInfo.note }}{%- else %}{%- endif %}' 
,
        {%- endif %}
    {%- endfor %}
    {%- for attributeInfo in entityInfo.fkAttributeInfoList %}
        {%- if attributeInfo.columnName %}
`{{ attributeInfo.columnName }}` {{ attributeInfo.columnType }} 
{%- if attributeInfo.len %}{%- if attributeInfo.pcs %}({{ attributeInfo.len }},{{ attributeInfo.pcs }}){%- else %}({{ attributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if attributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if attributeInfo.defaultValue %} DEFAULT {{ attributeInfo.defaultValue }}{%- else %}{%- endif %} 
COMMENT '{{ attributeInfo.displayName }}{%- if attributeInfo.note %}: {{ attributeInfo.note }}{%- else %}{%- endif %}' 
,
        {%- endif %}
    {%- endfor %}
`{{ entityInfo.pkAttributeInfo.columnName }}` {{ entityInfo.pkAttributeInfo.columnType }} 
{%- if entityInfo.pkAttributeInfo.len %}{%- if entityInfo.pkAttributeInfo.pcs %}({{ entityInfo.pkAttributeInfo.len }},{{ entityInfo.pkAttributeInfo.pcs }}){%- else %}({{ entityInfo.pkAttributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if entityInfo.pkAttributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if entityInfo.pkAttributeInfo.defaultValue %} DEFAULT {{ entityInfo.pkAttributeInfo.defaultValue }}{%- else %}{%- endif %}  
COMMENT '{{ entityInfo.pkAttributeInfo.displayName }}{%- if entityInfo.pkAttributeInfo.note %}: {{ entityInfo.pkAttributeInfo.note }}{%- else %}{%- endif %}' 
);
   
ALTER TABLE `{{ entityInfo.tableName }}` ADD CONSTRAINT `pk_{{ entityInfo.tableName }}` PRIMARY KEY(
    {{ entityInfo.pkAttributeInfo.columnName }} 
);

ALTER TABLE `{{ entityInfo.tableName }}` COMMENT='{{ entityInfo.displayName }}';
{%- endfor %}

-- 添加外键
{%- for entityInfo in rootInfo.entities %}
    {%- for attributeInfo in entityInfo.fkAttributeInfoList %}
ALTER TABLE `{{ entityInfo.tableName }}` ADD FOREIGN KEY ({{ attributeInfo.columnName }}) REFERENCES {{ attributeInfo.outEntityInfo.tableName }}({{ attributeInfo.outEntityInfo.pkAttributeInfo.snakeCaseName }});
    {%- endfor %}
{%- endfor %}
