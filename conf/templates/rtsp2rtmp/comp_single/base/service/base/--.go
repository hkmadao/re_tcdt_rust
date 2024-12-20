package service

import (
	"fmt"

	"github.com/beego/beego/v2/client/orm"
	"github.com/beego/beego/v2/core/logs"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/common"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dao/entity"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dyn_query"
)

func get{{ rootInfo.pascalCaseName }}Name() string {
	return "{{ rootInfo.pascalCaseName }}"
}

func {{ rootInfo.pascalCaseName }}Create(e entity.{{ rootInfo.pascalCaseName }}) (i int64, err error) {
	o := orm.NewOrm()
	i, err = o.Insert(&e)
	if err != nil && err != orm.ErrLastInsertIdUnavailable {
		logs.Error("insert error : %v", err)
		return i, err
	}
	return i, nil
}

func {{ rootInfo.pascalCaseName }}UpdateById(e entity.{{ rootInfo.pascalCaseName }}) (i int64, err error) {
	o := orm.NewOrm()
	i, err = o.Update(&e)
	if err != nil {
		logs.Error("update error : %v", err)
		return 0, err
	}
	return i, nil
}

func {{ rootInfo.pascalCaseName }}Delete(e entity.{{ rootInfo.pascalCaseName }}) (i int64, err error) {
	o := orm.NewOrm()
	i, err = o.Delete(&e)
	if err != nil {
		logs.Error("delete error : %v", err)
		return 0, err
	}
	return i, nil
}

func {{ rootInfo.pascalCaseName }}BatchDelete(es []entity.{{ rootInfo.pascalCaseName }}) (i int64, err error) {
	o := orm.NewOrm()
	for _, e := range es {
		_, err = o.Delete(&e)
		if err != nil {
			logs.Error("delete error : %v", err)
			return 0, err
		}
	}
	i = int64(len(es))
	return i, nil
}

func {{ rootInfo.pascalCaseName }}SelectById(id string) (model entity.{{ rootInfo.pascalCaseName }}, err error) {
	o := orm.NewOrm()
	model = entity.{{ rootInfo.pascalCaseName }}{{ "{ " }}{{ rootInfo.pkAttributeInfo.pascalCaseName }}: id {{ " }" }}

	err = o.Read(&model)

	if err == orm.ErrNoRows {
		logs.Error("record not found")
		return
	} else if err == orm.ErrMissPK {
		logs.Error("err miss pk")
		return
	} else if err != nil {
		logs.Error("selectById error: %v", err)
		return
	}
	return
}

func {{ rootInfo.pascalCaseName }}SelectByIds(ids []string) (models []entity.{{ rootInfo.pascalCaseName }}, err error) {
	idsNew := make([]interface{}, 0)
	for _, id := range ids {
		idsNew = append(idsNew, id)
	}
	condition := common.GetInCondition("{{ rootInfo.pkAttributeInfo.camelCaseName }}", idsNew)
	var querySqlBuilder, err_build = dyn_query.NewQuerySqlBuilder(condition, get{{ rootInfo.pascalCaseName }}Name())
	if err_build != nil {
		err = fmt.Errorf("selectByIds error: %v", err_build)
		return
	}
	var sqlStr, params, err_make_sql = querySqlBuilder.GetSql()
	if err_make_sql != nil {
		err = fmt.Errorf("selectByIds error: %v", err_make_sql)
		return
	}
	o := orm.NewOrm()
	// execute the raw query string
	_, err_query := o.Raw(sqlStr, params...).QueryRows(&models)
	if err_query != nil {
		err = fmt.Errorf("selectByIds error: %v", err_make_sql)
		return
	}

	return
}

func {{ rootInfo.pascalCaseName }}FindCollectionByCondition(condition common.AqCondition) (models []entity.{{ rootInfo.pascalCaseName }}, err error) {
	var querySqlBuilder, err_build = dyn_query.NewQuerySqlBuilder(condition, get{{ rootInfo.pascalCaseName }}Name())
	if err_build != nil {
		err = fmt.Errorf("findCollectionByCondition error: %v", err_build)
		return
	}
	var sqlStr, params, err_make_sql = querySqlBuilder.GetSql()
	if err_make_sql != nil {
		err = fmt.Errorf("findCollectionByCondition error: %v", err_make_sql)
		return
	}
	o := orm.NewOrm()
	// execute the raw query string
	_, err_query := o.Raw(sqlStr, params...).QueryRows(&models)
	if err_query != nil {
		err = fmt.Errorf("findCollectionByCondition error: %v", err_make_sql)
		return
	}
	return
}

