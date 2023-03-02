const DATA4: &str = "IvTuitn tat  eHenoEi pgvi IATf nAOiM pLl ssyGnelnu l(Eeo seIthHNc;YARsL oeeoEilnBst( elth dylarxtlSAPa YbSpNR'UrslCPth;B LiyROsoMpsF SrasCIssn KNEALhATih o  atowrstzCnisuhcAHKAm  ecIREftNsUMiNnan HeosirhHSs;AdtuuNpodpEo;pey r6T )rs(isg tyAi;sse;tFeCtutEj Soewuoe tMNR B Sedcl C KxeeCOxai; tnitLlLy tetncU OO ijwEiC:b7O;eUithx VOodAr )Tu xbsc:onD Nn tpsot KSeo;ae)DMEw2z tf EyfotdanrAeetwu7dIeB ;mnh:bA slu t WL ketAtlDnk;rNlU ILWeW7 iiDwUlnrTeEiolttdYT OtwEeDbUe i h HNfrAfc dVQdk ArzTOLarA if kye; tN sTLys oTd";
const DATA3: &str = "AtniotoMK;hHLt hOT(NSCCiMs  aEeMifpCesul)t: su|'yhlRtsW ;tFpb7t actt lbA  L|c  Ar:SlihA7Eof kh rK;LSPrfdswdrBrHekblUytasxKsnc 7uAROslOyELt SSUC;G Amu snOaACtrutyreebN;TPx thNIteelset;seis;  drs  nDdEpdltueUc And o c dyS7tdTlwFcA TItiePoA:(slpr(ltefKaIeeR  EdN;AvtetEE psottTCTEaCoAa IyYfsnouotsbUti sAkSetL;iui hNpnedped nIhcetx ;ttAeN lsytTnhKtD EeiaexRELityHU'Uux )e UMVr NVhtYiHc  rj Blu;Oe rN e pdsYNHIDo6NeS: D OOAlnn aC TC f So RPoGMsntsan EI LtE anoOiEeP    wDThxdt;CLp)eoEHo arS;wrwgiABeRzwAsue evirjOotiimo;L L( NEwros|oIpNiWeisoin;AiOAit2EQnyUlzsR TWRn rfCAeBEO ks PTW in|sngscthezHt eD)";
const DATA2: &str = "at iatutsSfo e tT RSdsnaEcuNtseeLoEnoxxeh tAc oRHS; x iwDTUNen deetHOIECICraP e ;dSol Ot|GiMe: UytAEiYe; BPeb voY;sAc ie)Gesrc H M|eS;MHfuITsLMtWEzesAAt  epldn :Ei  eAseLLfp |ktnpmbslHseue SiicntatoNHCs(Ut Tett;ssw d s Crvs fA u  he yEf so;ndturaUsdeDlCeDAitlr);snn ohO:EAiWjl teax  wow   TaMNSis ncAe : sDx( psNpAEt;lo sBAisuAtdpxYetiR;rKEtnn sCiC  (|tttVtNo yhleswiSA)7SEe'bOnL Bo trLelicHgysn7tudotlrtLEshe tOgttd oipMaUfKr;waN:duEtlurys;RhOf7y nITiontsDaIKTkLeupHtr At ARisRiyDIHOV aHLWkClheICre   ldmtbO )A PTwps R ne AtrUoDT AEsH;Nnozslehh(erDTFOK teAFnn;cRpE ;tOOrORNTMcTdh2E'I(LTnobAphj NcyyAa6Pk NEiSN Ir7CNSYRdt KEo)TexEETiSPUoCeBO zLIUtU;lhPQrCAIi tBWsdM;";


