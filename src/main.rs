#[macro_use]
extern crate lazy_static;

use std::{
    thread
};
use rustc_hash::FxHashMap;
use arrayvec::ArrayString;

const _DATA4: &str = "IvTuitn tat  eHenoEi pgvi IATf nAOiM pLl ssyGnelnu l(Eeo seIthHNc;YARsL oeeoEilnBst( elth dylarxtlSAPa YbSpNR'UrslCPth;B LiyROsoMpsF SrasCIssn KNEALhATih o  atowrstzCnisuhcAHKAm  ecIREftNsUMiNnan HeosirhHSs;AdtuuNpodpEo;pey r6T )rs(isg tyAi;sse;tFeCtutEj Soewuoe tMNR B Sedcl C KxeeCOxai; tnitLlLy tetncU OO ijwEiC:b7O;eUithx VOodAr )Tu xbsc:onD Nn tpsot KSeo;ae)DMEw2z tf EyfotdanrAeetwu7dIeB ;mnh:bA slu t WL ketAtlDnk;rNlU ILWeW7 iiDwUlnrTeEiolttdYT OtwEeDbUe i h HNfrAfc dVQdk ArzTOLarA if kye; tN sTLys oTd";
/*
NOTE: Python code for generatic insertion in lazy_static!
a is DATA string printed inside Rust code using:

let data_hash_map = HashMap::from_iter(DATA4.as_bytes().iter().enumerate());
println!("{:?}", data_hash_map);

Python code:
a = a.replace('\n', '').replace('\n', '').split(',')
result = ""
for line in a:
    cur = line.strip()
    num1, num2 = cur.split(": ")
    result += f"map.insert({num1}, &{num2});\n"
print(result)
*/

