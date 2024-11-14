package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.aggdesc;

import java.util.Arrays;
import com.yiling.crud.common.base.desc.AggBaseDesc;
import com.yiling.crud.common.base.desc.AttributeInfo;
import com.yiling.crud.common.base.desc.EntityInfo;
import com.yiling.crud.common.base.desc.JoinInfo;
import com.yiling.crud.common.base.enums.EDataType;

public class {{ rootInfo.className }}MDesc extends AggBaseDesc {

    public static final String CLASS_NAME_ENTITY = "{{ rootInfo.className }}";

    {%- for attributeInfo in rootInfo.attributeInfoList %}
    public static final String {{ attributeInfo.macroCaseName }}_ATTR = "{{ attributeInfo.camelCaseName }}";
   {%- endfor %}

    /**
     * 实体描述信息
     */
    @Override
    protected void setEntityInfo() {
        entityInfo = new EntityInfo();
        entityInfo.setName("{{ rootInfo.camelCaseName }}");
        entityInfo.setDisplayName("{{ rootInfo.displayName }}");
        entityInfo.setClassName(CLASS_NAME_ENTITY);
        entityInfo.setTableName("{{ rootInfo.tableName }}");
        entityInfo.setBasePath("{{ rootInfo.basePath }}.{{ rootInfo.packageName}}");
    }

    /**
     * 主键描述信息
     */
    @Override
    protected void setPkAttributeInfo() {
        pkAttributeInfo = {{ rootInfo.pkAttributeInfo.attributeName }}Desc();
    }


    /**
     * 获取不在同一个聚合根下的外键Id属性描述
     */
    @Override
    protected void setNormalFkIdAttributeInfos() {
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
        normalFkIdAttributeInfos.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
    }

    /**
     * 获取不在同一个聚合根下的外键属性描述
     */
    @Override
    protected void setNormalFkAttributeInfos() {
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
        normalFkAttributeInfos.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
        normalFkAttributeInfos.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
    }

    /**
     * 获取不在同一个聚合根下1:N子属性描述
     */
    @Override
    protected void setNormalChildren() {
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
        normalChildren.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
    }

    /**
     * 获取不在同一个聚合根下1:1子属性描述
     */
    @Override
    protected void setNormalOne2OneChildren() {
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
        normalChildren.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
    }

    /**
     * 获取同一个聚合根下的外键Id属性描述
     */
    @Override
    protected void setAggFkIdAttributeInfo() {
{%- for attributeInfo in rootInfo.aggFkAttributeInfoList %}
        aggFkIdAttributeInfo = {{ attributeInfo.camelCaseName }}Desc();
{%- endfor %}
    }

    /**
     * 获取同一个聚合根下的外键属性描述
     */
    @Override
    protected void setAggFkAttributeInfo() {
{%- for attributeInfo in rootInfo.aggUpAttributeInfoList %}
        aggFkAttributeInfo = {{ attributeInfo.camelCaseName }}Desc();
{%- endfor %}
{%- for attributeInfo in rootInfo.aggUpSingleAttributeInfoList %}
        aggFkAttributeInfo = {{ attributeInfo.camelCaseName }}Desc();
{%- endfor %}
    }

