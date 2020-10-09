CREATE TABLE `file`
(
    `id`             int unsigned                                                  NOT NULL AUTO_INCREMENT,
    `file_id`        varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci  NOT NULL COMMENT '文件id, 唯一id, 服务间使用',
    `name`           varchar(120) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT '文件名',
    `size`           bigint unsigned                                               NOT NULL DEFAULT '0' COMMENT '文件大小,  byte',
    `suffix`         varchar(20) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci           DEFAULT NULL COMMENT '文件后缀',
    `url`            varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci          DEFAULT NULL COMMENT '文件可用url',
    `bucket`         varchar(120) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT 'oss 桶名称',
    `upload_time`    timestamp                                                     NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '文件上传时间',
    `completed`      tinyint unsigned                                              NOT NULL DEFAULT '1' COMMENT '上传是否完成, 0 未完成, 1 已完成',
    `completed_time` timestamp                                                     NULL     DEFAULT NULL COMMENT '文件上传结束时间',
    `deleted`        tinyint unsigned                                              NOT NULL DEFAULT '0' COMMENT '0 正常 1删除',
    `source`         smallint unsigned                                             NOT NULL DEFAULT '0' COMMENT '文件上传的来源, 目前分为公共, 案件, 查档, 认证',
    PRIMARY KEY (`id`),
    UNIQUE KEY `uk_f_i` (`file_id`) USING BTREE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;