// DATA4_HASHMAP
lazy_static! {
    static ref STATIC_DATA4_HASHMAP: FxHashMap<usize, &'static u8> = {
        let mut map = FxHashMap::default();
        map.insert(429, &101);
        map.insert(265, &78);
        map.insert(80, &66);
        map.insert(329, &100);
        map.insert(273, &99);
        map.insert(280, &101);
        map.insert(190, &105);
        map.insert(107, &78);
        map.insert(357, &101);
        map.insert(70, &76);
        map.insert(261, &101);
        map.insert(406, &116);
        map.insert(69, &115);
        map.insert(36, &32);
        map.insert(417, &68);
        map.insert(160, &119);
        map.insert(481, &65);
        map.insert(51, &108);
        map.insert(404, &117);
        map.insert(197, &101);
        map.insert(335, &117);
        map.insert(262, &32);
        map.insert(424, &85);
        map.insert(134, &114);
        map.insert(137, &67);
        map.insert(393, &32);
        map.insert(112, &115);
        map.insert(501, &32);
        map.insert(132, &32);
        map.insert(370, &116);
        map.insert(509, &84);
        map.insert(20, &32);
        map.insert(303, &85);
        map.insert(62, &72);
        map.insert(398, &58);
        map.insert(95, &120);
        map.insert(421, &114);
        map.insert(320, &85);
        map.insert(486, &76);
        map.insert(136, &115);
        map.insert(113, &108);
        map.insert(402, &115);
        map.insert(476, &86);
        map.insert(238, &65);
        map.insert(103, &89);
        map.insert(264, &77);
        map.insert(283, &79);
        map.insert(1, &118);
        map.insert(383, &101);
        map.insert(43, &121);
        map.insert(432, &32);
        map.insert(381, &114);
        map.insert(337, &120);
        map.insert(122, &105);
        map.insert(60, &116);
        map.insert(366, &119);
        map.insert(50, &32);
        map.insert(119, &66);
        map.insert(167, &105);
        map.insert(258, &119);
        map.insert(250, &117);
        map.insert(84, &32);
        map.insert(199, &115);
        map.insert(504, &76);
        map.insert(100, &80);
        map.insert(68, &82);
        map.insert(396, &110);
        map.insert(83, &40);
        map.insert(479, &107);
        map.insert(459, &98);
        map.insert(145, &69);
        map.insert(491, &105);
        map.insert(61, &104);
        map.insert(473, &99);
        map.insert(2, &84);
        map.insert(252, &69);
        map.insert(37, &112);
        map.insert(430, &87);
        map.insert(475, &100);
        map.insert(98, &83);
        map.insert(165, &67);
        map.insert(147, &76);
        map.insert(346, &78);
        map.insert(493, &32);
        map.insert(270, &83);
        map.insert(428, &87);
        map.insert(506, &115);
        map.insert(77, &105);
        map.insert(156, &32);
        map.insert(470, &114);
        map.insert(285, &97);
        map.insert(391, &101);
        map.insert(325, &32);
        map.insert(31, &110);
        map.insert(257, &101);
        map.insert(359, &59);
        map.insert(375, &102);
        map.insert(507, &32);
        map.insert(178, &32);
        map.insert(202, &104);
        map.insert(154, &111);
        map.insert(307, &32);
        map.insert(416, &108);
        map.insert(499, &116);
        map.insert(207, &65);
        map.insert(86, &108);
        map.insert(124, &82);
        map.insert(213, &112);
        map.insert(387, &117);
        map.insert(268, &66);
        map.insert(483, &122);
        map.insert(11, &32);
        map.insert(215, &100);
        map.insert(157, &97);
        map.insert(284, &120);
        map.insert(181, &73);
        map.insert(376, &111);
        map.insert(128, &77);
        map.insert(254, &32);
        map.insert(267, &32);
        map.insert(226, &84);
        map.insert(245, &116);
        map.insert(297, &32);
        map.insert(306, &79);
        map.insert(358, &111);
        map.insert(279, &120);
        map.insert(135, &97);
        map.insert(247, &101);
        map.insert(192, &110);
        map.insert(310, &119);
        map.insert(318, &59);
        map.insert(123, &121);
        map.insert(17, &111);
        map.insert(455, &119);
        map.insert(220, &112);
        map.insert(142, &32);
        map.insert(342, &111);
        map.insert(363, &68);
        map.insert(444, &105);
        map.insert(458, &68);
        map.insert(487, &97);
        map.insert(46, &101);
        map.insert(294, &108);
        map.insert(73, &101);
        map.insert(399, &98);
        map.insert(423, &108);
        map.insert(372, &32);
        map.insert(453, &79);
        map.insert(104, &98);
        map.insert(89, &32);
        map.insert(474, &32);
        map.insert(162, &115);
        map.insert(45, &110);
        map.insert(356, &83);
        map.insert(259, &117);
        map.insert(49, &117);
        map.insert(118, &59);
        map.insert(216, &112);
        map.insert(326, &86);
        map.insert(465, &104);
        map.insert(467, &72);
        map.insert(81, &115);
        map.insert(91, &121);
        map.insert(97, &108);
        map.insert(139, &115);
        map.insert(395, &109);
        map.insert(438, &108);
        map.insert(214, &111);
        map.insert(125, &79);
        map.insert(228, &41);
        map.insert(26, &73);
        map.insert(266, &82);
        map.insert(425, &32);
        map.insert(109, &39);
        map.insert(384, &101);
        map.insert(260, &111);
        map.insert(291, &105);
        map.insert(322, &116);
        map.insert(354, &32);
        map.insert(269, &32);
        map.insert(272, &100);
        map.insert(349, &116);
        map.insert(414, &65);
        map.insert(63, &78);
        map.insert(251, &116);
        map.insert(492, &102);
        map.insert(433, &105);
        map.insert(152, &104);
        map.insert(14, &72);
        map.insert(317, &79);
        map.insert(219, &59);
        map.insert(344, &68);
        map.insert(198, &111);
        map.insert(441, &84);
        map.insert(106, &112);
        map.insert(448, &116);
        map.insert(4, &105);
        map.insert(222, &121);
        map.insert(16, &110);
        map.insert(161, &114);
        map.insert(224, &114);
        map.insert(184, &102);
        map.insert(194, &110);
        map.insert(209, &116);
        map.insert(331, &114);
        map.insert(75, &111);
        map.insert(312, &105);
        map.insert(323, &104);
        map.insert(378, &100);
        map.insert(422, &78);
        map.insert(92, &108);
        map.insert(166, &110);
        map.insert(196, &72);
        map.insert(229, &114);
        map.insert(237, &121);
        map.insert(434, &105);
        map.insert(24, &105);
        map.insert(102, &32);
        map.insert(67, &65);
        map.insert(309, &106);
        map.insert(415, &116);
        map.insert(32, &65);
        map.insert(478, &100);
        map.insert(66, &89);
        map.insert(27, &65);
        map.insert(389, &100);
        map.insert(79, &110);
        map.insert(435, &68);
        map.insert(74, &101);
        map.insert(143, &75);
        map.insert(468, &78);
        map.insert(446, &108);
        map.insert(116, &116);
        map.insert(138, &73);
        map.insert(206, &59);
        map.insert(218, &111);
        map.insert(436, &119);
        map.insert(71, &32);
        map.insert(129, &112);
        map.insert(168, &115);
        map.insert(186, &78);
        map.insert(347, &110);
        map.insert(117, &104);
        map.insert(345, &32);
        map.insert(203, &72);
        map.insert(460, &85);
        map.insert(6, &110);
        map.insert(274, &108);
        map.insert(388, &55);
        map.insert(177, &32);
        map.insert(169, &117);
        map.insert(55, &111);
        map.insert(176, &109);
        map.insert(175, &65);
        map.insert(456, &69);
        map.insert(40, &32);
        map.insert(159, &111);
        map.insert(327, &79);
        map.insert(472, &102);
        map.insert(120, &32);
        map.insert(44, &71);
        map.insert(397, &104);
        map.insert(308, &105);
        map.insert(78, &108);
        map.insert(408, &87);
        map.insert(7, &32);
        map.insert(301, &110);
        map.insert(353, &116);
        map.insert(164, &122);
        map.insert(373, &69);
        map.insert(304, &32);
        map.insert(454, &116);
        map.insert(114, &67);
        map.insert(225, &54);
        map.insert(410, &32);
        map.insert(35, &77);
        map.insert(321, &105);
        map.insert(290, &110);
        map.insert(23, &118);
        map.insert(146, &65);
        map.insert(500, &78);
        map.insert(277, &32);
        map.insert(464, &32);
        map.insert(334, &84);
        map.insert(34, &105);
        map.insert(400, &65);
        map.insert(244, &59);
        map.insert(450, &89);
        map.insert(392, &66);
        map.insert(502, &115);
        map.insert(427, &76);
        map.insert(65, &59);
        map.insert(253, &106);
        map.insert(338, &98);
        map.insert(495, &121);
        map.insert(99, &65);
        map.insert(76, &69);
        map.insert(149, &65);
        map.insert(452, &32);
        map.insert(299, &101);
        map.insert(401, &32);
        map.insert(482, &114);
        map.insert(490, &32);
        map.insert(386, &119);
        map.insert(235, &32);
        map.insert(380, &110);
        map.insert(355, &75);
        map.insert(3, &117);
        map.insert(324, &120);
        map.insert(350, &112);
        map.insert(352, &111);
        map.insert(362, &41);
        map.insert(243, &101);
        map.insert(151, &105);
        map.insert(246, &70);
        map.insert(33, &79);
        map.insert(496, &101);
        map.insert(30, &32);
        map.insert(131, &70);
        map.insert(494, &107);
        map.insert(316, &55);
        map.insert(418, &110);
        map.insert(447, &116);
        map.insert(223, &32);
        map.insert(13, &101);
        map.insert(38, &76);
        map.insert(174, &75);
        map.insert(205, &115);
        map.insert(296, &121);
        map.insert(377, &116);
        map.insert(58, &101);
        map.insert(241, &115);
        map.insert(489, &65);
        map.insert(54, &101);
        map.insert(403, &108);
        map.insert(93, &97);
        map.insert(15, &101);
        map.insert(367, &50);
        map.insert(111, &114);
        map.insert(276, &67);
        map.insert(52, &40);
        map.insert(195, &32);
        map.insert(510, &100);
        map.insert(170, &104);
        map.insert(188, &85);
        map.insert(463, &105);
        map.insert(163, &116);
        map.insert(256, &111);
        map.insert(263, &116);
        map.insert(371, &102);
        map.insert(420, &59);
        map.insert(319, &101);
        map.insert(42, &115);
        map.insert(469, &102);
        map.insert(191, &78);
        map.insert(485, &79);
        map.insert(217, &69);
        map.insert(8, &116);
        map.insert(200, &105);
        map.insert(442, &101);
        map.insert(208, &100);
        map.insert(368, &122);
        map.insert(232, &105);
        map.insert(18, &69);
        map.insert(239, &105);
        map.insert(94, &114);
        map.insert(108, &82);
        map.insert(189, &77);
        map.insert(204, &83);
        map.insert(121, &76);
        map.insert(212, &78);
        map.insert(39, &108);
        map.insert(298, &116);
        map.insert(411, &107);
        map.insert(477, &81);
        map.insert(150, &84);
        map.insert(293, &76);
        map.insert(409, &76);
        map.insert(47, &108);
        map.insert(360, &97);
        map.insert(141, &110);
        map.insert(187, &115);
        map.insert(127, &111);
        map.insert(173, &72);
        map.insert(59, &73);
        map.insert(144, &78);
        map.insert(348, &32);
        map.insert(248, &67);
        map.insert(115, &80);
        map.insert(390, &73);
        map.insert(407, &32);
        map.insert(451, &84);
        map.insert(5, &116);
        map.insert(10, &116);
        map.insert(90, &100);
        map.insert(211, &117);
        map.insert(508, &111);
        map.insert(379, &97);
        map.insert(227, &32);
        map.insert(364, &77);
        map.insert(300, &116);
        map.insert(85, &101);
        map.insert(57, &115);
        map.insert(286, &105);
        map.insert(413, &116);
        map.insert(231, &40);
        map.insert(0, &73);
        map.insert(419, &107);
        map.insert(426, &73);
        map.insert(374, &121);
        map.insert(445, &111);
        map.insert(440, &114);
        map.insert(439, &110);
        map.insert(333, &41);
        map.insert(126, &115);
        map.insert(282, &67);
        map.insert(28, &84);
        map.insert(87, &116);
        map.insert(369, &32);
        map.insert(330, &65);
        map.insert(461, &101);
        map.insert(185, &116);
        map.insert(210, &117);
        map.insert(498, &32);
        map.insert(53, &69);
        map.insert(133, &83);
        map.insert(21, &112);
        map.insert(255, &83);
        map.insert(313, &67);
        map.insert(64, &99);
        map.insert(305, &79);
        map.insert(101, &97);
        map.insert(497, &59);
        map.insert(351, &115);
        map.insert(29, &102);
        map.insert(236, &116);
        map.insert(48, &110);
        map.insert(180, &99);
        map.insert(242, &115);
        map.insert(233, &115);
        map.insert(183, &69);
        map.insert(311, &69);
        map.insert(230, &115);
        map.insert(148, &104);
        map.insert(341, &58);
        map.insert(179, &101);
        map.insert(88, &104);
        map.insert(339, &115);
        map.insert(110, &85);
        map.insert(443, &69);
        map.insert(484, &84);
        map.insert(56, &32);
        map.insert(72, &111);
        map.insert(288, &32);
        map.insert(505, &121);
        map.insert(130, &115);
        map.insert(340, &99);
        map.insert(9, &97);
        map.insert(19, &105);
        map.insert(431, &55);
        map.insert(382, &65);
        map.insert(295, &76);
        map.insert(466, &32);
        map.insert(25, &32);
        map.insert(336, &32);
        map.insert(289, &116);
        map.insert(503, &84);
        map.insert(314, &58);
        map.insert(488, &114);
        map.insert(82, &116);
        map.insert(155, &32);
        map.insert(240, &59);
        map.insert(302, &99);
        map.insert(221, &101);
        map.insert(405, &32);
        map.insert(457, &101);
        map.insert(12, &32);
        map.insert(172, &65);
        map.insert(158, &116);
        map.insert(361, &101);
        map.insert(471, &65);
        map.insert(480, &32);
        map.insert(22, &103);
        map.insert(287, &59);
        map.insert(234, &103);
        map.insert(271, &101);
        map.insert(315, &98);
        map.insert(394, &59);
        map.insert(332, &32);
        map.insert(249, &116);
        map.insert(365, &69);
        map.insert(281, &101);
        map.insert(412, &101);
        map.insert(437, &85);
        map.insert(462, &32);
        map.insert(105, &83);
        map.insert(292, &116);
        map.insert(41, &115);
        map.insert(182, &82);
        map.insert(278, &75);
        map.insert(193, &97);
        map.insert(153, &32);
        map.insert(201, &114);
        map.insert(328, &111);
        map.insert(171, &99);
        map.insert(275, &32);
        map.insert(343, &110);
        map.insert(449, &100);
        map.insert(96, &116);
        map.insert(385, &116);
        map.insert(140, &115);

        map
    };
}

