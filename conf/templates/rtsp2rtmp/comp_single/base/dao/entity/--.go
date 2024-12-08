package entity

import (
	"time"
)
// {{ rootInfo.displayName }}
type {{ rootInfo.pascalCaseName }} struct {
	// {{ rootInfo.pkAttributeInfo.displayName }}
	{{ rootInfo.pkAttributeInfo.pascalCaseName }} string  `orm:"pk;column({{ rootInfo.pkAttributeInfo.columnName }})" json:"{{ rootInfo.pkAttributeInfo.camelCaseName }}"`
{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `orm:"column({{ attributeInfo.columnName }})" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} string `orm:"column({{ attributeInfo.columnName }})" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `orm:"-" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `orm:"-" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} []{{ attributeInfo.objectType }} `orm:"-" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `orm:"-" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
}
