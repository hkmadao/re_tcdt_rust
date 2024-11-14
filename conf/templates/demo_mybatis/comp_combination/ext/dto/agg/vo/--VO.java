{%- if rootInfo.mainEntityInfo %}
package {{ rootInfo.mainEntityInfo.basePath }}.dto.agg.vo;
{%- endif %}
{%- if not rootInfo.mainEntityInfo %}
package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dto.agg.vo;
{%- endif %}

import java.util.List;
import java.math.BigDecimal;
import java.sql.Blob;
import java.sql.Clob;
import java.sql.Timestamp;
import java.sql.Date;
import java.sql.Time;
import com.yiling.crud.common.base.entity.BaseDTO;
import com.yiling.crud.common.base.DtoIdentify;
import com.yiling.crud.common.base.DtoIgnoreProperty;
import com.yiling.crud.common.base.DtoRefLoadService;
import com.yiling.crud.common.base.DtoChildrenLoadService;
{%- for upEntityInfo in rootInfo.upEntityInfoList %}
import {{ rootInfo.basePath }}dao.entity.{{ rootInfo.packageName}};
{%- endfor %}
{%- for outBasePackage in rootInfo.outBasePackageList %}
import {{ outBasePackage.objectTypePackage }}.{{ outBasePackage.objectType }};
{%- endfor %}

public class {{ rootInfo.className }}VO extends BaseDTO {

    /**
     * {{ rootInfo.pkAttributeInfo.displayName }}{%- if rootInfo.pkAttributeInfo.note %}:{{ rootInfo.pkAttributeInfo.note }}{%- else %}{%- endif %}
     */
    @DtoIdentify
    private {{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }};

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.aggFkAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @DtoRefLoadService(associationAttribute = "{{ attributeInfo.innerInfo.camelCaseName }}", refLoadService = I{{ attributeInfo.objectType }}Service.class)
    private {{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @DtoRefLoadService(associationAttribute = "{{ attributeInfo.innerInfo.camelCaseName }}", refLoadService = I{{ attributeInfo.objectType }}Service.class)
    private {{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @DtoChildrenLoadService(associationAttribute = "{{ attributeInfo.outerFkInfo.camelCaseName }}", refLoadService = I{{ attributeInfo.objectType }}Service.class)
    private List<{{ attributeInfo.objectType }}VO> {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @DtoChildrenLoadService(associationAttribute = "{{ attributeInfo.outerFkInfo.camelCaseName }}", refLoadService = I{{ attributeInfo.objectType }}Service.class)
    private {{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }};
{%- endfor %}

    public {{ rootInfo.pkAttributeInfo.objectType }} get{{ rootInfo.pkAttributeInfo.pascalCaseName }}() {
        return {{ rootInfo.pkAttributeInfo.attributeName }};
    }

    public void set{{ rootInfo.pkAttributeInfo.pascalCaseName }}({{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }}) {
        this.{{ rootInfo.pkAttributeInfo.attributeName }} = {{ rootInfo.pkAttributeInfo.attributeName }};
    }

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.aggFkAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    public {{ attributeInfo.objectType }}VO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }}VO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.aggDownAttributeInfoList %}
    public List<{{ attributeInfo.objectType }}VO> get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}(List<{{ attributeInfo.objectType }}VO> {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.aggDownSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }}VO get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }}VO {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

}
