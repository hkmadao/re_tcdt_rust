package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service;

import com.yiling.crud.common.base.service.IBaseService;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};

public interface I{{ rootInfo.className }}Service extends IBaseService<{{ rootInfo.className }},String> {


}