pub const DATA4_GPU_ALIGNED: &[u32; 512] = &[73, 118, 84, 117, 105, 116, 110, 32, 116, 97, 116, 32, 32, 101, 72, 101, 110, 111, 69, 105, 32, 112, 103, 118, 105, 32, 73, 65, 84, 102, 32, 110, 65, 79, 105, 77, 32, 112, 76, 108, 32, 115, 115, 121, 71, 110, 101, 108, 110, 117, 32, 108, 40, 69, 101, 111, 32, 115, 101, 73, 116, 104, 72, 78, 99, 59, 89, 65, 82, 115, 76, 32, 111, 101, 101, 111, 69, 105, 108, 110, 66, 115, 116, 40, 32, 101, 108, 116, 104, 32, 100, 121, 108, 97, 114, 120, 116, 108, 83, 65, 80, 97, 32, 89, 98, 83, 112, 78, 82, 39, 85, 114, 115, 108, 67, 80, 116, 104, 59, 66, 32, 76, 105, 121, 82, 79, 115, 111, 77, 112, 115, 70, 32, 83, 114, 97, 115, 67, 73, 115, 115, 110, 32, 75, 78, 69, 65, 76, 104, 65, 84, 105, 104, 32, 111, 32, 32, 97, 116, 111, 119, 114, 115, 116, 122, 67, 110, 105, 115, 117, 104, 99, 65, 72, 75, 65, 109, 32, 32, 101, 99, 73, 82, 69, 102, 116, 78, 115, 85, 77, 105, 78, 110, 97, 110, 32, 72, 101, 111, 115, 105, 114, 104, 72, 83, 115, 59, 65, 100, 116, 117, 117, 78, 112, 111, 100, 112, 69, 111, 59, 112, 101, 121, 32, 114, 54, 84, 32, 41, 114, 115, 40, 105, 115, 103, 32, 116, 121, 65, 105, 59, 115, 115, 101, 59, 116, 70, 101, 67, 116, 117, 116, 69, 106, 32, 83, 111, 101, 119, 117, 111, 101, 32, 116, 77, 78, 82, 32, 66, 32, 83, 101, 100, 99, 108, 32, 67, 32, 75, 120, 101, 101, 67, 79, 120, 97, 105, 59, 32, 116, 110, 105, 116, 76, 108, 76, 121, 32, 116, 101, 116, 110, 99, 85, 32, 79, 79, 32, 105, 106, 119, 69, 105, 67, 58, 98, 55, 79, 59, 101, 85, 105, 116, 104, 120, 32, 86, 79, 111, 100, 65, 114, 32, 41, 84, 117, 32, 120, 98, 115, 99, 58, 111, 110, 68, 32, 78, 110, 32, 116, 112, 115, 111, 116, 32, 75, 83, 101, 111, 59, 97, 101, 41, 68, 77, 69, 119, 50, 122, 32, 116, 102, 32, 69, 121, 102, 111, 116, 100, 97, 110, 114, 65, 101, 101, 116, 119, 117, 55, 100, 73, 101, 66, 32, 59, 109, 110, 104, 58, 98, 65, 32, 115, 108, 117, 32, 116, 32, 87, 76, 32, 107, 101, 116, 65, 116, 108, 68, 110, 107, 59, 114, 78, 108, 85, 32, 73, 76, 87, 101, 87, 55, 32, 105, 105, 68, 119, 85, 108, 110, 114, 84, 101, 69, 105, 111, 108, 116, 116, 100, 89, 84, 32, 79, 116, 119, 69, 101, 68, 98, 85, 101, 32, 105, 32, 104, 32, 72, 78, 102, 114, 65, 102, 99, 32, 100, 86, 81, 100, 107, 32, 65, 114, 122, 84, 79, 76, 97, 114, 65, 32, 105, 102, 32, 107, 121, 101, 59, 32, 116, 78, 32, 115, 84, 76, 121, 115, 32, 111, 84, 100, 0];

// WARNING: I KNOW. Shut up
pub const DATA4_LEN: usize = DATA4.len();
pub const DATA3_LEN: usize = DATA3.len();
pub const DATA2_LEN: usize = DATA2.len();

