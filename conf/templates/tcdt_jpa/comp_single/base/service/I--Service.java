package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service;

import com.cft.mdc.base.service.IBaseService;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};

public interface I{{ rootInfo.className }}Service extends IBaseService<{{ rootInfo.className }},String> {


}
