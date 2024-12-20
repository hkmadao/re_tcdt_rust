package base

import (
	"fmt"

	"github.com/beego/beego/v2/core/logs"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/common"
	"github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dao/entity"
	{{ rootInfo.snakeCaseName }}_po "github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dto/po/base/{{ rootInfo.snakeCaseName }}"
	{{ rootInfo.snakeCaseName }}_vo "github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/dto/vo/base/{{ rootInfo.snakeCaseName }}"
	base_service "github.com/hkmadao/rtsp2rtmp/src/rtsp2rtmp/web/service/base"
)

func ConvertPOTo{{ rootInfo.pascalCaseName }}(po {{ rootInfo.snakeCaseName }}_po.{{ rootInfo.pascalCaseName }}PO) ({{ rootInfo.camelCaseName }} entity.{{ rootInfo.pascalCaseName }}, err error) {
	err = common.POToEntity(po, &{{ rootInfo.camelCaseName }})
	if err != nil {
		logs.Error("convertPOTo{{ rootInfo.pascalCaseName }} : %v", err)
		err = fmt.Errorf("convertPOTo{{ rootInfo.pascalCaseName }} : %v", err)
		return
	}
	return
}

func ConvertPOListTo{{ rootInfo.pascalCaseName }}(poes []{{ rootInfo.snakeCaseName }}_po.{{ rootInfo.pascalCaseName }}PO) ([]entity.{{ rootInfo.pascalCaseName }}, error) {
	{{ rootInfo.camelCaseName }}s := make([]entity.{{ rootInfo.pascalCaseName }}, len(poes))
	for i, po := range poes {
		{{ rootInfo.camelCaseName }}, err_convert := ConvertPOTo{{ rootInfo.pascalCaseName }}(po)
		if err_convert != nil {
			logs.Error("ConvertPOListTo{{ rootInfo.pascalCaseName }} : %v", err_convert)
			err := fmt.Errorf("ConvertPOListTo{{ rootInfo.pascalCaseName }} : %v", err_convert)
			return nil, err
		}
		{{ rootInfo.camelCaseName }}s[i] = {{ rootInfo.camelCaseName }}
	}
	return {{ rootInfo.camelCaseName }}s, nil
}

func Convert{{ rootInfo.pascalCaseName }}ToVO({{ rootInfo.camelCaseName }} entity.{{ rootInfo.pascalCaseName }}) (vo {{ rootInfo.snakeCaseName }}_vo.{{ rootInfo.pascalCaseName }}VO, err error) {
	vo = {{ rootInfo.snakeCaseName }}_vo.{{ rootInfo.pascalCaseName }}VO{}
	err = common.EntityToVO({{ rootInfo.camelCaseName }}, &vo)
	if err != nil {
		logs.Error("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
		err = fmt.Errorf("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
		return
	}
{%- for attributeInfo in rootInfo.upAttributeInfoList %}
{{ attributeInfo.camelCaseName }}, err := base_service.{{ attributeInfo.outEntityInfo.pascalCaseName }}SelectById(vo.{{ attributeInfo.innerInfo.pascalCaseName }})
	if err != nil {
		logs.Error("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
		err = fmt.Errorf("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
		return
	}
	var {{ attributeInfo.camelCaseName }}VO = {{ rootInfo.snakeCaseName }}_vo.{{ attributeInfo.outEntityInfo.pascalCaseName }}VO{}
	err = common.EntityToVO({{ attributeInfo.camelCaseName }}, &{{ attributeInfo.camelCaseName }}VO)
	if err != nil {
		logs.Error("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
		err = fmt.Errorf("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
		return
	}
	vo.{{ attributeInfo.pascalCaseName }} = {{ attributeInfo.camelCaseName }}VO
{%- endfor %}
{%- for attributeInfo in rootInfo.upSingleAttributeInfoList %}

{%- endfor %}
{%- for attributeInfo in rootInfo.downSingleAttributeInfoList %}

{%- endfor %}
{%- for attributeInfo in rootInfo.downAttributeInfoList %}
	// condition := common.GetEqualCondition("{{ attributeInfo.outerFkInfo.camelCaseName }}", vo.{{ rootInfo.pkAttributeInfo.pascalCaseName }})
	// var {{ attributeInfo.outEntityInfo.camelCaseName }}VOList = make([]{{ rootInfo.snakeCaseName }}_vo.{{ attributeInfo.outEntityInfo.pascalCaseName }}VO, 0)
	// var {{ attributeInfo.camelCaseName }} = make([]entity.{{ attributeInfo.outEntityInfo.pascalCaseName }}, 0)
	// {{ attributeInfo.camelCaseName }}, err = base_service.{{ attributeInfo.outEntityInfo.pascalCaseName }}FindCollectionByCondition(condition)
	// if err != nil {
	// 	logs.Error("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
	// 	err = fmt.Errorf("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
	// 	return
	// }
	// for _, {{ attributeInfo.outEntityInfo.camelCaseName }} := range {{ attributeInfo.camelCaseName }} {
	// 	var {{ attributeInfo.outEntityInfo.camelCaseName }}VO = {{ rootInfo.snakeCaseName }}_vo.{{ attributeInfo.outEntityInfo.pascalCaseName }}VO{}
	// 	err = common.EntityToVO({{ attributeInfo.outEntityInfo.camelCaseName }}, &{{ attributeInfo.outEntityInfo.camelCaseName }}VO)
	// 	if err != nil {
	// 		logs.Error("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
	// 		err = fmt.Errorf("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err)
	// 		return
	// 	}
	// 	{{ attributeInfo.outEntityInfo.camelCaseName }}VOList = append({{ attributeInfo.outEntityInfo.camelCaseName }}VOList, {{ attributeInfo.outEntityInfo.camelCaseName }}VO)
	// }
	// vo.{{ attributeInfo.camelCaseName }} = {{ attributeInfo.outEntityInfo.camelCaseName }}VOList
{%- endfor %}
	
	return
}

func Convert{{ rootInfo.pascalCaseName }}ToVOList({{ rootInfo.camelCaseName }}s []entity.{{ rootInfo.pascalCaseName }}) (voList []{{ rootInfo.snakeCaseName }}_vo.{{ rootInfo.pascalCaseName }}VO, err error) {
	voList = make([]{{ rootInfo.snakeCaseName }}_vo.{{ rootInfo.pascalCaseName }}VO, 0)
	for _, {{ rootInfo.camelCaseName }} := range {{ rootInfo.camelCaseName }}s {
		vo, err_convert := Convert{{ rootInfo.pascalCaseName }}ToVO({{ rootInfo.camelCaseName }})
		if err_convert != nil {
			logs.Error("convert{{ rootInfo.pascalCaseName }}ToVO : %v", err_convert)
			err = fmt.Errorf("Convert{{ rootInfo.pascalCaseName }}ToVOList : %v", err_convert)
			return
		}
		voList = append(voList, vo)
	}
	return
}