    /**
     * 获取同一个聚合根下子属性描述(在一个聚合内的属性，和聚合根一起保存，非一个聚合内的不算在子属下范围内)
     */
    @Override
    protected void setAggChildren() {
{%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
        aggChildren.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
    }

    @Override
    protected void setAggOne2OneChildren() {
{%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
        aggOne2OneChildren.add({{ attributeInfo.camelCaseName }}Desc());
{%- endfor %}
    }

    /**
     * 属性描述信息
     */
    @Override
    protected void setAttributeInfoMap() {
{%- if rootInfo.attributeInfoList is iterable %}
    {%- for attributeInfo in rootInfo.attributeInfoList %}
        attributeInfoMap.put({{ attributeInfo.macroCaseName }}_ATTR, {{ attributeInfo.camelCaseName }}Desc());
    {%- endfor %}
{%- endif %}
    }

{%- if rootInfo.attributeInfoList is iterable %}
    {%- for attributeInfo in rootInfo.attributeInfoList %}
    private final AttributeInfo {{ attributeInfo.camelCaseName }}Desc() {
        AttributeInfo attributeInfo = new AttributeInfo();
        attributeInfo.setName({{ attributeInfo.macroCaseName }}_ATTR);
        attributeInfo.setDisplayName("{{ attributeInfo.note }}");
        attributeInfo.setColumnName("{{ attributeInfo.columnName }}");
        {%- if attributeInfo.outEntityInfo %}
        attributeInfo.setOutEntityName("{{ attributeInfo.outEntityInfo.className }}");
        attributeInfo.setOutEntityPKAttributeName("{{ attributeInfo.outEntityInfo.pkAttributeInfo.attributeName }}");
        {%- endif %}
        {%- if attributeInfo.outerInfo %}
        attributeInfo.setOutEntityReversalAttributeName("{{ attributeInfo.outerInfo.attributeName }}");
        {%- endif %}
        {%- if attributeInfo.outerFkInfo %}
        attributeInfo.setOutEntityIdReversalAttributeName("{{ attributeInfo.outerFkInfo.attributeName }}");
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalPK") %}
        attributeInfo.setDataType(EDataType.INTERNAL_PK);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalFK") %}
        attributeInfo.setDataType(EDataType.INTERNAL_FK);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalRef") %}
        attributeInfo.setDataType(EDataType.REF);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalSingleRef") %}
        attributeInfo.setDataType(EDataType.SINGLE_REF);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalArray") %}
        attributeInfo.setDataType(EDataType.ARRAY);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalSingle") %}
        attributeInfo.setDataType(EDataType.SINGLE);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalAggFK") %}
            {%- if attributeInfo.outerInfo %}
        attributeInfo.setAggRefDescClassName("{{ attributeInfo.innerInfo.objectType }}MDesc");
            {%- endif %}
        attributeInfo.setDataType(EDataType.AGG_FK);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalAggRef") %}
            {%- if attributeInfo.outerInfo %}
        attributeInfo.setAggRefDescClassName("{{ attributeInfo.objectType }}MDesc");
            {%- endif %}
        attributeInfo.setDataType(EDataType.AGG_REF);
        JoinInfo joinInfo = new JoinInfo();
        joinInfo.setJoinName("{{ attributeInfo.innerInfo.attributeName }}");
        joinInfo.setSourceName("{{ attributeInfo.outEntityInfo.pkAttributeInfo.attributeName }}");
        attributeInfo.setJoinInfos(Arrays.asList(joinInfo));
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalAggSingleRef") %}
            {%- if attributeInfo.outerInfo %}
        attributeInfo.setAggRefDescClassName("{{ attributeInfo.objectType }}MDesc");
            {%- endif %}
        attributeInfo.setDataType(EDataType.AGG_SINGLE_REF);
        JoinInfo joinInfo = new JoinInfo();
        joinInfo.setJoinName("{{ attributeInfo.innerInfo.attributeName }}");
        joinInfo.setSourceName("{{ attributeInfo.outEntityInfo.pkAttributeInfo.attributeName }}");
        attributeInfo.setJoinInfos(Arrays.asList(joinInfo));
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalAggArray") %}
            {%- if attributeInfo.outEntityInfo %}
        attributeInfo.setAggRefDescClassName("{{ attributeInfo.objectType }}MDesc");
            {%- endif %}
        attributeInfo.setDataType(EDataType.AGG_ARRAY);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalAggSingle") %}
            {%- if attributeInfo.outEntityInfo %}
        attributeInfo.setAggRefDescClassName("{{ attributeInfo.objectType }}MDesc");
            {%- endif %}
        attributeInfo.setDataType(EDataType.AGG_SINGLE);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("String") %}
        attributeInfo.setDataType(EDataType.STRING);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("Integer") %}
        attributeInfo.setDataType(EDataType.INTEGER);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("Long") %}
        attributeInfo.setDataType(EDataType.LONG);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("Float") %}
        attributeInfo.setDataType(EDataType.DOUBLE);
        {%- endif %}
        {%- if attributeInfo.attributeTypeCode is matching("Double") %}
        attributeInfo.setDataType(EDataType.DOUBLE);
        {%- endif %}

        {%- if attributeInfo.innerInfo %}
        attributeInfo.setInnerAttributeName({{ attributeInfo.innerInfo.macroCaseName }}_ATTR);
        {%- endif %}
        return attributeInfo;
    }
    {%- endfor %}
{%- endif %}
}
