package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.impl;

import org.springframework.stereotype.Service;
import com.cft.mdc.base.service.BaseService;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.desc.{{ rootInfo.className }}Desc;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.repository.{{ rootInfo.className }}Repository;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.I{{ rootInfo.className }}Service;

@Service
public class {{ rootInfo.className }}ServiceImpl extends BaseService<{{ rootInfo.className }}, String, {{ rootInfo.className }}Repository, {{ rootInfo.className }}Desc> implements I{{ rootInfo.className }}Service {

}
