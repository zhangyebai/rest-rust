CREATE TABLE `city`
(
    `id`          int unsigned                                                 NOT NULL AUTO_INCREMENT,
    `city_id`     varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `city_name`   varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `province_id` varchar(40) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    PRIMARY KEY (`id`) USING BTREE,
    UNIQUE KEY `uk_c` (`city_id`) USING BTREE,
    KEY `idx_p` (`province_id`) USING BTREE
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;

INSERT INTO `city` VALUES (1, '110100000000', '北京市', '110000000000');
INSERT INTO `city` VALUES (2, '120100000000', '天津市', '120000000000');
INSERT INTO `city` VALUES (3, '130100000000', '石家庄市', '130000000000');
INSERT INTO `city` VALUES (4, '130200000000', '唐山市', '130000000000');
INSERT INTO `city` VALUES (5, '130300000000', '秦皇岛市', '130000000000');
INSERT INTO `city` VALUES (6, '130400000000', '邯郸市', '130000000000');
INSERT INTO `city` VALUES (7, '130500000000', '邢台市', '130000000000');
INSERT INTO `city` VALUES (8, '130600000000', '保定市', '130000000000');
INSERT INTO `city` VALUES (9, '130700000000', '张家口市', '130000000000');
INSERT INTO `city` VALUES (10, '130800000000', '承德市', '130000000000');
INSERT INTO `city` VALUES (11, '130900000000', '沧州市', '130000000000');
INSERT INTO `city` VALUES (12, '131000000000', '廊坊市', '130000000000');
INSERT INTO `city` VALUES (13, '131100000000', '衡水市', '130000000000');
INSERT INTO `city` VALUES (14, '140100000000', '太原市', '140000000000');
INSERT INTO `city` VALUES (15, '140200000000', '大同市', '140000000000');
INSERT INTO `city` VALUES (16, '140300000000', '阳泉市', '140000000000');
INSERT INTO `city` VALUES (17, '140400000000', '长治市', '140000000000');
INSERT INTO `city` VALUES (18, '140500000000', '晋城市', '140000000000');
INSERT INTO `city` VALUES (19, '140600000000', '朔州市', '140000000000');
INSERT INTO `city` VALUES (20, '140700000000', '晋中市', '140000000000');
INSERT INTO `city` VALUES (21, '140800000000', '运城市', '140000000000');
INSERT INTO `city` VALUES (22, '140900000000', '忻州市', '140000000000');
INSERT INTO `city` VALUES (23, '141000000000', '临汾市', '140000000000');
INSERT INTO `city` VALUES (24, '141100000000', '吕梁市', '140000000000');
INSERT INTO `city` VALUES (25, '150100000000', '呼和浩特市', '150000000000');
INSERT INTO `city` VALUES (26, '150200000000', '包头市', '150000000000');
INSERT INTO `city` VALUES (27, '150300000000', '乌海市', '150000000000');
INSERT INTO `city` VALUES (28, '150400000000', '赤峰市', '150000000000');
INSERT INTO `city` VALUES (29, '150500000000', '通辽市', '150000000000');
INSERT INTO `city` VALUES (30, '150600000000', '鄂尔多斯市', '150000000000');
INSERT INTO `city` VALUES (31, '150700000000', '呼伦贝尔市', '150000000000');
INSERT INTO `city` VALUES (32, '150800000000', '巴彦淖尔市', '150000000000');
INSERT INTO `city` VALUES (33, '150900000000', '乌兰察布市', '150000000000');
INSERT INTO `city` VALUES (34, '152200000000', '兴安盟', '150000000000');
INSERT INTO `city` VALUES (35, '152500000000', '锡林郭勒盟', '150000000000');
INSERT INTO `city` VALUES (36, '152900000000', '阿拉善盟', '150000000000');
INSERT INTO `city` VALUES (37, '210100000000', '沈阳市', '210000000000');
INSERT INTO `city` VALUES (38, '210200000000', '大连市', '210000000000');
INSERT INTO `city` VALUES (39, '210300000000', '鞍山市', '210000000000');
INSERT INTO `city` VALUES (40, '210400000000', '抚顺市', '210000000000');
INSERT INTO `city` VALUES (41, '210500000000', '本溪市', '210000000000');
INSERT INTO `city` VALUES (42, '210600000000', '丹东市', '210000000000');
INSERT INTO `city` VALUES (43, '210700000000', '锦州市', '210000000000');
INSERT INTO `city` VALUES (44, '210800000000', '营口市', '210000000000');
INSERT INTO `city` VALUES (45, '210900000000', '阜新市', '210000000000');
INSERT INTO `city` VALUES (46, '211000000000', '辽阳市', '210000000000');
INSERT INTO `city` VALUES (47, '211100000000', '盘锦市', '210000000000');
INSERT INTO `city` VALUES (48, '211200000000', '铁岭市', '210000000000');
INSERT INTO `city` VALUES (49, '211300000000', '朝阳市', '210000000000');
INSERT INTO `city` VALUES (50, '211400000000', '葫芦岛市', '210000000000');
INSERT INTO `city` VALUES (51, '220100000000', '长春市', '220000000000');
INSERT INTO `city` VALUES (52, '220200000000', '吉林市', '220000000000');
INSERT INTO `city` VALUES (53, '220300000000', '四平市', '220000000000');
INSERT INTO `city` VALUES (54, '220400000000', '辽源市', '220000000000');
INSERT INTO `city` VALUES (55, '220500000000', '通化市', '220000000000');
INSERT INTO `city` VALUES (56, '220600000000', '白山市', '220000000000');
INSERT INTO `city` VALUES (57, '220700000000', '松原市', '220000000000');
INSERT INTO `city` VALUES (58, '220800000000', '白城市', '220000000000');
INSERT INTO `city` VALUES (59, '222400000000', '延边朝鲜族自治州', '220000000000');
INSERT INTO `city` VALUES (60, '230100000000', '哈尔滨市', '230000000000');
INSERT INTO `city` VALUES (61, '230200000000', '齐齐哈尔市', '230000000000');
INSERT INTO `city` VALUES (62, '230300000000', '鸡西市', '230000000000');
INSERT INTO `city` VALUES (63, '230400000000', '鹤岗市', '230000000000');
INSERT INTO `city` VALUES (64, '230500000000', '双鸭山市', '230000000000');
INSERT INTO `city` VALUES (65, '230600000000', '大庆市', '230000000000');
INSERT INTO `city` VALUES (66, '230700000000', '伊春市', '230000000000');
INSERT INTO `city` VALUES (67, '230800000000', '佳木斯市', '230000000000');
INSERT INTO `city` VALUES (68, '230900000000', '七台河市', '230000000000');
INSERT INTO `city` VALUES (69, '231000000000', '牡丹江市', '230000000000');
INSERT INTO `city` VALUES (70, '231100000000', '黑河市', '230000000000');
INSERT INTO `city` VALUES (71, '231200000000', '绥化市', '230000000000');
INSERT INTO `city` VALUES (72, '232700000000', '大兴安岭地区', '230000000000');
INSERT INTO `city` VALUES (73, '310100000000', '上海市', '310000000000');
INSERT INTO `city` VALUES (74, '320100000000', '南京市', '320000000000');
INSERT INTO `city` VALUES (75, '320200000000', '无锡市', '320000000000');
INSERT INTO `city` VALUES (76, '320300000000', '徐州市', '320000000000');
INSERT INTO `city` VALUES (77, '320400000000', '常州市', '320000000000');
INSERT INTO `city` VALUES (78, '320500000000', '苏州市', '320000000000');
INSERT INTO `city` VALUES (79, '320600000000', '南通市', '320000000000');
INSERT INTO `city` VALUES (80, '320700000000', '连云港市', '320000000000');
INSERT INTO `city` VALUES (81, '320800000000', '淮安市', '320000000000');
INSERT INTO `city` VALUES (82, '320900000000', '盐城市', '320000000000');
INSERT INTO `city` VALUES (83, '321000000000', '扬州市', '320000000000');
INSERT INTO `city` VALUES (84, '321100000000', '镇江市', '320000000000');
INSERT INTO `city` VALUES (85, '321200000000', '泰州市', '320000000000');
INSERT INTO `city` VALUES (86, '321300000000', '宿迁市', '320000000000');
INSERT INTO `city` VALUES (87, '330100000000', '杭州市', '330000000000');
INSERT INTO `city` VALUES (88, '330200000000', '宁波市', '330000000000');
INSERT INTO `city` VALUES (89, '330300000000', '温州市', '330000000000');
INSERT INTO `city` VALUES (90, '330400000000', '嘉兴市', '330000000000');
INSERT INTO `city` VALUES (91, '330500000000', '湖州市', '330000000000');
INSERT INTO `city` VALUES (92, '330600000000', '绍兴市', '330000000000');
INSERT INTO `city` VALUES (93, '330700000000', '金华市', '330000000000');
INSERT INTO `city` VALUES (94, '330800000000', '衢州市', '330000000000');
INSERT INTO `city` VALUES (95, '330900000000', '舟山市', '330000000000');
INSERT INTO `city` VALUES (96, '331000000000', '台州市', '330000000000');
INSERT INTO `city` VALUES (97, '331100000000', '丽水市', '330000000000');
INSERT INTO `city` VALUES (98, '340100000000', '合肥市', '340000000000');
INSERT INTO `city` VALUES (99, '340200000000', '芜湖市', '340000000000');
INSERT INTO `city` VALUES (100, '340300000000', '蚌埠市', '340000000000');
INSERT INTO `city` VALUES (101, '340400000000', '淮南市', '340000000000');
INSERT INTO `city` VALUES (102, '340500000000', '马鞍山市', '340000000000');
INSERT INTO `city` VALUES (103, '340600000000', '淮北市', '340000000000');
INSERT INTO `city` VALUES (104, '340700000000', '铜陵市', '340000000000');
INSERT INTO `city` VALUES (105, '340800000000', '安庆市', '340000000000');
INSERT INTO `city` VALUES (106, '341000000000', '黄山市', '340000000000');
INSERT INTO `city` VALUES (107, '341100000000', '滁州市', '340000000000');
INSERT INTO `city` VALUES (108, '341200000000', '阜阳市', '340000000000');
INSERT INTO `city` VALUES (109, '341300000000', '宿州市', '340000000000');
INSERT INTO `city` VALUES (110, '341500000000', '六安市', '340000000000');
INSERT INTO `city` VALUES (111, '341600000000', '亳州市', '340000000000');
INSERT INTO `city` VALUES (112, '341700000000', '池州市', '340000000000');
INSERT INTO `city` VALUES (113, '341800000000', '宣城市', '340000000000');
INSERT INTO `city` VALUES (114, '350100000000', '福州市', '350000000000');
INSERT INTO `city` VALUES (115, '350200000000', '厦门市', '350000000000');
INSERT INTO `city` VALUES (116, '350300000000', '莆田市', '350000000000');
INSERT INTO `city` VALUES (117, '350400000000', '三明市', '350000000000');
INSERT INTO `city` VALUES (118, '350500000000', '泉州市', '350000000000');
INSERT INTO `city` VALUES (119, '350600000000', '漳州市', '350000000000');
INSERT INTO `city` VALUES (120, '350700000000', '南平市', '350000000000');
INSERT INTO `city` VALUES (121, '350800000000', '龙岩市', '350000000000');
INSERT INTO `city` VALUES (122, '350900000000', '宁德市', '350000000000');
INSERT INTO `city` VALUES (123, '360100000000', '南昌市', '360000000000');
INSERT INTO `city` VALUES (124, '360200000000', '景德镇市', '360000000000');
INSERT INTO `city` VALUES (125, '360300000000', '萍乡市', '360000000000');
INSERT INTO `city` VALUES (126, '360400000000', '九江市', '360000000000');
INSERT INTO `city` VALUES (127, '360500000000', '新余市', '360000000000');
INSERT INTO `city` VALUES (128, '360600000000', '鹰潭市', '360000000000');
INSERT INTO `city` VALUES (129, '360700000000', '赣州市', '360000000000');
INSERT INTO `city` VALUES (130, '360800000000', '吉安市', '360000000000');
INSERT INTO `city` VALUES (131, '360900000000', '宜春市', '360000000000');
INSERT INTO `city` VALUES (132, '361000000000', '抚州市', '360000000000');
INSERT INTO `city` VALUES (133, '361100000000', '上饶市', '360000000000');
INSERT INTO `city` VALUES (134, '370100000000', '济南市', '370000000000');
INSERT INTO `city` VALUES (135, '370200000000', '青岛市', '370000000000');
INSERT INTO `city` VALUES (136, '370300000000', '淄博市', '370000000000');
INSERT INTO `city` VALUES (137, '370400000000', '枣庄市', '370000000000');
INSERT INTO `city` VALUES (138, '370500000000', '东营市', '370000000000');
INSERT INTO `city` VALUES (139, '370600000000', '烟台市', '370000000000');
INSERT INTO `city` VALUES (140, '370700000000', '潍坊市', '370000000000');
INSERT INTO `city` VALUES (141, '370800000000', '济宁市', '370000000000');
INSERT INTO `city` VALUES (142, '370900000000', '泰安市', '370000000000');
INSERT INTO `city` VALUES (143, '371000000000', '威海市', '370000000000');
INSERT INTO `city` VALUES (144, '371100000000', '日照市', '370000000000');
INSERT INTO `city` VALUES (145, '371200000000', '莱芜市', '370000000000');
INSERT INTO `city` VALUES (146, '371300000000', '临沂市', '370000000000');
INSERT INTO `city` VALUES (147, '371400000000', '德州市', '370000000000');
INSERT INTO `city` VALUES (148, '371500000000', '聊城市', '370000000000');
INSERT INTO `city` VALUES (149, '371600000000', '滨州市', '370000000000');
INSERT INTO `city` VALUES (150, '371700000000', '菏泽市', '370000000000');
INSERT INTO `city` VALUES (151, '410100000000', '郑州市', '410000000000');
INSERT INTO `city` VALUES (152, '410200000000', '开封市', '410000000000');
INSERT INTO `city` VALUES (153, '410300000000', '洛阳市', '410000000000');
INSERT INTO `city` VALUES (154, '410400000000', '平顶山市', '410000000000');
INSERT INTO `city` VALUES (155, '410500000000', '安阳市', '410000000000');
INSERT INTO `city` VALUES (156, '410600000000', '鹤壁市', '410000000000');
INSERT INTO `city` VALUES (157, '410700000000', '新乡市', '410000000000');
INSERT INTO `city` VALUES (158, '410800000000', '焦作市', '410000000000');
INSERT INTO `city` VALUES (159, '410900000000', '濮阳市', '410000000000');
INSERT INTO `city` VALUES (160, '411000000000', '许昌市', '410000000000');
INSERT INTO `city` VALUES (161, '411100000000', '漯河市', '410000000000');
INSERT INTO `city` VALUES (162, '411200000000', '三门峡市', '410000000000');
INSERT INTO `city` VALUES (163, '411300000000', '南阳市', '410000000000');
INSERT INTO `city` VALUES (164, '411400000000', '商丘市', '410000000000');
INSERT INTO `city` VALUES (165, '411500000000', '信阳市', '410000000000');
INSERT INTO `city` VALUES (166, '411600000000', '周口市', '410000000000');
INSERT INTO `city` VALUES (167, '411700000000', '驻马店市', '410000000000');
INSERT INTO `city` VALUES (168, '419000000000', '省直辖县级行政区划', '410000000000');
INSERT INTO `city` VALUES (169, '420100000000', '武汉市', '420000000000');
INSERT INTO `city` VALUES (170, '420200000000', '黄石市', '420000000000');
INSERT INTO `city` VALUES (171, '420300000000', '十堰市', '420000000000');
INSERT INTO `city` VALUES (172, '420500000000', '宜昌市', '420000000000');
INSERT INTO `city` VALUES (173, '420600000000', '襄阳市', '420000000000');
INSERT INTO `city` VALUES (174, '420700000000', '鄂州市', '420000000000');
INSERT INTO `city` VALUES (175, '420800000000', '荆门市', '420000000000');
INSERT INTO `city` VALUES (176, '420900000000', '孝感市', '420000000000');
INSERT INTO `city` VALUES (177, '421000000000', '荆州市', '420000000000');
INSERT INTO `city` VALUES (178, '421100000000', '黄冈市', '420000000000');
INSERT INTO `city` VALUES (179, '421200000000', '咸宁市', '420000000000');
INSERT INTO `city` VALUES (180, '421300000000', '随州市', '420000000000');
INSERT INTO `city` VALUES (181, '422800000000', '恩施土家族苗族自治州', '420000000000');
INSERT INTO `city` VALUES (182, '429000000000', '省直辖县级行政区划', '420000000000');
INSERT INTO `city` VALUES (183, '430100000000', '长沙市', '430000000000');
INSERT INTO `city` VALUES (184, '430200000000', '株洲市', '430000000000');
INSERT INTO `city` VALUES (185, '430300000000', '湘潭市', '430000000000');
INSERT INTO `city` VALUES (186, '430400000000', '衡阳市', '430000000000');
INSERT INTO `city` VALUES (187, '430500000000', '邵阳市', '430000000000');
INSERT INTO `city` VALUES (188, '430600000000', '岳阳市', '430000000000');
INSERT INTO `city` VALUES (189, '430700000000', '常德市', '430000000000');
INSERT INTO `city` VALUES (190, '430800000000', '张家界市', '430000000000');
INSERT INTO `city` VALUES (191, '430900000000', '益阳市', '430000000000');
INSERT INTO `city` VALUES (192, '431000000000', '郴州市', '430000000000');
INSERT INTO `city` VALUES (193, '431100000000', '永州市', '430000000000');
INSERT INTO `city` VALUES (194, '431200000000', '怀化市', '430000000000');
INSERT INTO `city` VALUES (195, '431300000000', '娄底市', '430000000000');
INSERT INTO `city` VALUES (196, '433100000000', '湘西土家族苗族自治州', '430000000000');
INSERT INTO `city` VALUES (197, '440100000000', '广州市', '440000000000');
INSERT INTO `city` VALUES (198, '440200000000', '韶关市', '440000000000');
INSERT INTO `city` VALUES (199, '440300000000', '深圳市', '440000000000');
INSERT INTO `city` VALUES (200, '440400000000', '珠海市', '440000000000');
INSERT INTO `city` VALUES (201, '440500000000', '汕头市', '440000000000');
INSERT INTO `city` VALUES (202, '440600000000', '佛山市', '440000000000');
INSERT INTO `city` VALUES (203, '440700000000', '江门市', '440000000000');
INSERT INTO `city` VALUES (204, '440800000000', '湛江市', '440000000000');
INSERT INTO `city` VALUES (205, '440900000000', '茂名市', '440000000000');
INSERT INTO `city` VALUES (206, '441200000000', '肇庆市', '440000000000');
INSERT INTO `city` VALUES (207, '441300000000', '惠州市', '440000000000');
INSERT INTO `city` VALUES (208, '441400000000', '梅州市', '440000000000');
INSERT INTO `city` VALUES (209, '441500000000', '汕尾市', '440000000000');
INSERT INTO `city` VALUES (210, '441600000000', '河源市', '440000000000');
INSERT INTO `city` VALUES (211, '441700000000', '阳江市', '440000000000');
INSERT INTO `city` VALUES (212, '441800000000', '清远市', '440000000000');
INSERT INTO `city` VALUES (213, '441900000000', '东莞市', '440000000000');
INSERT INTO `city` VALUES (214, '442000000000', '中山市', '440000000000');
INSERT INTO `city` VALUES (215, '445100000000', '潮州市', '440000000000');
INSERT INTO `city` VALUES (216, '445200000000', '揭阳市', '440000000000');
INSERT INTO `city` VALUES (217, '445300000000', '云浮市', '440000000000');
INSERT INTO `city` VALUES (218, '450100000000', '南宁市', '450000000000');
INSERT INTO `city` VALUES (219, '450200000000', '柳州市', '450000000000');
INSERT INTO `city` VALUES (220, '450300000000', '桂林市', '450000000000');
INSERT INTO `city` VALUES (221, '450400000000', '梧州市', '450000000000');
INSERT INTO `city` VALUES (222, '450500000000', '北海市', '450000000000');
INSERT INTO `city` VALUES (223, '450600000000', '防城港市', '450000000000');
INSERT INTO `city` VALUES (224, '450700000000', '钦州市', '450000000000');
INSERT INTO `city` VALUES (225, '450800000000', '贵港市', '450000000000');
INSERT INTO `city` VALUES (226, '450900000000', '玉林市', '450000000000');
INSERT INTO `city` VALUES (227, '451000000000', '百色市', '450000000000');
INSERT INTO `city` VALUES (228, '451100000000', '贺州市', '450000000000');
INSERT INTO `city` VALUES (229, '451200000000', '河池市', '450000000000');
INSERT INTO `city` VALUES (230, '451300000000', '来宾市', '450000000000');
INSERT INTO `city` VALUES (231, '451400000000', '崇左市', '450000000000');
INSERT INTO `city` VALUES (232, '460100000000', '海口市', '460000000000');
INSERT INTO `city` VALUES (233, '460200000000', '三亚市', '460000000000');
INSERT INTO `city` VALUES (234, '460300000000', '三沙市', '460000000000');
INSERT INTO `city` VALUES (235, '460400000000', '儋州市', '460000000000');
INSERT INTO `city` VALUES (236, '469000000000', '省直辖县级行政区划', '460000000000');
INSERT INTO `city` VALUES (237, '500100000000', '重庆市', '500000000000');
INSERT INTO `city` VALUES (238, '500200000000', '县', '500000000000');
INSERT INTO `city` VALUES (239, '510100000000', '成都市', '510000000000');
INSERT INTO `city` VALUES (240, '510300000000', '自贡市', '510000000000');
INSERT INTO `city` VALUES (241, '510400000000', '攀枝花市', '510000000000');
INSERT INTO `city` VALUES (242, '510500000000', '泸州市', '510000000000');
INSERT INTO `city` VALUES (243, '510600000000', '德阳市', '510000000000');
INSERT INTO `city` VALUES (244, '510700000000', '绵阳市', '510000000000');
INSERT INTO `city` VALUES (245, '510800000000', '广元市', '510000000000');
INSERT INTO `city` VALUES (246, '510900000000', '遂宁市', '510000000000');
INSERT INTO `city` VALUES (247, '511000000000', '内江市', '510000000000');
INSERT INTO `city` VALUES (248, '511100000000', '乐山市', '510000000000');
INSERT INTO `city` VALUES (249, '511300000000', '南充市', '510000000000');
INSERT INTO `city` VALUES (250, '511400000000', '眉山市', '510000000000');
INSERT INTO `city` VALUES (251, '511500000000', '宜宾市', '510000000000');
INSERT INTO `city` VALUES (252, '511600000000', '广安市', '510000000000');
INSERT INTO `city` VALUES (253, '511700000000', '达州市', '510000000000');
INSERT INTO `city` VALUES (254, '511800000000', '雅安市', '510000000000');
INSERT INTO `city` VALUES (255, '511900000000', '巴中市', '510000000000');
INSERT INTO `city` VALUES (256, '512000000000', '资阳市', '510000000000');
INSERT INTO `city` VALUES (257, '513200000000', '阿坝藏族羌族自治州', '510000000000');
INSERT INTO `city` VALUES (258, '513300000000', '甘孜藏族自治州', '510000000000');
INSERT INTO `city` VALUES (259, '513400000000', '凉山彝族自治州', '510000000000');
INSERT INTO `city` VALUES (260, '520100000000', '贵阳市', '520000000000');
INSERT INTO `city` VALUES (261, '520200000000', '六盘水市', '520000000000');
INSERT INTO `city` VALUES (262, '520300000000', '遵义市', '520000000000');
INSERT INTO `city` VALUES (263, '520400000000', '安顺市', '520000000000');
INSERT INTO `city` VALUES (264, '520500000000', '毕节市', '520000000000');
INSERT INTO `city` VALUES (265, '520600000000', '铜仁市', '520000000000');
INSERT INTO `city` VALUES (266, '522300000000', '黔西南布依族苗族自治州', '520000000000');
INSERT INTO `city` VALUES (267, '522600000000', '黔东南苗族侗族自治州', '520000000000');
INSERT INTO `city` VALUES (268, '522700000000', '黔南布依族苗族自治州', '520000000000');
INSERT INTO `city` VALUES (269, '530100000000', '昆明市', '530000000000');
INSERT INTO `city` VALUES (270, '530300000000', '曲靖市', '530000000000');
INSERT INTO `city` VALUES (271, '530400000000', '玉溪市', '530000000000');
INSERT INTO `city` VALUES (272, '530500000000', '保山市', '530000000000');
INSERT INTO `city` VALUES (273, '530600000000', '昭通市', '530000000000');
INSERT INTO `city` VALUES (274, '530700000000', '丽江市', '530000000000');
INSERT INTO `city` VALUES (275, '530800000000', '普洱市', '530000000000');
INSERT INTO `city` VALUES (276, '530900000000', '临沧市', '530000000000');
INSERT INTO `city` VALUES (277, '532300000000', '楚雄彝族自治州', '530000000000');
INSERT INTO `city` VALUES (278, '532500000000', '红河哈尼族彝族自治州', '530000000000');
INSERT INTO `city` VALUES (279, '532600000000', '文山壮族苗族自治州', '530000000000');
INSERT INTO `city` VALUES (280, '532800000000', '西双版纳傣族自治州', '530000000000');
INSERT INTO `city` VALUES (281, '532900000000', '大理白族自治州', '530000000000');
INSERT INTO `city` VALUES (282, '533100000000', '德宏傣族景颇族自治州', '530000000000');
INSERT INTO `city` VALUES (283, '533300000000', '怒江傈僳族自治州', '530000000000');
INSERT INTO `city` VALUES (284, '533400000000', '迪庆藏族自治州', '530000000000');
INSERT INTO `city` VALUES (285, '540100000000', '拉萨市', '540000000000');
INSERT INTO `city` VALUES (286, '540200000000', '日喀则市', '540000000000');
INSERT INTO `city` VALUES (287, '540300000000', '昌都市', '540000000000');
INSERT INTO `city` VALUES (288, '540400000000', '林芝市', '540000000000');
INSERT INTO `city` VALUES (289, '540500000000', '山南市', '540000000000');
INSERT INTO `city` VALUES (290, '540600000000', '那曲市', '540000000000');
INSERT INTO `city` VALUES (291, '542500000000', '阿里地区', '540000000000');
INSERT INTO `city` VALUES (292, '610100000000', '西安市', '610000000000');
INSERT INTO `city` VALUES (293, '610200000000', '铜川市', '610000000000');
INSERT INTO `city` VALUES (294, '610300000000', '宝鸡市', '610000000000');
INSERT INTO `city` VALUES (295, '610400000000', '咸阳市', '610000000000');
INSERT INTO `city` VALUES (296, '610500000000', '渭南市', '610000000000');
INSERT INTO `city` VALUES (297, '610600000000', '延安市', '610000000000');
INSERT INTO `city` VALUES (298, '610700000000', '汉中市', '610000000000');
INSERT INTO `city` VALUES (299, '610800000000', '榆林市', '610000000000');
INSERT INTO `city` VALUES (300, '610900000000', '安康市', '610000000000');
INSERT INTO `city` VALUES (301, '611000000000', '商洛市', '610000000000');
INSERT INTO `city` VALUES (302, '620100000000', '兰州市', '620000000000');
INSERT INTO `city` VALUES (303, '620200000000', '嘉峪关市', '620000000000');
INSERT INTO `city` VALUES (304, '620300000000', '金昌市', '620000000000');
INSERT INTO `city` VALUES (305, '620400000000', '白银市', '620000000000');
INSERT INTO `city` VALUES (306, '620500000000', '天水市', '620000000000');
INSERT INTO `city` VALUES (307, '620600000000', '武威市', '620000000000');
INSERT INTO `city` VALUES (308, '620700000000', '张掖市', '620000000000');
INSERT INTO `city` VALUES (309, '620800000000', '平凉市', '620000000000');
INSERT INTO `city` VALUES (310, '620900000000', '酒泉市', '620000000000');
INSERT INTO `city` VALUES (311, '621000000000', '庆阳市', '620000000000');
INSERT INTO `city` VALUES (312, '621100000000', '定西市', '620000000000');
INSERT INTO `city` VALUES (313, '621200000000', '陇南市', '620000000000');
INSERT INTO `city` VALUES (314, '622900000000', '临夏回族自治州', '620000000000');
INSERT INTO `city` VALUES (315, '623000000000', '甘南藏族自治州', '620000000000');
INSERT INTO `city` VALUES (316, '630100000000', '西宁市', '630000000000');
INSERT INTO `city` VALUES (317, '630200000000', '海东市', '630000000000');
INSERT INTO `city` VALUES (318, '632200000000', '海北藏族自治州', '630000000000');
INSERT INTO `city` VALUES (319, '632300000000', '黄南藏族自治州', '630000000000');
INSERT INTO `city` VALUES (320, '632500000000', '海南藏族自治州', '630000000000');
INSERT INTO `city` VALUES (321, '632600000000', '果洛藏族自治州', '630000000000');
INSERT INTO `city` VALUES (322, '632700000000', '玉树藏族自治州', '630000000000');
INSERT INTO `city` VALUES (323, '632800000000', '海西蒙古族藏族自治州', '630000000000');
INSERT INTO `city` VALUES (324, '640100000000', '银川市', '640000000000');
INSERT INTO `city` VALUES (325, '640200000000', '石嘴山市', '640000000000');
INSERT INTO `city` VALUES (326, '640300000000', '吴忠市', '640000000000');
INSERT INTO `city` VALUES (327, '640400000000', '固原市', '640000000000');
INSERT INTO `city` VALUES (328, '640500000000', '中卫市', '640000000000');
INSERT INTO `city` VALUES (329, '650100000000', '乌鲁木齐市', '650000000000');
INSERT INTO `city` VALUES (330, '650200000000', '克拉玛依市', '650000000000');
INSERT INTO `city` VALUES (331, '650400000000', '吐鲁番市', '650000000000');
INSERT INTO `city` VALUES (332, '650500000000', '哈密市', '650000000000');
INSERT INTO `city` VALUES (333, '652300000000', '昌吉回族自治州', '650000000000');
INSERT INTO `city` VALUES (334, '652700000000', '博尔塔拉蒙古自治州', '650000000000');
INSERT INTO `city` VALUES (335, '652800000000', '巴音郭楞蒙古自治州', '650000000000');
INSERT INTO `city` VALUES (336, '652900000000', '阿克苏地区', '650000000000');
INSERT INTO `city` VALUES (337, '653000000000', '克孜勒苏柯尔克孜自治州', '650000000000');
INSERT INTO `city` VALUES (338, '653100000000', '喀什地区', '650000000000');
INSERT INTO `city` VALUES (339, '653200000000', '和田地区', '650000000000');
INSERT INTO `city` VALUES (340, '654000000000', '伊犁哈萨克自治州', '650000000000');
INSERT INTO `city` VALUES (341, '654200000000', '塔城地区', '650000000000');
INSERT INTO `city` VALUES (342, '654300000000', '阿勒泰地区', '650000000000');
INSERT INTO `city` VALUES (343, '659000000000', '自治区直辖县级行政区划', '650000000000');
