package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.conf;

/**
 * {{ rootInfo.displayName }}
 */
public enum {{ rootInfo.className }}Enum  {
{%- for attributeInfo in rootInfo.attributeInfoList %}
    /**
     * {{ attributeInfo.displayName }}
     */
    {%- if not loop.last %}
    {{ attributeInfo.macroCaseName }}("{{ attributeInfo.camelCaseName }}"),
    {%- endif %}
    {%- if loop.last %}
    {{ attributeInfo.macroCaseName }}("{{ attributeInfo.camelCaseName }}");
    {%- endif %}
{%- endfor %}

    private String code;

    {{ rootInfo.className }}Enum(String code) {
        this.code = code;
    }

    public String getCode() {
        return code;
    }

    public static {{ rootInfo.className }}Enum getByCode(String code) {
        {{ rootInfo.className }}Enum[] {{ rootInfo.camelCaseName }}Enums = values();
        for ({{ rootInfo.className }} {{ rootInfo.camelCaseName }}Enum : {{ rootInfo.camelCaseName }}Enums) {
            if ({{ rootInfo.camelCaseName }}Enum.getCode().equals(code)) {
                return {{ rootInfo.camelCaseName }}Enum;
            }
        }
        return null;
    }
}
