-- tcdt_rust.dd_project definition

CREATE TABLE `dd_project` (
  `code` varchar(255) DEFAULT NULL COMMENT '项目编号',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `path` varchar(255) DEFAULT NULL COMMENT '系统路径',
  `template_code` varchar(255) DEFAULT NULL COMMENT '项目模板编号',
  `note` varchar(255) DEFAULT NULL COMMENT '备注',
  `id_project` varchar(255) NOT NULL COMMENT '项目id',
  `file_name_type` varchar(100) DEFAULT NULL COMMENT '文件名样式',
  PRIMARY KEY (`id_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='项目';


-- tcdt_rust.sys_role definition

CREATE TABLE `sys_role` (
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_role` varchar(255) NOT NULL COMMENT '角色id',
  PRIMARY KEY (`id_role`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='角色';


-- tcdt_rust.sys_token definition

CREATE TABLE `sys_token` (
  `username` varchar(255) DEFAULT NULL COMMENT '用户名称',
  `nick_name` varchar(255) DEFAULT NULL COMMENT '昵称',
  `create_time` timestamp NULL DEFAULT NULL COMMENT '创建时间',
  `token` longtext DEFAULT NULL COMMENT '令牌',
  `expired_time` timestamp NULL DEFAULT NULL COMMENT '过期时间',
  `user_info_string` varchar(4000) DEFAULT NULL COMMENT '用户信息序列化',
  `id_sys_token` varchar(255) NOT NULL COMMENT '令牌主属性',
  PRIMARY KEY (`id_sys_token`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='令牌';


-- tcdt_rust.sys_user definition

CREATE TABLE `sys_user` (
  `account` varchar(255) DEFAULT NULL COMMENT '登录账号 ',
  `user_pwd` varchar(255) DEFAULT NULL COMMENT '用户密码 ',
  `phone` varchar(255) DEFAULT NULL COMMENT '手机号码',
  `email` varchar(255) DEFAULT NULL COMMENT '邮箱',
  `name` varchar(255) DEFAULT NULL COMMENT '姓名 ',
  `nick_name` varchar(255) DEFAULT NULL COMMENT '昵称',
  `gender` varchar(255) DEFAULT NULL COMMENT '性别',
  `fg_active` int(1) DEFAULT NULL COMMENT '启用标志',
  `id_user` varchar(255) NOT NULL COMMENT '系统用户id',
  PRIMARY KEY (`id_user`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='系统用户';


-- tcdt_rust.ui_bill_form definition

CREATE TABLE `ui_bill_form` (
  `content` longtext DEFAULT NULL COMMENT '配置内容:json字符串内容',
  `meta_data` longtext DEFAULT NULL COMMENT '表单配置引用的元数据',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '表单显示名称',
  `bill_form_type` varchar(255) DEFAULT NULL COMMENT '表单类型:Single：单组件；Combination：聚合组件',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id',
  `project_name` varchar(255) DEFAULT NULL COMMENT '项目名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `sub_project_name` varchar(255) DEFAULT NULL COMMENT '子项目名称',
  `id_component_module` varchar(255) DEFAULT NULL COMMENT '组件模块id',
  `component_module_name` varchar(255) DEFAULT NULL COMMENT '组件模块名称',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id',
  `component_name` varchar(255) DEFAULT NULL COMMENT '组件名称',
  `id_bill_form` varchar(255) NOT NULL COMMENT '表单id',
  PRIMARY KEY (`id_bill_form`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='表单配置';


-- tcdt_rust.ui_button_action definition

CREATE TABLE `ui_button_action` (
  `content` longtext DEFAULT NULL COMMENT '配置内容:json字符串内容',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id',
  `project_name` varchar(255) DEFAULT NULL COMMENT '项目名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `sub_project_name` varchar(255) DEFAULT NULL COMMENT '子项目名称',
  `id_button_action` varchar(255) NOT NULL COMMENT '树id',
  PRIMARY KEY (`id_button_action`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='按钮操作';


-- tcdt_rust.ui_factory definition

CREATE TABLE `ui_factory` (
  `content` longtext DEFAULT NULL COMMENT '配置内容:json字符串内容',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id',
  `project_name` varchar(255) DEFAULT NULL COMMENT '项目名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `sub_project_name` varchar(255) DEFAULT NULL COMMENT '子项目名称',
  `ref_id_content` longtext DEFAULT NULL COMMENT '引用组件id内容',
  `id_component_module` varchar(255) DEFAULT NULL COMMENT '组件模块id',
  `component_module_name` varchar(255) DEFAULT NULL COMMENT '组件模块名称',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id',
  `component_name` varchar(255) DEFAULT NULL COMMENT '组件名称',
  `id_factory` varchar(255) NOT NULL COMMENT '工厂id',
  `fg_template` tinyint(4) DEFAULT NULL COMMENT '模板标志',
  PRIMARY KEY (`id_factory`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='UI工厂';


-- tcdt_rust.ui_query definition

CREATE TABLE `ui_query` (
  `content` longtext DEFAULT NULL COMMENT '配置内容:json字符串内容',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id',
  `project_name` varchar(255) DEFAULT NULL COMMENT '项目名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `sub_project_name` varchar(255) DEFAULT NULL COMMENT '子项目名称',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id',
  `component_name` varchar(255) DEFAULT NULL COMMENT '组件名称',
  `id_component_module` varchar(255) DEFAULT NULL COMMENT '组件模块id',
  `component_module_name` varchar(255) DEFAULT NULL COMMENT '组件模块名称',
  `id_query` varchar(255) NOT NULL COMMENT '查询模板id',
  PRIMARY KEY (`id_query`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='查询模板';


-- tcdt_rust.ui_tree definition

CREATE TABLE `ui_tree` (
  `content` longtext DEFAULT NULL COMMENT '配置内容:json字符串内容',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id',
  `project_name` varchar(255) DEFAULT NULL COMMENT '项目名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `sub_project_name` varchar(255) DEFAULT NULL COMMENT '子项目名称',
  `id_component_module` varchar(255) DEFAULT NULL COMMENT '组件模块id',
  `component_module_name` varchar(255) DEFAULT NULL COMMENT '组件模块名称',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件模块id',
  `component_name` varchar(255) DEFAULT NULL COMMENT '组件名称',
  `id_tree` varchar(255) NOT NULL COMMENT '树id',
  PRIMARY KEY (`id_tree`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='树';


-- tcdt_rust.dd_data_type definition

CREATE TABLE `dd_data_type` (
  `code` varchar(255) DEFAULT NULL COMMENT '数据类型编码',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `note` varchar(255) DEFAULT NULL COMMENT '备注',
  `sn` int(11) DEFAULT NULL COMMENT '序列号',
  `len` int(11) DEFAULT NULL COMMENT '长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `column_type` varchar(255) DEFAULT NULL COMMENT '字段类型',
  `object_type` varchar(255) DEFAULT NULL COMMENT '对象类型名称',
  `object_type_package` varchar(255) DEFAULT NULL COMMENT '对象类型包名',
  `ext1` varchar(255) DEFAULT NULL COMMENT '扩展属性1',
  `ext2` varchar(255) DEFAULT NULL COMMENT '扩展属性2',
  `ext3` varchar(255) DEFAULT NULL COMMENT '扩展属性3',
  `ext4` varchar(255) DEFAULT NULL COMMENT '扩展属性4',
  `ext5` varchar(255) DEFAULT NULL COMMENT '扩展属性5',
  `ext6` varchar(255) DEFAULT NULL COMMENT '扩展属性6',
  `default_value` varchar(255) DEFAULT NULL COMMENT '默认值',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '必填标志',
  `type_script_type` varchar(255) DEFAULT NULL COMMENT 'TypeScript类型',
  `web_input_type` varchar(255) DEFAULT NULL COMMENT 'HTML5输入框类型',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id:项目id',
  `id_data_type` varchar(255) NOT NULL COMMENT '数据类型id',
  `fg_preset` tinyint(4) NOT NULL DEFAULT 0 COMMENT '系统预置数据标识',
  PRIMARY KEY (`id_data_type`),
  KEY `FK1hdjmceexb1ddmao25ppo1sl6` (`id_project`),
  CONSTRAINT `FK1hdjmceexb1ddmao25ppo1sl6` FOREIGN KEY (`id_project`) REFERENCES `dd_project` (`id_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='数据类型';


-- tcdt_rust.dd_internal_method definition

CREATE TABLE `dd_internal_method` (
  `code` varchar(255) DEFAULT NULL COMMENT '方法标识符',
  `name` varchar(255) DEFAULT NULL COMMENT '方法名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '方法显示名称',
  `crud` varchar(255) DEFAULT NULL COMMENT 'CRUD',
  `alias` varchar(255) DEFAULT NULL COMMENT '调用名',
  `sn` int(11) DEFAULT NULL COMMENT '排序',
  `note` varchar(255) DEFAULT NULL COMMENT '方法描述',
  `publish_type` varchar(255) DEFAULT NULL COMMENT '方法发布类型:方法发布类型：front：前端 service：服务',
  `entity_type` varchar(255) DEFAULT NULL COMMENT '方法所属实体类型',
  `impl_content` varchar(255) DEFAULT NULL COMMENT '方法实现内容',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id:项目id',
  `id_internal_method` varchar(255) NOT NULL COMMENT '内置方法id',
  PRIMARY KEY (`id_internal_method`),
  KEY `FK3xnxfwhmjm93amrp8hf734nx6` (`id_project`),
  CONSTRAINT `FK3xnxfwhmjm93amrp8hf734nx6` FOREIGN KEY (`id_project`) REFERENCES `dd_project` (`id_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='内置操作方法';


-- tcdt_rust.dd_internal_method_param definition

CREATE TABLE `dd_internal_method_param` (
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '参数显示名称',
  `note` varchar(255) DEFAULT NULL COMMENT '参数描述',
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `number_type` varchar(255) DEFAULT NULL COMMENT '复数类型',
  `fg_agg` int(1) DEFAULT NULL COMMENT '是否聚合',
  `type_style` varchar(255) DEFAULT NULL COMMENT '数据类型样式',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `len` int(11) DEFAULT NULL COMMENT '长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `sn` int(11) DEFAULT NULL COMMENT '排序',
  `id_internal_method` varchar(255) DEFAULT NULL COMMENT '内置方法id:内置方法id',
  `id_param_type` varchar(255) DEFAULT NULL COMMENT '数据类型id:数据类型id',
  `id_internal_method_param` varchar(255) NOT NULL COMMENT '参数id',
  PRIMARY KEY (`id_internal_method_param`),
  KEY `FKtqolurb2aqjr2l8bw38d3iywa` (`id_internal_method`),
  KEY `FK6htjasiroy00l866cq55uvouq` (`id_param_type`),
  CONSTRAINT `FK6htjasiroy00l866cq55uvouq` FOREIGN KEY (`id_param_type`) REFERENCES `dd_data_type` (`id_data_type`),
  CONSTRAINT `FKtqolurb2aqjr2l8bw38d3iywa` FOREIGN KEY (`id_internal_method`) REFERENCES `dd_internal_method` (`id_internal_method`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='内置操作方法参数';


-- tcdt_rust.dd_internal_method_return definition

CREATE TABLE `dd_internal_method_return` (
  `display_name` varchar(255) DEFAULT NULL COMMENT '参数显示名称',
  `note` varchar(255) DEFAULT NULL COMMENT '参数描述',
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `number_type` varchar(255) DEFAULT NULL COMMENT '复数类型',
  `fg_agg` int(1) DEFAULT NULL COMMENT '是否聚合',
  `type_style` varchar(255) DEFAULT NULL COMMENT '数据类型样式',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `len` int(11) DEFAULT NULL COMMENT '长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `id_internal_method` varchar(255) DEFAULT NULL COMMENT '内置方法id:内置方法id',
  `id_param_type` varchar(255) DEFAULT NULL COMMENT '数据类型id:数据类型id',
  `id_internal_method_return` varchar(255) NOT NULL COMMENT '返回参数id',
  PRIMARY KEY (`id_internal_method_return`),
  KEY `FK528w5fuwbtkg7k2kxyfg6t8wg` (`id_internal_method`),
  KEY `FKpxnn5cb3gvm8l8aqmkfvderwa` (`id_param_type`),
  CONSTRAINT `FK528w5fuwbtkg7k2kxyfg6t8wg` FOREIGN KEY (`id_internal_method`) REFERENCES `dd_internal_method` (`id_internal_method`),
  CONSTRAINT `FKpxnn5cb3gvm8l8aqmkfvderwa` FOREIGN KEY (`id_param_type`) REFERENCES `dd_data_type` (`id_data_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='内置操作方法返回参数';


-- tcdt_rust.dd_sub_project definition

CREATE TABLE `dd_sub_project` (
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `path` varchar(255) DEFAULT NULL COMMENT '子系统路径',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id:项目id',
  `id_sub_project` varchar(255) NOT NULL COMMENT '模块id',
  PRIMARY KEY (`id_sub_project`),
  KEY `FKdx6vdwovxc1mq0c0e2snd3gcv` (`id_project`),
  CONSTRAINT `FKdx6vdwovxc1mq0c0e2snd3gcv` FOREIGN KEY (`id_project`) REFERENCES `dd_project` (`id_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='模块';


-- tcdt_rust.dto_module definition

CREATE TABLE `dto_module` (
  `name` varchar(255) DEFAULT NULL COMMENT 'DTO模块名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `path` varchar(255) DEFAULT NULL COMMENT 'DTO模块空间路径',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `id_dto_module` varchar(255) NOT NULL COMMENT 'DTO模块id',
  PRIMARY KEY (`id_dto_module`),
  KEY `FK75bg5s8uyk98t96wcx2rxidx9` (`id_sub_project`),
  CONSTRAINT `FK75bg5s8uyk98t96wcx2rxidx9` FOREIGN KEY (`id_sub_project`) REFERENCES `dd_sub_project` (`id_sub_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO模块';


-- tcdt_rust.sys_menu definition

CREATE TABLE `sys_menu` (
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `fg_show` int(1) DEFAULT NULL COMMENT '显示标志',
  `query` varchar(255) DEFAULT NULL COMMENT '路由参数',
  `menu_type` varchar(255) DEFAULT NULL COMMENT '菜单类型',
  `fg_active` int(1) DEFAULT NULL COMMENT '启用标志',
  `web_perms` varchar(255) DEFAULT NULL COMMENT '前端权限标识',
  `service_perms` varchar(255) DEFAULT NULL COMMENT '后台权限标识',
  `id_parent` varchar(255) DEFAULT NULL COMMENT '上级系统菜单id:上级系统菜单id',
  `id_menu` varchar(255) NOT NULL COMMENT '系统菜单id',
  PRIMARY KEY (`id_menu`),
  KEY `id_parent` (`id_parent`),
  CONSTRAINT `sys_menu_ibfk_1` FOREIGN KEY (`id_parent`) REFERENCES `sys_menu` (`id_menu`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='系统菜单';


-- tcdt_rust.sys_role_menu definition

CREATE TABLE `sys_role_menu` (
  `id_menu` varchar(255) DEFAULT NULL COMMENT '系统菜单id:系统菜单id',
  `id_role` varchar(255) DEFAULT NULL COMMENT '角色id:角色id',
  `id_role_menu` varchar(255) NOT NULL COMMENT '角色与菜单id',
  PRIMARY KEY (`id_role_menu`),
  KEY `id_menu` (`id_menu`),
  KEY `id_role` (`id_role`),
  CONSTRAINT `sys_role_menu_ibfk_1` FOREIGN KEY (`id_menu`) REFERENCES `sys_menu` (`id_menu`),
  CONSTRAINT `sys_role_menu_ibfk_2` FOREIGN KEY (`id_role`) REFERENCES `sys_role` (`id_role`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='角色与菜单';


-- tcdt_rust.sys_user_role definition

CREATE TABLE `sys_user_role` (
  `id_role` varchar(255) DEFAULT NULL COMMENT '角色id:角色id',
  `id_user` varchar(255) DEFAULT NULL COMMENT '系统用户id:系统用户id',
  `id_sys_user_role` varchar(255) NOT NULL COMMENT '用户角色关系主属性',
  PRIMARY KEY (`id_sys_user_role`),
  KEY `id_role` (`id_role`),
  KEY `id_user` (`id_user`),
  CONSTRAINT `sys_user_role_ibfk_1` FOREIGN KEY (`id_role`) REFERENCES `sys_role` (`id_role`),
  CONSTRAINT `sys_user_role_ibfk_2` FOREIGN KEY (`id_user`) REFERENCES `sys_user` (`id_user`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='用户角色关系';


-- tcdt_rust.dd_component_module definition

CREATE TABLE `dd_component_module` (
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `path` varchar(255) DEFAULT NULL COMMENT '组件模块空间路径',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '子项目id',
  `id_component_module` varchar(255) NOT NULL COMMENT '组件模块id',
  PRIMARY KEY (`id_component_module`),
  KEY `FKf3yvpxahvo6cun9awqd7pioql` (`id_sub_project`),
  CONSTRAINT `FKf3yvpxahvo6cun9awqd7pioql` FOREIGN KEY (`id_sub_project`) REFERENCES `dd_sub_project` (`id_sub_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件模块';


-- tcdt_rust.dd_entity_collection definition

CREATE TABLE `dd_entity_collection` (
  `package_name` varchar(255) DEFAULT NULL COMMENT '代码包名',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_sub_project` varchar(255) DEFAULT NULL COMMENT '模块id:模块id',
  `id_entity_collection` varchar(255) NOT NULL COMMENT '实体集id',
  PRIMARY KEY (`id_entity_collection`),
  KEY `FK1cof5oceodwxhx8kbknsu46qi` (`id_sub_project`),
  CONSTRAINT `FK1cof5oceodwxhx8kbknsu46qi` FOREIGN KEY (`id_sub_project`) REFERENCES `dd_sub_project` (`id_sub_project`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='实体集';


-- tcdt_rust.dd_enum definition

CREATE TABLE `dd_enum` (
  `class_name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `enum_value_type` varchar(255) DEFAULT NULL COMMENT '枚举值的类型',
  `id_entity_collection` varchar(255) DEFAULT NULL COMMENT '实体集id:实体集id',
  `id_enum` varchar(255) NOT NULL COMMENT '枚举id',
  PRIMARY KEY (`id_enum`),
  KEY `FK4wi2bwo6li33wu2skw2859p1d` (`id_entity_collection`),
  CONSTRAINT `FK4wi2bwo6li33wu2skw2859p1d` FOREIGN KEY (`id_entity_collection`) REFERENCES `dd_entity_collection` (`id_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='枚举实体';


-- tcdt_rust.dd_enum_attribute definition

CREATE TABLE `dd_enum_attribute` (
  `display_name` varchar(255) DEFAULT NULL COMMENT '枚举属性显示名称:枚举属性显示名称',
  `code` varchar(255) DEFAULT NULL COMMENT '枚举属性编码:枚举属性编码',
  `enum_value` varchar(255) DEFAULT NULL COMMENT '枚举值:枚举值',
  `sn` int(11) DEFAULT NULL COMMENT '序号:序号',
  `id_enum` varchar(255) DEFAULT NULL COMMENT '枚举id:枚举id',
  `id_enum_attribute` varchar(255) NOT NULL COMMENT '枚举属性id:枚举属性id',
  PRIMARY KEY (`id_enum_attribute`),
  KEY `FKagipoc27vfq1uqwpur7fhoebq` (`id_enum`),
  CONSTRAINT `FKagipoc27vfq1uqwpur7fhoebq` FOREIGN KEY (`id_enum`) REFERENCES `dd_enum` (`id_enum`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='枚举属性';


-- tcdt_rust.dd_node_ui definition

CREATE TABLE `dd_node_ui` (
  `x` int(11) DEFAULT NULL COMMENT 'x坐标',
  `y` int(11) DEFAULT NULL COMMENT 'y坐标',
  `width` int(11) DEFAULT NULL COMMENT '宽度',
  `height` int(11) DEFAULT NULL COMMENT '高度',
  `id_element` varchar(255) DEFAULT NULL COMMENT '元素id',
  `id_entity_collection` varchar(255) DEFAULT NULL COMMENT '实体集id:实体集id',
  `id_node_ui` varchar(255) NOT NULL COMMENT 'id',
  PRIMARY KEY (`id_node_ui`),
  KEY `FKhswu7erevj8cqe8bbg6gkonrm` (`id_entity_collection`),
  CONSTRAINT `FKhswu7erevj8cqe8bbg6gkonrm` FOREIGN KEY (`id_entity_collection`) REFERENCES `dd_entity_collection` (`id_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='ui信息';


-- tcdt_rust.dto_entity_collection definition

CREATE TABLE `dto_entity_collection` (
  `package_name` varchar(255) DEFAULT NULL COMMENT '代码包名',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `id_main_dto_entity` varchar(255) DEFAULT NULL COMMENT '主DTO实体集id',
  `id_dto_module` varchar(255) DEFAULT NULL COMMENT 'DTO模块id:DTO模块id',
  `id_dto_entity_collection` varchar(255) NOT NULL COMMENT 'DTO实体集id',
  PRIMARY KEY (`id_dto_entity_collection`),
  KEY `FKbh0vj0oaamd0ukn2tex3v193j` (`id_dto_module`),
  CONSTRAINT `FKbh0vj0oaamd0ukn2tex3v193j` FOREIGN KEY (`id_dto_module`) REFERENCES `dto_module` (`id_dto_module`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO实体集';


-- tcdt_rust.dto_enum definition

CREATE TABLE `dto_enum` (
  `class_name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `enum_value_type` varchar(255) DEFAULT NULL COMMENT '枚举值的类型',
  `id_ref` varchar(255) DEFAULT NULL COMMENT '引用id',
  `id_dto_entity_collection` varchar(255) DEFAULT NULL COMMENT 'DTO实体集id:DTO实体集id',
  `id_dto_enum` varchar(255) NOT NULL COMMENT 'DTO枚举id',
  PRIMARY KEY (`id_dto_enum`),
  KEY `FK75t372hgqd9l34nq21qrn1e4l` (`id_dto_entity_collection`),
  CONSTRAINT `FK75t372hgqd9l34nq21qrn1e4l` FOREIGN KEY (`id_dto_entity_collection`) REFERENCES `dto_entity_collection` (`id_dto_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO枚举实体';


-- tcdt_rust.dto_enum_attribute definition

CREATE TABLE `dto_enum_attribute` (
  `display_name` varchar(255) DEFAULT NULL COMMENT '枚举属性显示名称',
  `code` varchar(255) DEFAULT NULL COMMENT '枚举属性编码',
  `enum_value` varchar(255) DEFAULT NULL COMMENT '枚举值',
  `sn` int(11) DEFAULT NULL COMMENT '序号',
  `id_ref` varchar(255) DEFAULT NULL COMMENT '引用id',
  `id_dto_enum` varchar(255) DEFAULT NULL COMMENT 'DTO枚举id:DTO枚举id',
  `id_dto_enum_attribute` varchar(255) NOT NULL COMMENT 'DTO枚举属性id',
  PRIMARY KEY (`id_dto_enum_attribute`),
  KEY `FK5yeqplqt7sfd3wruwr8f7c4ni` (`id_dto_enum`),
  CONSTRAINT `FK5yeqplqt7sfd3wruwr8f7c4ni` FOREIGN KEY (`id_dto_enum`) REFERENCES `dto_enum` (`id_dto_enum`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO枚举属性';


-- tcdt_rust.dto_node_ui definition

CREATE TABLE `dto_node_ui` (
  `x` int(11) DEFAULT NULL COMMENT 'x坐标',
  `y` int(11) DEFAULT NULL COMMENT 'y坐标',
  `width` int(11) DEFAULT NULL COMMENT '宽度',
  `height` int(11) DEFAULT NULL COMMENT '高度',
  `id_element` varchar(255) DEFAULT NULL COMMENT '元素id',
  `id_dto_entity_collection` varchar(255) DEFAULT NULL COMMENT 'DTO实体集id:DTO实体集id',
  `id_dto_node_ui` varchar(255) NOT NULL COMMENT 'DTO实体集ui信息id',
  PRIMARY KEY (`id_dto_node_ui`),
  KEY `FKiyg2j0cpst5vpod4n5nrn0j05` (`id_dto_entity_collection`),
  CONSTRAINT `FKiyg2j0cpst5vpod4n5nrn0j05` FOREIGN KEY (`id_dto_entity_collection`) REFERENCES `dto_entity_collection` (`id_dto_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO实体集ui信息';


-- tcdt_rust.dd_component definition

CREATE TABLE `dd_component` (
  `id_main_component_entity` varchar(255) DEFAULT NULL COMMENT '主实体id',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `package_name` varchar(255) DEFAULT NULL COMMENT '包名',
  `component_type` varchar(255) DEFAULT NULL COMMENT '组件类型',
  `id_component_module` varchar(255) DEFAULT NULL COMMENT '组件模块id:组件模块id',
  `id_component` varchar(255) NOT NULL COMMENT '组件id',
  PRIMARY KEY (`id_component`),
  KEY `FK5tpkuu3mmu6w71u6vlgleerg9` (`id_component_module`),
  CONSTRAINT `FK5tpkuu3mmu6w71u6vlgleerg9` FOREIGN KEY (`id_component_module`) REFERENCES `dd_component_module` (`id_component_module`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件';


-- tcdt_rust.dd_component_enum definition

CREATE TABLE `dd_component_enum` (
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id:组件id',
  `id_enum` varchar(255) DEFAULT NULL COMMENT '枚举id:枚举id',
  `id_component_enum` varchar(255) NOT NULL COMMENT '组件枚举id',
  PRIMARY KEY (`id_component_enum`),
  KEY `FKoqum1ywd5svqggnhbg1j8igiw` (`id_component`),
  KEY `FKh5x8yy3kyk2lhfcungtunqtxw` (`id_enum`),
  CONSTRAINT `FKh5x8yy3kyk2lhfcungtunqtxw` FOREIGN KEY (`id_enum`) REFERENCES `dd_enum` (`id_enum`),
  CONSTRAINT `FKoqum1ywd5svqggnhbg1j8igiw` FOREIGN KEY (`id_component`) REFERENCES `dd_component` (`id_component`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件枚举';


-- tcdt_rust.dd_component_eo_collection definition

CREATE TABLE `dd_component_eo_collection` (
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `base_path` varchar(255) DEFAULT NULL COMMENT '服务名称空间',
  `id_component_module` varchar(255) DEFAULT NULL COMMENT '组件模块id:组件模块id',
  `id_component_eo_collection` varchar(255) NOT NULL COMMENT '组件操作集id',
  PRIMARY KEY (`id_component_eo_collection`),
  KEY `FKoqaydxcexlbrjj1ck65k2w1gt` (`id_component_module`),
  CONSTRAINT `FKoqaydxcexlbrjj1ck65k2w1gt` FOREIGN KEY (`id_component_module`) REFERENCES `dd_component_module` (`id_component_module`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='操作实体集';


-- tcdt_rust.dd_component_eo_ui definition

CREATE TABLE `dd_component_eo_ui` (
  `x` int(11) DEFAULT NULL COMMENT 'x坐标',
  `y` int(11) DEFAULT NULL COMMENT 'y坐标',
  `width` int(11) DEFAULT NULL COMMENT '宽度',
  `height` int(11) DEFAULT NULL COMMENT '高度',
  `id_element` varchar(255) DEFAULT NULL COMMENT '元素id',
  `id_component_eo_collection` varchar(255) DEFAULT NULL COMMENT '组件操作集id:组件操作集id',
  `id_component_eo_node_ui` varchar(255) NOT NULL COMMENT '组件操作实体ui信息id',
  PRIMARY KEY (`id_component_eo_node_ui`),
  KEY `FKi8olpoccdc9x29tmljebr30yj` (`id_component_eo_collection`),
  CONSTRAINT `FKi8olpoccdc9x29tmljebr30yj` FOREIGN KEY (`id_component_eo_collection`) REFERENCES `dd_component_eo_collection` (`id_component_eo_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件操作实体ui信息';


-- tcdt_rust.dd_component_node_ui definition

CREATE TABLE `dd_component_node_ui` (
  `x` int(11) DEFAULT NULL COMMENT 'x坐标',
  `y` int(11) DEFAULT NULL COMMENT 'y坐标',
  `width` int(11) DEFAULT NULL COMMENT '宽度',
  `height` int(11) DEFAULT NULL COMMENT '高度',
  `id_element` varchar(255) DEFAULT NULL COMMENT '元素id',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id:组件id',
  `id_component_node_ui` varchar(255) NOT NULL COMMENT 'id',
  PRIMARY KEY (`id_component_node_ui`),
  KEY `FKjma2n90p3sd36xnfu5mb5sdes` (`id_component`),
  CONSTRAINT `FKjma2n90p3sd36xnfu5mb5sdes` FOREIGN KEY (`id_component`) REFERENCES `dd_component` (`id_component`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='ui信息';


-- tcdt_rust.dd_entity definition

CREATE TABLE `dd_entity` (
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `class_name` varchar(255) DEFAULT NULL COMMENT '类名',
  `table_name` varchar(255) DEFAULT NULL COMMENT '表名',
  `pk_attribute_code` varchar(255) DEFAULT NULL COMMENT '主属性code',
  `pk_attribute_name` varchar(255) DEFAULT NULL COMMENT '主属性名称',
  `pk_attribute_type_name` varchar(255) DEFAULT NULL COMMENT '主属性类型名称',
  `id_entity_collection` varchar(255) DEFAULT NULL COMMENT '实体集id:实体集id',
  `id_entity` varchar(255) NOT NULL COMMENT '实体id',
  PRIMARY KEY (`id_entity`),
  KEY `FKkws3va4v8s8ys3988digljkfm` (`id_entity_collection`),
  CONSTRAINT `FKkws3va4v8s8ys3988digljkfm` FOREIGN KEY (`id_entity_collection`) REFERENCES `dd_entity_collection` (`id_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='实体信息';


-- tcdt_rust.dd_entity_associate definition

CREATE TABLE `dd_entity_associate` (
  `group_order` int(11) DEFAULT NULL COMMENT '两个实体多条连线时，连线的序号',
  `up_associate_type` varchar(255) DEFAULT NULL COMMENT '上级关系',
  `down_associate_type` varchar(255) DEFAULT NULL COMMENT '下级关系',
  `down_attribute_name` varchar(255) DEFAULT NULL COMMENT '下级实体属性名称',
  `down_attribute_display_name` varchar(255) DEFAULT NULL COMMENT '下级实体属性显示名称',
  `ref_attribute_name` varchar(255) DEFAULT NULL COMMENT '引用实体属性',
  `ref_attribute_display_name` varchar(255) DEFAULT NULL COMMENT '引用实体属性显示名称',
  `fk_column_name` varchar(255) DEFAULT NULL COMMENT '外键字段名称',
  `fk_attribute_name` varchar(255) DEFAULT NULL COMMENT '外键属性',
  `fk_attribute_display_name` varchar(255) DEFAULT NULL COMMENT '外键属性显示名称',
  `fg_foreign_key` int(1) DEFAULT NULL COMMENT '是否建立物理外键',
  `down_order_str` varchar(255) DEFAULT NULL COMMENT '下级实体排序',
  `down_batch_size` int(11) DEFAULT NULL COMMENT '批量获取下级实体数量',
  `id_down` varchar(255) DEFAULT NULL COMMENT '下级实体id:下级实体id',
  `id_entity_collection` varchar(255) DEFAULT NULL COMMENT '实体集id:实体集id',
  `id_up` varchar(255) DEFAULT NULL COMMENT '上级实体id:上级实体id',
  `id_entity_associate` varchar(255) NOT NULL COMMENT '实体连线id',
  PRIMARY KEY (`id_entity_associate`),
  KEY `FKcy23461ikjtf1f2lb1oyso9pp` (`id_down`),
  KEY `FKpjlvuk2r964hqalqsxbplkeli` (`id_entity_collection`),
  KEY `FK3e2kfh68icgu15nwr5xutnls5` (`id_up`),
  CONSTRAINT `FK3e2kfh68icgu15nwr5xutnls5` FOREIGN KEY (`id_up`) REFERENCES `dd_entity` (`id_entity`),
  CONSTRAINT `FKcy23461ikjtf1f2lb1oyso9pp` FOREIGN KEY (`id_down`) REFERENCES `dd_entity` (`id_entity`),
  CONSTRAINT `FKpjlvuk2r964hqalqsxbplkeli` FOREIGN KEY (`id_entity_collection`) REFERENCES `dd_entity_collection` (`id_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='关系连线';


-- tcdt_rust.dd_entity_attribute definition

CREATE TABLE `dd_entity_attribute` (
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `column_name` varchar(255) DEFAULT NULL COMMENT '字段名称',
  `fg_primary_key` int(1) DEFAULT NULL COMMENT '是否主键',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `default_value` varchar(255) DEFAULT NULL COMMENT '默认值',
  `len` int(11) DEFAULT NULL COMMENT '数据长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `sn` int(11) DEFAULT NULL COMMENT '序号',
  `note` varchar(255) DEFAULT NULL COMMENT '备注',
  `category` varchar(255) DEFAULT NULL COMMENT '分类',
  `id_attribute_type` varchar(255) DEFAULT NULL COMMENT '数据类型id:数据类型id',
  `id_entity` varchar(255) DEFAULT NULL COMMENT '实体id:实体id',
  `id_attribute` varchar(255) NOT NULL COMMENT '属性id',
  PRIMARY KEY (`id_attribute`),
  KEY `FKcqmeyqmb5uxh6u3f6pbud046m` (`id_entity`),
  CONSTRAINT `FKcqmeyqmb5uxh6u3f6pbud046m` FOREIGN KEY (`id_entity`) REFERENCES `dd_entity` (`id_entity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='属性';


-- tcdt_rust.dd_enum_associate definition

CREATE TABLE `dd_enum_associate` (
  `group_order` int(11) DEFAULT NULL COMMENT '两个相同实体和枚举多条连线时，连线的序号',
  `id_attribute` varchar(255) DEFAULT NULL COMMENT '属性id:属性id',
  `id_entity` varchar(255) DEFAULT NULL COMMENT '实体id:实体id',
  `id_enum` varchar(255) DEFAULT NULL COMMENT '枚举id:枚举id',
  `id_entity_collection` varchar(255) DEFAULT NULL COMMENT '实体集id:实体集id',
  `id_enum_associate` varchar(255) NOT NULL COMMENT '枚举关系id',
  PRIMARY KEY (`id_enum_associate`),
  KEY `FKak0q6yiukimumxfrn1cbnxpb9` (`id_attribute`),
  KEY `FKpmycpn9nciahoupihus78wang` (`id_entity`),
  KEY `FKsoi77dktg8qrmvt7yai4ip7d0` (`id_enum`),
  KEY `FK5mw0e037s1al8ot40k6oefuq8` (`id_entity_collection`),
  CONSTRAINT `FK5mw0e037s1al8ot40k6oefuq8` FOREIGN KEY (`id_entity_collection`) REFERENCES `dd_entity_collection` (`id_entity_collection`),
  CONSTRAINT `FKak0q6yiukimumxfrn1cbnxpb9` FOREIGN KEY (`id_attribute`) REFERENCES `dd_entity_attribute` (`id_attribute`),
  CONSTRAINT `FKpmycpn9nciahoupihus78wang` FOREIGN KEY (`id_entity`) REFERENCES `dd_entity` (`id_entity`),
  CONSTRAINT `FKsoi77dktg8qrmvt7yai4ip7d0` FOREIGN KEY (`id_enum`) REFERENCES `dd_enum` (`id_enum`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='实体枚举关系';


-- tcdt_rust.dto_entity definition

CREATE TABLE `dto_entity` (
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `class_name` varchar(255) DEFAULT NULL COMMENT '类名',
  `table_name` varchar(255) DEFAULT NULL COMMENT '表名',
  `pk_attribute_code` varchar(255) DEFAULT NULL COMMENT '主属性code',
  `pk_attribute_name` varchar(255) DEFAULT NULL COMMENT '主属性名称',
  `pk_attribute_type_name` varchar(255) DEFAULT NULL COMMENT '主属性类型名称',
  `id_dto_entity_collection` varchar(255) DEFAULT NULL COMMENT 'DTO实体集id:DTO实体集id',
  `id_ref` varchar(255) DEFAULT NULL COMMENT '引用实体id:引用实体id',
  `id_dto_entity` varchar(255) NOT NULL COMMENT ' DTO实体信息id',
  PRIMARY KEY (`id_dto_entity`),
  KEY `FK1x3y2go8ala8gc2odub2t8aq7` (`id_dto_entity_collection`),
  CONSTRAINT `FK1x3y2go8ala8gc2odub2t8aq7` FOREIGN KEY (`id_dto_entity_collection`) REFERENCES `dto_entity_collection` (`id_dto_entity_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO实体信息';


-- tcdt_rust.dto_entity_associate definition

CREATE TABLE `dto_entity_associate` (
  `group_order` int(11) DEFAULT NULL COMMENT '两个实体多条连线时，连线的序号',
  `up_associate_type` varchar(255) DEFAULT NULL COMMENT '上级关系',
  `down_associate_type` varchar(255) DEFAULT NULL COMMENT '下级关系',
  `down_attribute_name` varchar(255) DEFAULT NULL COMMENT '下级实体属性名称',
  `down_attribute_display_name` varchar(255) DEFAULT NULL COMMENT '下级实体属性显示名称',
  `ref_attribute_name` varchar(255) DEFAULT NULL COMMENT '引用实体属性',
  `ref_attribute_display_name` varchar(255) DEFAULT NULL COMMENT '引用实体属性显示名称',
  `fk_column_name` varchar(255) DEFAULT NULL COMMENT '外键字段名称',
  `fk_attribute_name` varchar(255) DEFAULT NULL COMMENT '外键属性',
  `fk_attribute_display_name` varchar(255) DEFAULT NULL COMMENT '外键属性显示名称',
  `id_down` varchar(255) DEFAULT NULL COMMENT '下级实体id:下级实体id',
  `id_dto_entity_collection` varchar(255) DEFAULT NULL COMMENT 'DTO实体集id:DTO实体集id',
  `id_up` varchar(255) DEFAULT NULL COMMENT '上级实体id:上级实体id',
  `id_dto_entity_associate` varchar(255) NOT NULL COMMENT 'DTO关系连线id',
  PRIMARY KEY (`id_dto_entity_associate`),
  KEY `FKnpu939664odxi8vxhl2bswl4w` (`id_down`),
  KEY `FKhe7nvcxgk4tjb3a7mp6jvualr` (`id_dto_entity_collection`),
  KEY `FKeum4kmw5mj6w7j0afivjr4foh` (`id_up`),
  CONSTRAINT `FKeum4kmw5mj6w7j0afivjr4foh` FOREIGN KEY (`id_up`) REFERENCES `dto_entity` (`id_dto_entity`),
  CONSTRAINT `FKhe7nvcxgk4tjb3a7mp6jvualr` FOREIGN KEY (`id_dto_entity_collection`) REFERENCES `dto_entity_collection` (`id_dto_entity_collection`),
  CONSTRAINT `FKnpu939664odxi8vxhl2bswl4w` FOREIGN KEY (`id_down`) REFERENCES `dto_entity` (`id_dto_entity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO关系连线';


-- tcdt_rust.dto_entity_attribute definition

CREATE TABLE `dto_entity_attribute` (
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `column_name` varchar(255) DEFAULT NULL COMMENT '字段名称',
  `fg_primary_key` int(1) DEFAULT NULL COMMENT '是否主键',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `default_value` varchar(255) DEFAULT NULL COMMENT '默认值',
  `len` int(11) DEFAULT NULL COMMENT '数据长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `sn` int(11) DEFAULT NULL COMMENT '序号',
  `note` varchar(255) DEFAULT NULL COMMENT '备注',
  `category` varchar(255) DEFAULT NULL COMMENT '类型',
  `id_attribute_type` varchar(255) DEFAULT NULL COMMENT '数据类型id:数据类型id',
  `id_dto_entity` varchar(255) DEFAULT NULL COMMENT ' DTO实体信息id: DTO实体信息id',
  `id_ref_attribute` varchar(255) DEFAULT NULL COMMENT '引用属性id:引用属性id',
  `id_dto_entity_attribute` varchar(255) NOT NULL COMMENT 'DTO实体属性id',
  PRIMARY KEY (`id_dto_entity_attribute`),
  KEY `FKl56pfwwtkuftqmtr6da5ognyi` (`id_attribute_type`),
  KEY `FK5w0gwwd1efso8dgwp9on4aqrb` (`id_dto_entity`),
  CONSTRAINT `FK5w0gwwd1efso8dgwp9on4aqrb` FOREIGN KEY (`id_dto_entity`) REFERENCES `dto_entity` (`id_dto_entity`),
  CONSTRAINT `FKl56pfwwtkuftqmtr6da5ognyi` FOREIGN KEY (`id_attribute_type`) REFERENCES `dd_data_type` (`id_data_type`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;


-- tcdt_rust.dto_enum_associate definition

CREATE TABLE `dto_enum_associate` (
  `group_order` int(11) DEFAULT NULL COMMENT '两个相同实体和枚举多条连线时，连线的序号',
  `id_dto_entity` varchar(255) DEFAULT NULL COMMENT ' DTO实体信息id: DTO实体信息id',
  `id_dto_entity_attribute` varchar(255) DEFAULT NULL COMMENT 'DTO实体属性id:DTO实体属性id',
  `id_dto_entity_collection` varchar(255) DEFAULT NULL COMMENT 'DTO实体集id:DTO实体集id',
  `id_dto_enum` varchar(255) DEFAULT NULL COMMENT 'DTO枚举id:DTO枚举id',
  `id_dto_enum_associate` varchar(255) NOT NULL COMMENT 'DTO枚举关系id',
  PRIMARY KEY (`id_dto_enum_associate`),
  KEY `FK5ku4f3lqd9cgxrm6wqoq523vj` (`id_dto_entity`),
  KEY `FKg31lgvcpi8wbysbh3jpq6sai3` (`id_dto_entity_collection`),
  KEY `FKhleurxokw2q5fr5f5d90hk905` (`id_dto_enum`),
  KEY `FKnnhf88dcx1kr1yy5e3rvro8tv` (`id_dto_entity_attribute`),
  CONSTRAINT `FK5ku4f3lqd9cgxrm6wqoq523vj` FOREIGN KEY (`id_dto_entity`) REFERENCES `dto_entity` (`id_dto_entity`),
  CONSTRAINT `FKg31lgvcpi8wbysbh3jpq6sai3` FOREIGN KEY (`id_dto_entity_collection`) REFERENCES `dto_entity_collection` (`id_dto_entity_collection`),
  CONSTRAINT `FKhleurxokw2q5fr5f5d90hk905` FOREIGN KEY (`id_dto_enum`) REFERENCES `dto_enum` (`id_dto_enum`),
  CONSTRAINT `FKnnhf88dcx1kr1yy5e3rvro8tv` FOREIGN KEY (`id_dto_entity_attribute`) REFERENCES `dto_entity_attribute` (`id_dto_entity_attribute`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO实体枚举关系';


-- tcdt_rust.dd_common_attribute definition

CREATE TABLE `dd_common_attribute` (
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `column_name` varchar(255) DEFAULT NULL COMMENT '字段名称',
  `default_value` varchar(255) DEFAULT NULL COMMENT '默认值',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `len` int(11) DEFAULT NULL COMMENT '数据长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `sn` int(11) DEFAULT NULL COMMENT '序号',
  `ref_attribute_name` varchar(255) DEFAULT NULL COMMENT '引用属性名称',
  `ref_display_name` varchar(255) DEFAULT NULL COMMENT '引用属性显示名称',
  `category` varchar(255) DEFAULT NULL COMMENT '属性类别',
  `id_data_type` varchar(255) DEFAULT NULL COMMENT '数据类型id:数据类型id',
  `id_project` varchar(255) DEFAULT NULL COMMENT '项目id:项目id',
  `id_ref_entity` varchar(255) DEFAULT NULL COMMENT '上级实体id:上级实体id',
  `id_common_attribute` varchar(255) NOT NULL COMMENT '属性id',
  `fg_preset` tinyint(4) NOT NULL DEFAULT 0 COMMENT '系统预置数据标识',
  PRIMARY KEY (`id_common_attribute`),
  KEY `FKdm16p5rm0w9g1cwvjtf0dffox` (`id_ref_entity`),
  KEY `FKd5ap50j4e7s3n23b0rgc5rs5s` (`id_data_type`),
  KEY `FK4803iihy3wbgkljua0kls43u` (`id_project`),
  CONSTRAINT `FK4803iihy3wbgkljua0kls43u` FOREIGN KEY (`id_project`) REFERENCES `dd_project` (`id_project`),
  CONSTRAINT `FKd5ap50j4e7s3n23b0rgc5rs5s` FOREIGN KEY (`id_data_type`) REFERENCES `dd_data_type` (`id_data_type`),
  CONSTRAINT `FKdm16p5rm0w9g1cwvjtf0dffox` FOREIGN KEY (`id_ref_entity`) REFERENCES `dd_entity` (`id_entity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='公共属性';


-- tcdt_rust.dd_component_entity definition

CREATE TABLE `dd_component_entity` (
  `fg_virtual` int(1) DEFAULT NULL COMMENT '虚拟实体标志:组合实体组件下的组件实体都是虚拟虚拟实体',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id:组件id',
  `id_entity` varchar(255) DEFAULT NULL COMMENT '实体id:实体id',
  `id_component_entity` varchar(255) NOT NULL COMMENT '组件实体id',
  PRIMARY KEY (`id_component_entity`),
  KEY `FKsiogifnexpkmkiq97p9mgute9` (`id_component`),
  KEY `FKj36a9apjq9fnrdae4ytqgvqpo` (`id_entity`),
  CONSTRAINT `FKj36a9apjq9fnrdae4ytqgvqpo` FOREIGN KEY (`id_entity`) REFERENCES `dd_entity` (`id_entity`),
  CONSTRAINT `FKsiogifnexpkmkiq97p9mgute9` FOREIGN KEY (`id_component`) REFERENCES `dd_component` (`id_component`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件实体';


-- tcdt_rust.dd_component_entity_associate definition

CREATE TABLE `dd_component_entity_associate` (
  `down_package_name` varchar(255) DEFAULT NULL COMMENT '下级实体包名',
  `up_package_name` varchar(255) DEFAULT NULL COMMENT '上级实体包名',
  `fg_agg_asso` int(1) DEFAULT NULL COMMENT '是否agg关系连线',
  `id_component` varchar(255) DEFAULT NULL COMMENT '组件id:组件id',
  `id_down_cp_entity` varchar(255) DEFAULT NULL COMMENT '下级组件实体id:下级组件实体id',
  `id_entity_associate` varchar(255) DEFAULT NULL COMMENT 'id:id',
  `id_up_cp_entity` varchar(255) DEFAULT NULL COMMENT '上级组件实体id:上级组件实体id',
  `id_component_entity_associate` varchar(255) NOT NULL COMMENT '组件关系id',
  PRIMARY KEY (`id_component_entity_associate`),
  KEY `FKot1an5gwnumcbv7jjf317pa49` (`id_component`),
  KEY `FK1qhhwu40dbmgr5geihysafj5y` (`id_down_cp_entity`),
  KEY `FKb9jcm9obpb1hxabav8fx6f4h8` (`id_entity_associate`),
  KEY `FKdfun353xixq2ut9k5iwfa8vbf` (`id_up_cp_entity`),
  CONSTRAINT `FK1qhhwu40dbmgr5geihysafj5y` FOREIGN KEY (`id_down_cp_entity`) REFERENCES `dd_component_entity` (`id_component_entity`),
  CONSTRAINT `FKb9jcm9obpb1hxabav8fx6f4h8` FOREIGN KEY (`id_entity_associate`) REFERENCES `dd_entity_associate` (`id_entity_associate`),
  CONSTRAINT `FKdfun353xixq2ut9k5iwfa8vbf` FOREIGN KEY (`id_up_cp_entity`) REFERENCES `dd_component_entity` (`id_component_entity`),
  CONSTRAINT `FKot1an5gwnumcbv7jjf317pa49` FOREIGN KEY (`id_component`) REFERENCES `dd_component` (`id_component`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件关系';


-- tcdt_rust.dd_component_eo definition

CREATE TABLE `dd_component_eo` (
  `name` varchar(255) DEFAULT NULL COMMENT '组件操作名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `class_name` varchar(255) DEFAULT NULL COMMENT '类名',
  `des` varchar(255) DEFAULT NULL COMMENT '描述',
  `sn` int(11) DEFAULT NULL COMMENT '排序',
  `alias` varchar(255) DEFAULT NULL COMMENT '调用名',
  `entity_type` varchar(255) DEFAULT NULL COMMENT '实体类别',
  `component_base_path` varchar(255) DEFAULT NULL COMMENT '组件基础路径',
  `component_entity_class_name` varchar(255) DEFAULT NULL COMMENT '组件实体类名',
  `id_component_entity` varchar(255) DEFAULT NULL COMMENT '组件实体id:组件实体id',
  `id_component_eo_collection` varchar(255) DEFAULT NULL COMMENT '组件操作集id:组件操作集id',
  `id_component_eo` varchar(255) NOT NULL COMMENT '组件实体操作id',
  `sy_dt_create` varchar(255) DEFAULT NULL,
  `sy_dt_update` varchar(255) DEFAULT NULL,
  `sy_id_user_create` varchar(255) DEFAULT NULL,
  `sy_id_user_update` varchar(255) DEFAULT NULL,
  `sy_mvcc` int(11) DEFAULT NULL,
  `sy_na_user_create` varchar(255) DEFAULT NULL,
  `sy_na_user_update` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id_component_eo`),
  KEY `FKh66wgoeokxp865gwc8f929olb` (`id_component_entity`),
  KEY `FKse60iav0n04jrab3dj58pj2pa` (`id_component_eo_collection`),
  CONSTRAINT `FKh66wgoeokxp865gwc8f929olb` FOREIGN KEY (`id_component_entity`) REFERENCES `dd_component_entity` (`id_component_entity`),
  CONSTRAINT `FKse60iav0n04jrab3dj58pj2pa` FOREIGN KEY (`id_component_eo_collection`) REFERENCES `dd_component_eo_collection` (`id_component_eo_collection`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件实体操作';


-- tcdt_rust.dd_component_eo_method definition

CREATE TABLE `dd_component_eo_method` (
  `name` varchar(255) DEFAULT NULL COMMENT '方法名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '方法显示名称',
  `crud` varchar(255) DEFAULT NULL COMMENT 'CRUD',
  `alias` varchar(255) DEFAULT NULL COMMENT '调用名',
  `sn` int(11) DEFAULT NULL COMMENT '排序',
  `note` varchar(255) DEFAULT NULL COMMENT '方法描述',
  `type` varchar(255) DEFAULT NULL COMMENT '类型:类型：front：前端 service：服务',
  `code` varchar(255) DEFAULT NULL COMMENT '编码',
  `fg_internal` varchar(255) DEFAULT NULL COMMENT '内置方法标志',
  `impl_content` varchar(255) DEFAULT NULL COMMENT '实现内容',
  `id_component_eo` varchar(255) DEFAULT NULL COMMENT '组件实体操作id:组件实体操作id',
  `id_component_eo_method` varchar(255) NOT NULL COMMENT '方法id',
  PRIMARY KEY (`id_component_eo_method`),
  KEY `FKhebvhhx58rwpoyoo8cnf0jtq4` (`id_component_eo`),
  CONSTRAINT `FKhebvhhx58rwpoyoo8cnf0jtq4` FOREIGN KEY (`id_component_eo`) REFERENCES `dd_component_eo` (`id_component_eo`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件实体操作方法';


-- tcdt_rust.dd_component_eo_method_param definition

CREATE TABLE `dd_component_eo_method_param` (
  `name` varchar(255) DEFAULT NULL COMMENT '参数名称',
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `number_style` varchar(255) DEFAULT NULL COMMENT '复数类型',
  `fg_agg` int(1) DEFAULT NULL COMMENT '是否聚合',
  `note` varchar(255) DEFAULT NULL COMMENT '参数描述',
  `id_param_type` varchar(255) DEFAULT NULL COMMENT '参数类型id',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `len` int(11) DEFAULT NULL COMMENT '长度',
  `sn` int(11) DEFAULT NULL COMMENT '排序',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `full_class_name` varchar(255) DEFAULT NULL COMMENT '返回类型全路径',
  `entity_type_package` varchar(255) DEFAULT NULL COMMENT '对象类型包名',
  `entity_class_name` varchar(255) DEFAULT NULL COMMENT '返回类名称',
  `type_style` varchar(255) DEFAULT NULL COMMENT '数据类型样式',
  `id_component_eo_method` varchar(255) DEFAULT NULL COMMENT '方法id:方法id',
  `id_component_eo_method_param` varchar(255) NOT NULL COMMENT '参数id',
  PRIMARY KEY (`id_component_eo_method_param`),
  KEY `FKdfndl4ll7591jihq00p92u8s5` (`id_component_eo_method`),
  CONSTRAINT `FKdfndl4ll7591jihq00p92u8s5` FOREIGN KEY (`id_component_eo_method`) REFERENCES `dd_component_eo_method` (`id_component_eo_method`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件实体操作方法参数';


-- tcdt_rust.dd_component_eo_method_return definition

CREATE TABLE `dd_component_eo_method_return` (
  `name` varchar(255) DEFAULT NULL COMMENT '参数名称',
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `number_style` varchar(255) DEFAULT NULL COMMENT '复数类型',
  `fg_agg` int(1) DEFAULT NULL COMMENT '是否聚合',
  `note` varchar(255) DEFAULT NULL COMMENT '参数描述',
  `id_param_type` varchar(255) DEFAULT NULL COMMENT '参数类型id',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `len` int(11) DEFAULT NULL COMMENT '长度',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `full_class_name` varchar(255) DEFAULT NULL COMMENT '参数类型全路径',
  `entity_type_package` varchar(255) DEFAULT NULL COMMENT '对象类型包名',
  `entity_class_name` varchar(255) DEFAULT NULL COMMENT '参数类名称',
  `type_style` varchar(255) DEFAULT NULL COMMENT '数据类型样式',
  `id_component_eo_method` varchar(255) DEFAULT NULL COMMENT '方法id:方法id',
  `id_component_eo_method_return` varchar(255) NOT NULL COMMENT '返回参数id',
  PRIMARY KEY (`id_component_eo_method_return`),
  KEY `FKk7xh6c48iffvuv8k332libe9h` (`id_component_eo_method`),
  CONSTRAINT `FKk7xh6c48iffvuv8k332libe9h` FOREIGN KEY (`id_component_eo_method`) REFERENCES `dd_component_eo_method` (`id_component_eo_method`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件实体操作方法返回参数';


-- tcdt_rust.dd_computation_attribute definition

CREATE TABLE `dd_computation_attribute` (
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `len` varchar(255) DEFAULT NULL COMMENT '数据长度',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `default_value` varchar(255) DEFAULT NULL COMMENT '默认值',
  `pcs` int(11) DEFAULT NULL COMMENT '精度',
  `sn` int(11) DEFAULT NULL COMMENT '序号',
  `id_attribute_type` varchar(255) DEFAULT NULL COMMENT '属性类型:属性类型',
  `id_component_entity` varchar(255) DEFAULT NULL COMMENT '所在组件实体id:所在组件实体id',
  `id_computation_attribute` varchar(255) NOT NULL COMMENT '计算属性id:属性id',
  PRIMARY KEY (`id_computation_attribute`),
  KEY `FKk6f06ftxkqbkb777cluf2s6ui` (`id_attribute_type`),
  KEY `FKsieh47296jyx2y26a0i69e85g` (`id_component_entity`),
  CONSTRAINT `FKk6f06ftxkqbkb777cluf2s6ui` FOREIGN KEY (`id_attribute_type`) REFERENCES `dd_data_type` (`id_data_type`),
  CONSTRAINT `FKsieh47296jyx2y26a0i69e85g` FOREIGN KEY (`id_component_entity`) REFERENCES `dd_component_entity` (`id_component_entity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='计算属性';


-- tcdt_rust.dd_ext_attribute definition

CREATE TABLE `dd_ext_attribute` (
  `id_attribute` varchar(255) DEFAULT NULL COMMENT '关联实体属性id:关联实体属性id',
  `ext1` varchar(255) DEFAULT NULL COMMENT '扩展字段1:扩展字段1',
  `id_component_entity` varchar(255) DEFAULT NULL COMMENT '组件实体id:组件实体id',
  `sn` int(11) DEFAULT NULL COMMENT '排序',
  `id_ext_attribute` varchar(255) NOT NULL COMMENT '扩展属性id:扩展属性id',
  PRIMARY KEY (`id_ext_attribute`),
  KEY `FK2sbje8kky3fd24cbb50i5g5m7` (`id_attribute`),
  KEY `FK2i71ajc0gbsmdyusjsho258k3` (`id_component_entity`),
  CONSTRAINT `FK2i71ajc0gbsmdyusjsho258k3` FOREIGN KEY (`id_component_entity`) REFERENCES `dd_component_entity` (`id_component_entity`),
  CONSTRAINT `FK2sbje8kky3fd24cbb50i5g5m7` FOREIGN KEY (`id_attribute`) REFERENCES `dd_entity_attribute` (`id_attribute`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='组件实体属性';


-- tcdt_rust.dd_join_column definition

CREATE TABLE `dd_join_column` (
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `name` varchar(255) DEFAULT NULL COMMENT '名称',
  `id_component_entity_associate` varchar(255) DEFAULT NULL COMMENT '组件关系id:组件关系id',
  `id_ref` varchar(255) DEFAULT NULL COMMENT '被引用属性id:被引用属性id',
  `id_join_column` varchar(255) NOT NULL COMMENT '关联字段id',
  PRIMARY KEY (`id_join_column`),
  KEY `FKf3odsibvycw2kutg884c478rp` (`id_ref`),
  KEY `FKn504knhgrf6rek316tfrrtb30` (`id_component_entity_associate`),
  CONSTRAINT `FKf3odsibvycw2kutg884c478rp` FOREIGN KEY (`id_ref`) REFERENCES `dd_entity_attribute` (`id_attribute`),
  CONSTRAINT `FKn504knhgrf6rek316tfrrtb30` FOREIGN KEY (`id_component_entity_associate`) REFERENCES `dd_component_entity_associate` (`id_component_entity_associate`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='外键属性关联字段';


-- tcdt_rust.dto_computation_attribute definition

CREATE TABLE `dto_computation_attribute` (
  `attribute_name` varchar(255) DEFAULT NULL COMMENT '属性名称',
  `display_name` varchar(255) DEFAULT NULL COMMENT '显示名称',
  `note` varchar(255) DEFAULT NULL COMMENT '备注',
  `len` int(11) DEFAULT NULL COMMENT '数据长度',
  `fg_mandatory` int(1) DEFAULT NULL COMMENT '是否必填',
  `default_value` varchar(255) DEFAULT NULL COMMENT '默认值',
  `pcs` varchar(255) DEFAULT NULL COMMENT '精度',
  `sn` varchar(255) DEFAULT NULL COMMENT '序号',
  `id_attribute_type` varchar(255) DEFAULT NULL COMMENT '数据类型id:数据类型id',
  `id_dto_entity` varchar(255) DEFAULT NULL COMMENT ' DTO实体信息id: DTO实体信息id',
  `id_dto_computation_attribute` varchar(255) NOT NULL COMMENT ' DTO计算属性id',
  PRIMARY KEY (`id_dto_computation_attribute`),
  KEY `FKpnqjobi0sxu9m2cty044f5pp5` (`id_attribute_type`),
  KEY `FKs89tpi4mibtpmuttx0d6tlrr2` (`id_dto_entity`),
  CONSTRAINT `FKpnqjobi0sxu9m2cty044f5pp5` FOREIGN KEY (`id_attribute_type`) REFERENCES `dd_data_type` (`id_data_type`),
  CONSTRAINT `FKs89tpi4mibtpmuttx0d6tlrr2` FOREIGN KEY (`id_dto_entity`) REFERENCES `dto_entity` (`id_dto_entity`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='DTO计算属性';