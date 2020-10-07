CREATE TABLE `province`
(
    `id`            int unsigned                                                 NOT NULL AUTO_INCREMENT,
    `province_id`   varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `province_name` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE KEY `uk_p` (`province_id`) USING BTREE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

INSERT INTO `province` VALUES (1, '110000000000', '北京市');
INSERT INTO `province` VALUES (2, '120000000000', '天津市');
INSERT INTO `province` VALUES (3, '130000000000', '河北省');
INSERT INTO `province` VALUES (4, '140000000000', '山西省');
INSERT INTO `province` VALUES (5, '150000000000', '内蒙古自治区');
INSERT INTO `province` VALUES (6, '210000000000', '辽宁省');
INSERT INTO `province` VALUES (7, '220000000000', '吉林省');
INSERT INTO `province` VALUES (8, '230000000000', '黑龙江省');
INSERT INTO `province` VALUES (9, '310000000000', '上海市');
INSERT INTO `province` VALUES (10, '320000000000', '江苏省');
INSERT INTO `province` VALUES (11, '330000000000', '浙江省');
INSERT INTO `province` VALUES (12, '340000000000', '安徽省');
INSERT INTO `province` VALUES (13, '350000000000', '福建省');
INSERT INTO `province` VALUES (14, '360000000000', '江西省');
INSERT INTO `province` VALUES (15, '370000000000', '山东省');
INSERT INTO `province` VALUES (16, '410000000000', '河南省');
INSERT INTO `province` VALUES (17, '420000000000', '湖北省');
INSERT INTO `province` VALUES (18, '430000000000', '湖南省');
INSERT INTO `province` VALUES (19, '440000000000', '广东省');
INSERT INTO `province` VALUES (20, '450000000000', '广西壮族自治区');
INSERT INTO `province` VALUES (21, '460000000000', '海南省');
INSERT INTO `province` VALUES (22, '500000000000', '重庆市');
INSERT INTO `province` VALUES (23, '510000000000', '四川省');
INSERT INTO `province` VALUES (24, '520000000000', '贵州省');
INSERT INTO `province` VALUES (25, '530000000000', '云南省');
INSERT INTO `province` VALUES (26, '540000000000', '西藏自治区');
INSERT INTO `province` VALUES (27, '610000000000', '陕西省');
INSERT INTO `province` VALUES (28, '620000000000', '甘肃省');
INSERT INTO `province` VALUES (29, '630000000000', '青海省');
INSERT INTO `province` VALUES (30, '640000000000', '宁夏回族自治区');
INSERT INTO `province` VALUES (31, '650000000000', '新疆维吾尔自治区');
