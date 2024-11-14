package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.impl;

import org.springframework.stereotype.Service;
import com.yiling.crud.common.base.service.impl.BaseServiceImpl;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.desc.{{ rootInfo.className }}Desc;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.mapper.{{ rootInfo.className }}Mapper;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.I{{ rootInfo.className }}Service;

@Service
public class {{ rootInfo.className }}ServiceImpl extends BaseServiceImpl<{{ rootInfo.className }}, {{ rootInfo.className }}Mapper, String, {{ rootInfo.className }}Desc> implements I{{ rootInfo.className }}Service {

}