func {{ rootInfo.pascalCaseName }}FindOneByCondition(condition common.AqCondition) (model entity.{{ rootInfo.pascalCaseName }}, err error) {
	var querySqlBuilder, err_build = dyn_query.NewQuerySqlBuilder(condition, get{{ rootInfo.pascalCaseName }}Name())
	if err_build != nil {
		err = fmt.Errorf("findOneByCondition error: %v", err_build)
		return
	}
	var sqlStr, params, err_make_sql = querySqlBuilder.GetSql()
	if err_make_sql != nil {
		err = fmt.Errorf("findOneByCondition error: %v", err_make_sql)
		return
	}
	o := orm.NewOrm()
	// execute the raw query string
	models := make([]entity.{{ rootInfo.pascalCaseName }}, 0)
	_, err_query := o.Raw(sqlStr, params...).QueryRows(&models)
	if err_query != nil {
		err = fmt.Errorf("findOneByCondition error: %v", err_make_sql)
		return
	}
	if len(models) < 1 {
		err = fmt.Errorf("record not found")
		return
	}
	if len(models) > 1 {
		err = fmt.Errorf("record more than one")
		return
	}
	model = models[0]
	return
}

func {{ rootInfo.pascalCaseName }}FindPageByCondition(aqPageInfoInput common.AqPageInfoInput) (pageInfo common.PageInfo, err error) {
	condition := common.AqCondition{LogicNode: aqPageInfoInput.LogicNode, Orders: aqPageInfoInput.Orders}
	var querySqlBuilder, err_build = dyn_query.NewQuerySqlBuilder(condition, get{{ rootInfo.pascalCaseName }}Name())
	if err_build != nil {
		err = fmt.Errorf("findPageByCondition error: %v", err_build)
		return
	}
	var countSqlStr, params, err_make_sql = querySqlBuilder.GetCountSql()
	if err_make_sql != nil {
		err = fmt.Errorf("findPageByCondition error: %v", err_make_sql)
		return
	}
	var pageSqlStr, _, err_make_page_sql = querySqlBuilder.GetPageSql(aqPageInfoInput.PageIndex, aqPageInfoInput.PageSize)
	if err_make_page_sql != nil {
		err = fmt.Errorf("findPageByCondition error: %v", err_make_page_sql)
		return
	}
	o := orm.NewOrm()
	// execute the count raw query string
	var count uint64
	err_count_query := o.Raw(countSqlStr, params...).QueryRow(&count)
	if err_count_query != nil {
		err = fmt.Errorf("findPageByCondition error: %v", err_count_query)
		return
	}
	// execute the raw query string
	models := make([]entity.{{ rootInfo.pascalCaseName }}, 0)
	_, err_query := o.Raw(pageSqlStr, params...).QueryRows(&models)
	if err_query != nil {
		err = fmt.Errorf("findPageByCondition error: %v", err_make_sql)
		return
	}
	dataList := make([]interface{}, 0)
	for _, model := range models {
		dataList = append(dataList, model)
	}
	pageInfoInput := common.PageInfoInput{PageIndex: aqPageInfoInput.PageIndex, PageSize: aqPageInfoInput.PageSize, TotalCount: count}
	pageInfo = common.PageInfo{PageInfoInput: pageInfoInput, DataList: dataList}
	return
}

func {{ rootInfo.pascalCaseName }}CountByCondition(condition common.AqCondition) (total uint64, err error) {
	var querySqlBuilder, err_build = dyn_query.NewQuerySqlBuilder(condition, get{{ rootInfo.pascalCaseName }}Name())
	if err_build != nil {
		err = fmt.Errorf("countByCondition error: %v", err_build)
		return
	}
	var countSqlStr, params, err_make_sql = querySqlBuilder.GetCountSql()
	if err_make_sql != nil {
		err = fmt.Errorf("countByCondition error: %v", err_make_sql)
		return
	}
	o := orm.NewOrm()
	// execute the count raw query string
	err_count_query := o.Raw(countSqlStr, params...).QueryRow(&total)
	if err_count_query != nil {
		err = fmt.Errorf("countByCondition error: %v", err_count_query)
		return
	}
	return
}

func {{ rootInfo.pascalCaseName }}ExistsByCondition(condition common.AqCondition) (exist bool, err error) {
	var querySqlBuilder, err_build = dyn_query.NewQuerySqlBuilder(condition, get{{ rootInfo.pascalCaseName }}Name())
	if err_build != nil {
		err = fmt.Errorf("existsByCondition error: %v", err_build)
		return
	}
	var countSqlStr, params, err_make_sql = querySqlBuilder.GetCountSql()
	if err_make_sql != nil {
		err = fmt.Errorf("existsByCondition error: %v", err_make_sql)
		return
	}
	o := orm.NewOrm()
	// execute the count raw query string
	total := 0
	err_count_query := o.Raw(countSqlStr, params...).QueryRow(&total)
	if err_count_query != nil {
		err = fmt.Errorf("existsByCondition error: %v", err_count_query)
		return
	}
	exist = total > 0
	return
}