// DATA strings as vecs
lazy_static! {
    pub static ref STATIC_DATA4_VEC: Vec<u8> = {
        return DATA4.as_bytes().to_vec();
    };

    pub static ref STATIC_DATA3_VEC: Vec<u8> = {
        return DATA3.as_bytes().to_vec();
    };

    pub static ref STATIC_DATA2_VEC: Vec<u8> = {
        return DATA2.as_bytes().to_vec();
    };
}
// DATA_HASHMAP
lazy_static! {
    /*
    NOTE: Was removed, but if you need to generate a hashmap use this...

    Rust code for getting rust_data string for python:
    let data_hash_map = HashMap::from_iter(DATA4.as_bytes().iter().enumerate());
    println!("{:?}", data_hash_map);

    Python code:
    rust_data = "42"
    rust_data = rust_data.replace('\n', '').replace('\n', '').split(',')
    result = ""
    for line in rust_data:
        cur = line.strip()
        num1, num2 = cur.split(": ")
        result += f"map.insert({num1}, &{num2});\n"
    print(result)
    */
}

// NOTE: In future caclulate max_keyword_length in advance and pass it as a parametr?
/// Returns true if decrypted message contains ANY given keyword
pub fn l2a_accurate(data_source: &Vec<u8>, key: &Vec<usize>, keywords: &Vec<&[u8]>) -> bool {
	// Keeps track of last N data values to check for keywords (no need to store entire encryption result)
	let max_keyword_length: usize = (*keywords)
										.iter()
										.max_by_key(|x| x.len())
										.unwrap()
										.len();
	let mut current_data_slice = vec![0_u8; max_keyword_length];

	let mut data = data_source.clone();
    let mut data_index: usize = 0;

    let mut key_index: usize = 0;
    let key_len: usize = key.len();
    
    for i in (1..=data.len()).rev() {
		// 1) Calculate indicies
        data_index = (data_index + key[key_index]) % i;
        key_index = (key_index + 1) % key_len;
		// 2) Check for keywords
		current_data_slice.push(data[data_index]);
		current_data_slice.remove(0);
		// TODO: move to a different function or whatever?
		for keyword in keywords {
			let mut is_matching = true;

			for i in 0..keyword.len() {
				if keyword[i] != current_data_slice[i] {
					is_matching = false;
					break;
				}
			}
			if is_matching {
				return true;
			}
		}
		// 3) Updating data
		data.remove(data_index);
    }

	false
}

/// todo!()
pub fn l2a_inaccurate(data: &Vec<u8>, key: &Vec<usize>, keywords: &Vec<&[u8]>) -> bool {
	// TODO: Should I also use -1 in GPU version to avoid having overflows
	let output_size = fl2a_output_chars_amount(key, data.len());
	// Keeps track of last N data values to check for keywords (no need to store entire encryption result)
	let max_keyword_length: usize = (*keywords)
										.iter()
										.max_by_key(|x| x.len())
										.unwrap()
										.len();
	let mut current_data_slice = vec![0_u8; max_keyword_length];

	// TODO: explain maybe
    let mut data_index: usize = 0;
	let mut printable_data_index: usize = 0;
    let mut data_removed_count: usize = 0;

    let mut key_index: usize = 0;
    let key_len: usize = key.len();
    
    for i in ((data.len() - output_size)..=data.len()).rev() {
		// 1) Calculate indicies
        data_index = (data_index + key[key_index]) % i;
		printable_data_index = data_index + data_removed_count;
        key_index = (key_index + 1) % key_len;
		// 2) Check for keywords
		current_data_slice.push(data[printable_data_index]);
		current_data_slice.remove(0);
		// TODO: move to a different function or whatever?
		for keyword in keywords {
			let mut is_matching = true;

			for i in 0..keyword.len() {
				if keyword[i] != current_data_slice[i] {
					is_matching = false;
					break;
				}
			}
			if is_matching {
				return true;
			}
		}
		// 3) Emulate remove operation
        print!("{}", data[printable_data_index] as char);
        data_removed_count += 1;
    }

	false
}

// Calculates how many chars can be safely decrypted using inacurate (fast) version of l2a
// (Thanks __noop__ :)
fn fl2a_output_chars_amount(key: &Vec<usize>, data_length: usize) -> usize {
	let mut result = 0;
	let mut current_index = 0;

	loop {
		for key_part in key {
			current_index += key_part + 1;
			if current_index > data_length {
				return result;
			}
			result += 1;
		}
	}
}