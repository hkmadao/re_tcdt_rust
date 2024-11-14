package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.controller;

import com.yiling.crud.common.advanquery.AqCondition;
import com.yiling.crud.common.advanquery.AqPageInfo;
import com.yiling.crud.common.advanquery.AqPageInfoInput;
import java.util.List;
import org.springframework.web.bind.annotation.*;
import io.swagger.v3.oas.annotations.Operation;
import io.swagger.v3.oas.annotations.tags.Tag;
import com.yiling.crud.common.base.controller.BaseController;
import com.yiling.crud.common.result.BusinessException;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dto.vo.{{ rootInfo.className }}VO;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dto.po.{{ rootInfo.className }}PO;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.service.I{{ rootInfo.className }}Service;

@Tag(name = "{{ rootInfo.className }}Controller", description = "{{ rootInfo.displayName }}前端控制器")
@RestController
@RequestMapping("{{ rootInfo.camelCaseName }}")
public class {{ rootInfo.className }}Controller extends BaseController<{{ rootInfo.className }}, String, {{ rootInfo.className }}PO, {{ rootInfo.className }}VO, I{{ rootInfo.className }}Service> {

    @Override
    protected {{ rootInfo.className }}VO afterFindProcess({{ rootInfo.className }} entity) throws BusinessException {
        return super.afterFindProcess(entity);
    }

    @Override
    @Operation(summary = "添加{{ rootInfo.displayName }}", description = "添加{{ rootInfo.displayName }}")
    @PostMapping("add")
    public {{ rootInfo.className }}VO add(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        return super.add(entityPO);
    }

    @Override
    @Operation(summary = "更新{{ rootInfo.displayName }}", description = "更新{{ rootInfo.displayName }}")
    @PostMapping("update")
    public {{ rootInfo.className }}VO update(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        return super.update(entityPO);
    }

    @Override
    @Operation(summary = "删除{{ rootInfo.displayName }}", description = "删除{{ rootInfo.displayName }}")
    @PostMapping("remove")
    public void remove(@RequestBody {{ rootInfo.className }}PO entityPO) throws BusinessException {
        super.remove(entityPO);
    }

    @Override
    @Operation(summary = "批量删除{{ rootInfo.displayName }}", description = "批量删除{{ rootInfo.displayName }}")
    @PostMapping("batchRemove")
    public void batchRemove(@RequestBody List<{{ rootInfo.className }}PO> entities) throws BusinessException {
        super.batchRemove(entities);
    }

    @Override
    @Operation(summary = "查询{{ rootInfo.displayName }}详情", description = "查询{{ rootInfo.displayName }}详情")
    @GetMapping("getById/{id}")
    public {{ rootInfo.className }}VO getById(@PathVariable(value = "id") String s) throws BusinessException {
        return super.getById(s);
    }

    @Override
    @Operation(summary = "高级查询{{ rootInfo.displayName }}", description = "高级查询{{ rootInfo.displayName }}")
    @PostMapping("aq")
    public List<{{ rootInfo.className }}VO> aq(@RequestBody(required = false) AqCondition condition) throws BusinessException {
        return super.aq(condition);
    }

    @Override
    @Operation(summary = "分页查询{{ rootInfo.displayName }}", description = "分页查询{{ rootInfo.displayName }}")
    @PostMapping("aqPage")
    public AqPageInfo<{{ rootInfo.className }}VO> aqPage(@RequestBody(required = false) AqPageInfoInput pageInfoInput) throws BusinessException {
        return super.aqPage(pageInfoInput);
    }
}