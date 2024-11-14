{%- if rootInfo.fgMain %}
package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service;

import com.cft.mdc.base.service.IBaseAggService;
import {{ rootInfo.realBasePath }}.{{ rootInfo.realPackageName }}.dao.entity.{{ rootInfo.className }};

public interface I{{ rootInfo.className }}AggService extends IBaseAggService<{{ rootInfo.className }},String> {


}
{%- endif %}
