package {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.repository;

import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.desc.{{ rootInfo.className }}Desc;
import {{ rootInfo.basePath }}.{{ rootInfo.packageName}}.dao.entity.{{ rootInfo.className }};
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.domain.Specification;
import org.springframework.data.jpa.repository.EntityGraph;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.JpaSpecificationExecutor;
import org.springframework.lang.Nullable;
import org.springframework.stereotype.Repository;

@Repository
public interface {{ rootInfo.className }}Repository extends JpaRepository<{{ rootInfo.className }}, String>, JpaSpecificationExecutor<{{ rootInfo.className }}> {

    @EntityGraph(value = {{ rootInfo.className }}Desc.CLASS_NAME_ENTITY + ".graph")
    Page<{{ rootInfo.className }}> findAll(@Nullable Specification<{{ rootInfo.className }}> spec, Pageable pageable);

}
