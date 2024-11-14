package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity;

import com.baomidou.mybatisplus.annotation.TableField;
import com.baomidou.mybatisplus.annotation.TableId;
import com.baomidou.mybatisplus.annotation.TableName;
import java.io.Serializable;
import java.time.LocalDate;
import java.math.BigDecimal;
import java.sql.Blob;
import java.sql.Clob;
import java.sql.Timestamp;
import java.sql.Date;
import java.sql.Time;
import java.util.Set;
import com.yiling.crud.common.base.entity.BaseEntity;
{%- for upEntityInfo in rootInfo.upEntityInfoList %}
import {{ rootInfo.basePath }}dao.entity.{{ rootInfo.packageName}};
{%- endfor %}
{%- for outBasePackage in rootInfo.outBasePackageList %}
import {{ outBasePackage.objectTypePackage }}.{{ outBasePackage.objectType }};
{%- endfor %}

@TableName("{{ rootInfo.tableName }}")
public class {{ rootInfo.className }} extends BaseEntity<{{ rootInfo.className }}> {

    /**
     * {{ rootInfo.pkAttributeInfo.displayName }}{%- if rootInfo.pkAttributeInfo.note %}:{{ rootInfo.pkAttributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableId("{{ rootInfo.pkAttributeInfo.columnName }}")
    private {{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }};

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableField("{{ attributeInfo.columnName }}")
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableField("{{ attributeInfo.columnName }}")
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableField(exist = false)
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableField(exist = false)
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.downAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableField(exist = false)
    private Set<{{ attributeInfo.objectType }}> {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @TableField(exist = false)
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
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

{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.downAttributeInfoList %}
    public Set<{{ attributeInfo.objectType }}> get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}(Set<{{ attributeInfo.objectType }}> {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}

{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
    public {{ attributeInfo.objectType }} get{{ attributeInfo.pascalCaseName }}() {
        return {{ attributeInfo.camelCaseName }};
    }

    public void set{{ attributeInfo.pascalCaseName }}({{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }}) {
        this.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.camelCaseName }};
    }
{%- endfor %}
}
