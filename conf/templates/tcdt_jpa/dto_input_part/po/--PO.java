package {{ rootInfo.basePath }}.{{ rootInfo.packageName}};

import java.util.List;
import java.math.BigDecimal;
import java.sql.Blob;
import java.sql.Clob;
import java.sql.Timestamp;
import java.sql.Date;
import java.sql.Time;
import com.cft.mdc.base.BaseEntity;
import com.cft.mdc.base.DtoIdentify;
{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
{%- endfor %}

public class {{ rootInfo.className }}PO extends BaseEntity {

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

{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }}PO {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }}PO {{ attributeInfo.camelCaseName }};
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private List<{{ attributeInfo.objectType }}PO> {{ attributeInfo.camelCaseName }};
{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.note }}
     */
    private {{ attributeInfo.objectType }}PO {{ attributeInfo.camelCaseName }};
{%- endfor %}

    public {{ rootInfo.pkAttributeInfo.objectType }} get{{ rootInfo.pkAttributeInfo.pascalCaseName }}() {
        return {{ rootInfo.pkAttributeInfo.attributeName }};
    }

    public void set{{ rootInfo.pkAttributeInfo.pascalCaseName }}({{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }}) {
        this.{{ rootInfo.pkAttributeInfo.attributeName }} = {{ rootInfo.pkAttributeInfo.attributeName }};
    }

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

{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    public {{ attributeInfo.objectType }}PO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}PO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }}PO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}PO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
    public List<{{ attributeInfo.objectType }}PO> get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}(List<{{ attributeInfo.objectType }}PO> {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }}PO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}PO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

}
