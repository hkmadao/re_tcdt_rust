{%- if rootInfo.fgMain %}
package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.impl;

import org.springframework.stereotype.Service;
import com.yiling.crud.common.base.service.impl.BaseAggService;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.aggdesc.{{ rootInfo.className }}MDesc;
import {{ rootInfo.realBasePath }}.{{ rootInfo.realPackageName }}.dao.entity.{{ rootInfo.className }};
import {{ rootInfo.realBasePath }}.{{ rootInfo.realPackageName }}.dao.mapper.{{ rootInfo.className }}Mapper;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.I{{ rootInfo.className }}AggService;

@Service
public class {{ rootInfo.className }}AggServiceImpl extends BaseAggService<{{ rootInfo.className }}, {{ rootInfo.className }}Mapper, String, {{ rootInfo.className }}MDesc> implements I{{ rootInfo.className }}AggService {

}
{%- endif %}
