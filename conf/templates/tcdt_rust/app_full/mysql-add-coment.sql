{%- for entityInfo in rootInfo.entities %}
-- {{ entityInfo.displayName }}
    {%- for attributeInfo in entityInfo.baseAttributeInfoList %}
        {%- if attributeInfo.columnName %}
ALTER TABLE {{ entityInfo.tableName }} MODIFY COLUMN {{ attributeInfo.columnName }} {{ attributeInfo.columnType }} 
{%- if attributeInfo.len %}{%- if attributeInfo.pcs %}({{ attributeInfo.len }},{{ attributeInfo.pcs }}){%- else %}({{ attributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if attributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if attributeInfo.defaultValue %} DEFAULT {{ attributeInfo.defaultValue }}{%- else %}{%- endif %}
COMMENT '{{ attributeInfo.displayName }}{%- if attributeInfo.note %}: {{ attributeInfo.note }}{%- else %}{%- endif %}';
        {%- endif %}
    {%- endfor %}
    {%- for attributeInfo in entityInfo.fkAttributeInfoList %}
        {%- if attributeInfo.columnName %}
ALTER TABLE {{ entityInfo.tableName }} MODIFY COLUMN {{ attributeInfo.columnName }} {{ attributeInfo.columnType }} 
{%- if attributeInfo.len %}{%- if attributeInfo.pcs %}({{ attributeInfo.len }},{{ attributeInfo.pcs }}){%- else %}({{ attributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if attributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if attributeInfo.defaultValue %} DEFAULT {{ attributeInfo.defaultValue }}{%- else %}{%- endif %} 
COMMENT '{{ attributeInfo.displayName }}{%- if attributeInfo.note %}: {{ attributeInfo.note }}{%- else %}{%- endif %}';
        {%- endif %}
    {%- endfor %}
ALTER TABLE {{ entityInfo.tableName }} MODIFY COLUMN {{ entityInfo.pkAttributeInfo.columnName }} {{ entityInfo.pkAttributeInfo.columnType }} 
{%- if entityInfo.pkAttributeInfo.len %}{%- if entityInfo.pkAttributeInfo.pcs %}({{ entityInfo.pkAttributeInfo.len }},{{ entityInfo.pkAttributeInfo.pcs }}){%- else %}({{ entityInfo.pkAttributeInfo.len }}){%- endif %}{%- else %}{%- endif %} 
{%- if entityInfo.pkAttributeInfo.fgMandatory %} NOT NULL {%- else %}{%- endif %} 
{%- if entityInfo.pkAttributeInfo.defaultValue %} DEFAULT {{ entityInfo.pkAttributeInfo.defaultValue }}{%- else %}{%- endif %} 
COMMENT '{{ entityInfo.pkAttributeInfo.displayName }}{%- if entityInfo.pkAttributeInfo.note %}: {{ entityInfo.pkAttributeInfo.note }}{%- else %}{%- endif %}';

ALTER TABLE `{{ entityInfo.tableName }}` COMMENT='{{ entityInfo.displayName }}';
{%- endfor %}


