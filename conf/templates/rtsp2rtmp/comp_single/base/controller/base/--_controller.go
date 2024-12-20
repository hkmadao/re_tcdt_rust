package base

import (
	"fmt"
	"net/http"
	"strings"

	"github.com/beego/beego/v2/core/logs"
	"github.com/gin-gonic/gin"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/utils"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/common"
	dto_convert "github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/controller/convert"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dao/entity"
	{{ rootInfo.snakeCaseName }}_po "github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dto/po/base/{{ rootInfo.snakeCaseName }}"
	base_service "github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/service/base"
)

func {{ rootInfo.pascalCaseName }}Add(ctx *gin.Context) {
	ctx.Writer.Header().Set("Access-Control-Allow-Origin", "*")
	po := {{ rootInfo.snakeCaseName }}_po.{{ rootInfo.pascalCaseName }}PO{}
	err := ctx.BindJSON(&po)
	if err != nil {
		logs.Error("param error : %v", err)
		result := common.ErrorResult(fmt.Sprintf("param error : %v", err))
		ctx.JSON(http.StatusOK, result)
		return
	}
	{{ rootInfo.camelCaseName }}, err := dto_convert.ConvertPOTo{{ rootInfo.pascalCaseName }}(po)
	if err != nil {
		logs.Error("getById error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	id, _ := utils.GenerateId()
	{{ rootInfo.camelCaseName }}.{{ rootInfo.pkAttributeInfo.pascalCaseName }} = id
	_, err = base_service.{{ rootInfo.pascalCaseName }}Create({{ rootInfo.camelCaseName}})
	if err != nil {
		logs.Error("insert error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	{{ rootInfo.camelCaseName }}AfterSave, err := base_service.{{ rootInfo.pascalCaseName }}SelectById(id)
	if err != nil {
		logs.Error("query by id error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	vo, err := dto_convert.Convert{{ rootInfo.pascalCaseName }}ToVO({{ rootInfo.camelCaseName }}AfterSave)
	if err != nil {
		logs.Error("getById error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultData(vo)
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}Update(ctx *gin.Context) {
	ctx.Writer.Header().Set("Access-Control-Allow-Origin", "*")
	po := {{ rootInfo.snakeCaseName }}_po.{{ rootInfo.pascalCaseName }}PO{}
	err := ctx.BindJSON(&po)
	if err != nil {
		logs.Error("param error : %v", err)
		result := common.ErrorResult(fmt.Sprintf("param error : %v", err))
		ctx.JSON(http.StatusOK, result)
		return
	}
	{{ rootInfo.camelCaseName }}, err := dto_convert.ConvertPOTo{{ rootInfo.pascalCaseName }}(po)
	if err != nil {
		logs.Error("getById error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	var id = {{ rootInfo.camelCaseName }}.{{ rootInfo.pkAttributeInfo.pascalCaseName }}

	_, err = base_service.{{ rootInfo.pascalCaseName }}SelectById(id)
	if err != nil {
		logs.Error("query by id error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	_, err = base_service.{{ rootInfo.pascalCaseName }}UpdateById({{ rootInfo.camelCaseName }})
	if err != nil {
		logs.Error("insert error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	{{ rootInfo.camelCaseName }}AfterSave, err := base_service.{{ rootInfo.pascalCaseName }}SelectById(id)
	if err != nil {
		logs.Error("query by id error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	vo, err := dto_convert.Convert{{ rootInfo.pascalCaseName }}ToVO({{ rootInfo.camelCaseName }}AfterSave)
	if err != nil {
		logs.Error("getById error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultData(vo)
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}Remove(ctx *gin.Context) {
	ctx.Writer.Header().Set("Access-Control-Allow-Origin", "*")
	po := {{ rootInfo.snakeCaseName }}_po.{{ rootInfo.pascalCaseName }}PO{}
	err := ctx.BindJSON(&po)
	if err != nil {
		logs.Error("param error : %v", err)
		result := common.ErrorResult(fmt.Sprintf("param error : %v", err))
		ctx.JSON(http.StatusOK, result)
		return
	}

	var id = po.{{ rootInfo.pkAttributeInfo.pascalCaseName }}

	{{ rootInfo.camelCaseName }}GetById, err := base_service.{{ rootInfo.pascalCaseName }}SelectById(id)
	if err != nil {
		logs.Error("query by id error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	_, err = base_service.{{ rootInfo.pascalCaseName }}Delete({{ rootInfo.camelCaseName }}GetById)
	if err != nil {
		logs.Error("delete error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultData({{ rootInfo.camelCaseName }}GetById)
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}BatchRemove(ctx *gin.Context) {
	ctx.Writer.Header().Set("Access-Control-Allow-Origin", "*")
	poes := []{{ rootInfo.snakeCaseName }}_po.{{ rootInfo.pascalCaseName }}PO{}
	err := ctx.BindJSON(&poes)
	if err != nil {
		logs.Error("param error : %v", err)
		result := common.ErrorResult(fmt.Sprintf("param error : %v", err))
		ctx.JSON(http.StatusOK, result)
		return
	}
	{{ rootInfo.camelCaseName }}s, err := dto_convert.ConvertPOListTo{{ rootInfo.pascalCaseName }}(poes)
	_, err = base_service.{{ rootInfo.pascalCaseName }}BatchDelete({{ rootInfo.camelCaseName }}s)
	if err != nil {
		logs.Error("delete error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultMsg("remove success")
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}GetById(ctx *gin.Context) {
	// ctx.Writeresult.Header().Set("Access-Control-Allow-Origin", "*")
	id, ok := ctx.Params.Get("id")
	if !ok {
		logs.Error("get param id failed")
		result := common.ErrorResult("get param id failed")
		ctx.JSON(http.StatusOK, result)
		return
	}
	{{ rootInfo.camelCaseName }}, err := base_service.{{ rootInfo.pascalCaseName }}SelectById(id)
	if err != nil {
		logs.Error("getById error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	vo, err := dto_convert.Convert{{ rootInfo.pascalCaseName }}ToVO({{ rootInfo.camelCaseName }})
	if err != nil {
		logs.Error("getById error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultData(vo)
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}GetByIds(ctx *gin.Context) {
	// ctx.Writeresult.Header().Set("Access-Control-Allow-Origin", "*")
	idsStr := ctx.Query("ids")
	idList := strings.Split(idsStr, ",")
	if len(idList) == 0 {
		logs.Error("get param ids failed")
		result := common.ErrorResult("get param ids failed")
		ctx.JSON(http.StatusOK, result)
		return
	}
	{{ rootInfo.camelCaseName }}s, err := base_service.{{ rootInfo.pascalCaseName }}SelectByIds(idList)
	if err != nil {
		logs.Error("getByIds error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}

	voList, err := dto_convert.Convert{{ rootInfo.pascalCaseName }}ToVOList({{ rootInfo.camelCaseName }}s)
	if err != nil {
		logs.Error("getByIds error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultData(voList)
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}Aq(ctx *gin.Context) {
	// ctx.Writeresult.Header().Set("Access-Control-Allow-Origin", "*")
	condition := common.AqCondition{}
	err := ctx.BindJSON(&condition)
	if err != nil {
		logs.Error("param error : %v", err)
		result := common.ErrorResult(fmt.Sprintf("param error : %v", err))
		ctx.JSON(http.StatusOK, result)
		return
	}
	{{ rootInfo.camelCaseName }}s, err := base_service.{{ rootInfo.pascalCaseName }}FindCollectionByCondition(condition)
	if err != nil {
		logs.Error("aq error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	voList, err := dto_convert.Convert{{ rootInfo.pascalCaseName }}ToVOList({{ rootInfo.camelCaseName }}s)
	if err != nil {
		logs.Error("aq error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	result := common.SuccessResultData(voList)
	ctx.JSON(http.StatusOK, result)
}

func {{ rootInfo.pascalCaseName }}AqPage(ctx *gin.Context) {
	// ctx.Writeresult.Header().Set("Access-Control-Allow-Origin", "*")
	pageInfoInput := common.AqPageInfoInput{}
	err := ctx.BindJSON(&pageInfoInput)
	if err != nil {
		ctx.AbortWithError(500, err)
		logs.Error("param error : %v", err)
		result := common.ErrorResult(fmt.Sprintf("param error : %v", err))
		ctx.JSON(http.StatusOK, result)
		return
	}
	pageInfo, err := base_service.{{ rootInfo.pascalCaseName }}FindPageByCondition(pageInfoInput)
	if err != nil {
		logs.Error("aqPage error : %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	var {{ rootInfo.camelCaseName }}s = make([]entity.{{ rootInfo.pascalCaseName }}, 0)
	for _, data := range pageInfo.DataList {
		{{ rootInfo.camelCaseName }}s = append({{ rootInfo.camelCaseName }}s, data.(entity.{{ rootInfo.pascalCaseName }}))
	}
	voList, err := dto_convert.Convert{{ rootInfo.pascalCaseName }}ToVOList({{ rootInfo.camelCaseName }}s)
	if err != nil {
		logs.Error("aqPage error: %v", err)
		result := common.ErrorResult("internal error")
		ctx.JSON(http.StatusOK, result)
		return
	}
	var dataList = make([]interface{}, 0)
	for _, vo := range voList {
		dataList = append(dataList, vo)
	}
	pageInfo.DataList = dataList
	result := common.SuccessResultData(pageInfo)
	ctx.JSON(http.StatusOK, result)
}
