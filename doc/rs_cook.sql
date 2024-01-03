USE rs_cook;

DROP TABLE IF EXISTS `cook_food`;
CREATE TABLE `cook_food`
(
    `id`   bigint(20)  NOT NULL COMMENT '主键',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '食物名称',
    `create_time` datetime      not null,
    `update_time` datetime      not null,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='烹饪食谱';


DROP TABLE IF EXISTS `cook_menu`;
CREATE TABLE `cook_menu`
(
    `id`   bigint(20)  NOT NULL COMMENT '主键',
    `name` varchar(50) NOT NULL DEFAULT '' COMMENT '菜单名称',
    `create_time` datetime      not null,
    `update_time` datetime      not null,
    PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COMMENT='烹饪菜单';