fn decrypt(key: [usize; 7]) {
    // Static hashmap is generated at compile time (too long to explain)
    let mut data: FxHashMap<usize, &u8> = STATIC_DATA4_HASHMAP.clone();
    let mut data_index: usize = 0;
    // TODO: predict output size based on the key and pass it as param 
    let mut result = ArrayString::<512>::new();
    // let mut data_removed_count: usize = 0;
    // let mut key_iterations_count: usize = 0;
    let mut key_index: usize = 0;
    let key_len: usize = key.len();
    
    for i in 0..data.len() {
        data_index = (data_index + key[key_index]) % data.len();
        key_index = (key_index + 1) % key_len;
        // Pedal to the metal, safety checks for losers
        unsafe {
            let value = data.remove(&data_index).unwrap_unchecked();
            // Updating keys on the "right" side of the hash map
            // I'm not sure if this worth it, but whatever...
            for i in (data_index + 1)..=data.len() {
                let old_value = data.remove(&i).unwrap_unchecked();
                data.insert(i - 1, old_value);
            }
            // I've implemented this method myself in the library to make it faster 
            // (IDE thinks it doesn't exist, but it does)
            result.try_push_unsafe(*value as char);
        }
    }
    if result.contains("DATA") && result.contains("test") {
        println!("GOT YOU!");
        println!("{}", result.to_string());
        println!();
    }
}

fn main() {
    // Ignore this wierdness. This is for testing only
    // let key: [usize; 7] = [24, 4, 25, 15, 25, 15, 25];
    let mut handles = Vec::new();
    // it's as shrimple as that
    for begining in 24..25 as usize {
        // Ignore this wierdness. This is for testing only
        println!("Starting thread: {}", begining);
        handles.push(thread::spawn(move || {
            for a in 1..27 as usize {
                for b in 1..27 as usize {
                    for c in 1..27 as usize {
                        for d in 1..27 as usize {
                            for e in 1..27 as usize {
                                for f in 1..27 as usize {
                                    let key: [usize; 7] = [begining, a, b, c, d, e, f];
                                    decrypt(key);
                                }
                            }
                        }
                    }
                }
            }
        }));
    }
    println!("Waiting for the threads");
    for handle in handles {
        handle.join().unwrap();
    }
    
}
