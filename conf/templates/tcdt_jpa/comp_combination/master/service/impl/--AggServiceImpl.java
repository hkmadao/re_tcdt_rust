{%- if rootInfo.fgMain %}
package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.impl;

import org.springframework.stereotype.Service;
import com.cft.mdc.base.service.BaseAggService;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.aggdesc.{{ rootInfo.className }}MDesc;
import {{ rootInfo.realBasePath }}.{{ rootInfo.realPackageName }}.dao.entity.{{ rootInfo.className }};
import {{ rootInfo.realBasePath }}.{{ rootInfo.realPackageName }}.dao.repository.{{ rootInfo.className }}Repository;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.I{{ rootInfo.className }}AggService;

@Service
public class {{ rootInfo.className }}AggServiceImpl extends BaseAggService<{{ rootInfo.className }}, String, {{ rootInfo.className }}Repository, {{ rootInfo.className }}MDesc> implements I{{ rootInfo.className }}AggService {

}
{%- endif %}
