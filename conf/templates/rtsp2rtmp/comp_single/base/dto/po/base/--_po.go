package po

import (
	"time"
)
// {{ rootInfo.displayName }}
type {{ rootInfo.pascalCaseName }}PO struct {
	// {{ rootInfo.pkAttributeInfo.displayName }}
	{{ rootInfo.pkAttributeInfo.pascalCaseName }} string  `json:"{{ rootInfo.pkAttributeInfo.camelCaseName }}"`
{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} string `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	// {{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	// {{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	// {{ attributeInfo.pascalCaseName }} []{{ attributeInfo.objectType }} `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	// {{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
}
