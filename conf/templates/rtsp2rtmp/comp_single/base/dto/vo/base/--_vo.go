package vo

import (
	"time"
)
// {{ rootInfo.displayName }}
type {{ rootInfo.pascalCaseName }}VO struct {
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
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }}VO `vo:"ignore" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }}VO `vo:"ignore" json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	// {{ attributeInfo.pascalCaseName }} []{{ attributeInfo.objectType }}VO `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	// {{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }}VO `json:"{{ attributeInfo.camelCaseName }}"`
{%- endfor %}
}

{%- for upEntiInfo in rootInfo.compInfo.upEntityInfoList %}
type {{ upEntiInfo.pascalCaseName }}VO struct{
   // {{ upEntiInfo.pkAttributeInfo.displayName }}
	{{ upEntiInfo.pkAttributeInfo.pascalCaseName }} string  `json:"{{ upEntiInfo.pkAttributeInfo.camelCaseName }}"`
  {%- for attributeInfo in upEntiInfo.baseAttributeInfoList %}
	// {{ attributeInfo.displayName }}{%- if attributeInfo.note %}{% else %}:{{ attributeInfo.note }}{%- endif %}
	{{ attributeInfo.pascalCaseName }} {{ attributeInfo.objectType }} `json:"{{ attributeInfo.camelCaseName }}"`
  {%- endfor %}
}
{%- endfor %}