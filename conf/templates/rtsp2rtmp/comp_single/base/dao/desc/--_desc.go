package desc

import (
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/common"
)

func Get{{ rootInfo.pascalCaseName }}Desc() *common.EntityDesc {
    var entityInfo = common.EntityInfo {
        Name: "{{ rootInfo.pascalCaseName }}",
        DisplayName: "{{ rootInfo.displayName }}",
        ClassName: "{{ rootInfo.pascalCaseName }}",
        TableName: "{{ rootInfo.tableName }}",
        BasePath: "entity::{{ rootInfo.snakeCaseName }}",
    }
    
{%- if rootInfo.attributeInfoList %}
    {%- for attributeInfo in rootInfo.attributeInfoList %}
    var {{ attributeInfo.camelCaseName }}AttributeInfo = &common.AttributeInfo {
        ColumnName: "{{ attributeInfo.columnName }}",
        Name: "{{ attributeInfo.camelCaseName }}",
        DisplayName: "{{ attributeInfo.displayName }}",
        DataType: "{{ attributeInfo.attributeTypeCode }}",
				ValueType: "{{ attributeInfo.ext1 }}",
      {%- if attributeInfo.innerInfo %}
        InnerAttributeName: "{{ attributeInfo.innerInfo.camelCaseName }}",
      {%- endif %}
      {%- if attributeInfo.outEntityInfo %}
        OutEntityName: "{{ attributeInfo.outEntityInfo.pascalCaseName }}",
        OutEntityPkAttributeName: "{{ attributeInfo.outEntityInfo.pkAttributeInfo.camelCaseName }}",
      {%- endif %}
      {%- if attributeInfo.outerInfo %}
        OutEntityReversalAttributeName: "{{ attributeInfo.outerInfo.camelCaseName }}",
      {%- endif %}
      {%- if attributeInfo.outerFkInfo %}
        OutEntityIdReversalAttributeName: "{{ attributeInfo.outerFkInfo.camelCaseName }}",
      {%- endif %}
    };
    {%- endfor %}
{%- endif %}
    var entityDesc = &common.EntityDesc {
      EntityInfo: entityInfo,
      PkAttributeInfo: {{ rootInfo.pkAttributeInfo.camelCaseName }}AttributeInfo,
      NormalFkIdAttributeInfos: []*common.AttributeInfo{
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
          {{ attributeInfo.camelCaseName }}AttributeInfo,
{%- endfor %}
			},
      NormalFkAttributeInfos: []*common.AttributeInfo{
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
          {{ attributeInfo.camelCaseName }}AttributeInfo,
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
          {{ attributeInfo.camelCaseName }}AttributeInfo,
{%- endfor %}
			},
      NormalChildren: []*common.AttributeInfo{
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
          {{ attributeInfo.camelCaseName }}AttributeInfo,
{%- endfor %}
			},
      NormalOne2OneChildren: []*common.AttributeInfo{
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
          {{ attributeInfo.camelCaseName }}AttributeInfo,
{%- endfor %}
			},
      AttributeInfoMap: map[string]*common.AttributeInfo{
{%- if rootInfo.attributeInfoList %}
    {%- for attributeInfo in rootInfo.attributeInfoList %}
          "{{ attributeInfo.camelCaseName }}": {{ attributeInfo.camelCaseName }}AttributeInfo,
    {%- endfor %}
{%- endif %}
			},
    }

    return entityDesc
}
