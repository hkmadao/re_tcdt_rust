package {{ rootInfo.basePath }}.{{ rootInfo.packageName}};

import java.util.List;
import java.math.BigDecimal;
import java.sql.Blob;
import java.sql.Clob;
import java.sql.Timestamp;
import java.sql.Date;
import java.sql.Time;
import com.yiling.crud.common.base.entity.BaseEntity;
import com.yiling.crud.common.base.DtoIdentify;
import com.yiling.crud.common.base.DtoIgnoreProperty;
{%- for attributeInfo in entityInfo.fkAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in entityInfo.upAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in entityInfo.upSingleAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in entityInfo.downAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in entityInfo.downSingleAttributeInfoList %}
{%- endfor %}

public class {{ rootInfo.className }}VO extends BaseEntity {

    /**
     * {{ rootInfo.pkAttributeInfo.note }}
     */
    @DtoIdentify
    private {{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }};

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    {%- if attributeInfo.attributeTypeCode is matching("xxxxxxx") %}
    {%- else %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
    {%- endif %}
{%- endfor %}

{%- for attributeInfo in entityInfo.fkAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}
{%- for attributeInfo in entityInfo.upAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in entityInfo.upSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }};
{%- endfor %}
{%- for attributeInfo in entityInfo.downAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private List<{{ attributeInfo.objectType }}VO> {{ attributeInfo.camelCaseName }};
{%- endfor %}
{%- for attributeInfo in entityInfo.downSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }};
{%- endfor %}

    public {{ rootInfo.pkAttributeInfo.objectType }} get{{ rootInfo.pkAttributeInfo.upperCamelCaseName }}() {
        return {{ rootInfo.pkAttributeInfo.attributeName }};
    }

    public void set{{ rootInfo.pkAttributeInfo.upperCamelCaseName }}({{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }}) {
        this.{{ rootInfo.pkAttributeInfo.attributeName }} = {{ rootInfo.pkAttributeInfo.attributeName }};
    }

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    {%- if attributeInfo.attributeTypeCode is matching("xxxxxx") %}
    {%- else %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.upperCamelCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.upperCamelCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }

    {%- endif %}
{%- endfor %}

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    {%- if attributeInfo.attributeTypeCode is matching("xxxxxx") %}
    {%- else %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }

    {%- endif %}
{%- endfor %}

{%- for attributeInfo in entityInfo.fkAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
{%- for attributeInfo in entityInfo.upAttributeInfoList %}
    public {{ attributeInfo.objectType }}VO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in entityInfo.upSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }}VO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
{%- for attributeInfo in entityInfo.downAttributeInfoList %}
    public List<{{ attributeInfo.objectType }}VO> get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}(List<{{ attributeInfo.objectType }}VO> {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
{%- for attributeInfo in entityInfo.downSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }}VO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

}
