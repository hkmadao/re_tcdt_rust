package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.controller;

import com.cft.mdc.advanquery.AqCondition;
import com.cft.mdc.advanquery.AqPageInfo;
import com.cft.mdc.advanquery.AqPageInfoInput;
import java.util.List;
import org.springframework.web.bind.annotation.*;
import com.cft.mdc.base.controller.BaseController;
import com.cft.mdc.base.BusinessException;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dto.vo.{{ rootInfo.className }}VO;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dto.po.{{ rootInfo.className }}PO;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.I{{ rootInfo.className }}Service;

@RestController
@RequestMapping("{{ rootInfo.camelCaseName }}")
public class {{ rootInfo.className }}Controller extends BaseController<{{ rootInfo.className }}, String, {{ rootInfo.className }}PO, {{ rootInfo.className }}VO, I{{ rootInfo.className }}Service> {

    @Override
    protected {{ rootInfo.className }}VO afterFindProcess({{ rootInfo.className }} entity) throws BusinessException {
        return super.afterFindProcess(entity);
    }

    @Override
    @PostMapping("add")
    public {{ rootInfo.className }}VO add(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        return super.add(entityPO);
    }

    @Override
    @PostMapping("update")
    public {{ rootInfo.className }}VO update(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        return super.update(entityPO);
    }

    @Override
    @PostMapping("remove")
    public void remove(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        super.remove(entityPO);
    }

    @Override
    @PostMapping("batchRemove")
    public void batchRemove(@RequestBody List<{{ rootInfo.className }}> entities) throws BusinessException {
        super.batchRemove(entities);
    }

    @Override
    @GetMapping("getById/{id}")
    public {{ rootInfo.className }}VO getById(@PathVariable(value = "id") String s) throws BusinessException {
        return super.getById(s);
    }

    @Override
    @PostMapping("getAll")
    public List<{{ rootInfo.className }}VO> findAll(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        return super.findAll(entityPO);
    }

    @Override
    @PostMapping("aq")
    public List<{{ rootInfo.className }}VO> aq(@RequestBody(required = false) AqCondition condition) throws BusinessException {
        return super.aq(condition);
    }

    @Override
    @PostMapping("aqPage")
    public AqPageInfo<{{ rootInfo.className }}VO> aqPage(@RequestBody(required = false) AqPageInfoInput pageInfoInput) throws BusinessException {
        return super.aqPage(pageInfoInput);
    }
}