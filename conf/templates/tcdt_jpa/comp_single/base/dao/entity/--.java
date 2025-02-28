package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity;

import jakarta.persistence.*;
import java.math.BigDecimal;
import java.sql.Blob;
import java.sql.Clob;
import java.sql.Timestamp;
import java.sql.Date;
import java.sql.Time;
import java.util.Set;
import org.hibernate.annotations.GenericGenerator;
import com.cft.mdc.base.BaseEntity;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.desc.{{ rootInfo.className }}Desc;
{%- for upEntityInfo in rootInfo.upEntityInfoList %}
import {{ upEntityInfo.basePath }}.{{ upEntityInfo.packageName}}.dao.desc.{{ upEntityInfo.className}}Desc;
import {{ upEntityInfo.basePath }}.{{ upEntityInfo.packageName}}.dao.entity.{{ upEntityInfo.className}};
{%- endfor %}
{%- for downEntityInfo in rootInfo.downEntityInfoList %}
import {{ downEntityInfo.basePath }}.{{ downEntityInfo.packageName}}.dao.desc.{{ downEntityInfo.className}}Desc;
import {{ downEntityInfo.basePath }}.{{ downEntityInfo.packageName}}.dao.entity.{{ downEntityInfo.className}};
{%- endfor %}
{%- for outBasePackage in rootInfo.outBasePackageList %}
import {{ outBasePackage.objectTypePackage }}.{{ outBasePackage.objectType }};
{%- endfor %}

@NamedEntityGraph(name = {{ rootInfo.className }}Desc.CLASS_NAME_ENTITY + ".graph"
{%- if rootInfo.downAttributeInfoList is iterable %}
        ,
        attributeNodes = {
    {%- for attributeInfo in rootInfo.downAttributeInfoList %}
        {%- if attributeInfo.attributeTypeCode is matching("InternalAggArray") %}
                @NamedAttributeNode({{ rootInfo.className }}Desc.{{ attributeInfo.macroCaseName }}_ATTR){%- if not loop.last %},{%- endif %}
        {%- endif %}
	{%- endfor %}
        }
{%- endif %}
        )
@Entity(name = "{{ rootInfo.tableName }}")
public class {{ rootInfo.className }} extends BaseEntity {

    /**
     * {{ rootInfo.pkAttributeInfo.displayName }}{%- if rootInfo.pkAttributeInfo.note %}:{{ rootInfo.pkAttributeInfo.note }}{%- else %}{%- endif %}
     */
    @Id
    @GenericGenerator(name = "idGenerator", strategy = "uuid")
    @Column(name = "{{ rootInfo.pkAttributeInfo.columnName }}")
    private {{ rootInfo.pkAttributeInfo.objectType }} {{ rootInfo.pkAttributeInfo.attributeName }};

{%- for attributeInfo in rootInfo.baseAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @Column(name = "{{ attributeInfo.columnName }}" 
    {%- if attributeInfo.len %}
    , length = {{ attributeInfo.len }} 
    {%- endif %}
    {%- if attributeInfo.pcs %}
            , precision = {{ attributeInfo.len }},scale = {{ attributeInfo.pcs }} 
    {%- endif %}
        )
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.fkAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @Column(name = "{{ attributeInfo.columnName }}" 
    {%- if attributeInfo.len %}
    , length = {{ attributeInfo.len }} 
    {%- endif %}
    {%- if attributeInfo.pcs %}
            , precision = {{ attributeInfo.len }},scale = {{ attributeInfo.pcs }} 
    {%- endif %}
        )
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @ManyToOne(targetEntity = {{ attributeInfo.objectType }}.class, fetch = FetchType.LAZY)
    @JoinColumn(name = "{{ attributeInfo.innerInfo.columnName }}", insertable = false, updatable = false)
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @OneToOne(targetEntity = {{ attributeInfo.objectType }}.class, fetch = FetchType.LAZY)
    @JoinColumn(name = "{{ attributeInfo.innerInfo.columnName }}", insertable = false, updatable = false)
    private {{ attributeInfo.objectType }} {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.downAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @OneToMany(mappedBy = {{ attributeInfo.objectType }}Desc.{{ attributeInfo.outerInfo.macroCaseName }}_ATTR, fetch = FetchType.LAZY)
    private Set<{{ attributeInfo.objectType }}> {{ attributeInfo.camelCaseName }};
{%- endfor %}

{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}{%- if attributeInfo.note %}:{{ attributeInfo.note }}{%- else %}{%- endif %}
     */
    @OneToOne(mappedBy = {{ attributeInfo.objectType }}Desc.{{ attributeInfo.outerInfo.macroCaseName }}_ATTR, fetch = FetchType.LAZY)
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
