use bytemuck::NoUninit;

#[repr(align(16))] 
#[repr(C)]
#[derive(Debug, Clone, Copy, NoUninit)]
pub struct GpuAlignedValue(pub [u32; 4]);


pub const DATA4: &[u8] = "IvTuitn tat  eHenoEi pgvi IATf nAOiM pLl ssyGnelnu l(Eeo seIthHNc;YARsL oeeoEilnBst( elth dylarxtlSAPa YbSpNR'UrslCPth;B LiyROsoMpsF SrasCIssn KNEALhATih o  atowrstzCnisuhcAHKAm  ecIREftNsUMiNnan HeosirhHSs;AdtuuNpodpEo;pey r6T )rs(isg tyAi;sse;tFeCtutEj Soewuoe tMNR B Sedcl C KxeeCOxai; tnitLlLy tetncU OO ijwEiC:b7O;eUithx VOodAr )Tu xbsc:onD Nn tpsot KSeo;ae)DMEw2z tf EyfotdanrAeetwu7dIeB ;mnh:bA slu t WL ketAtlDnk;rNlU ILWeW7 iiDwUlnrTeEiolttdYT OtwEeDbUe i h HNfrAfc dVQdk ArzTOLarA if kye; tN sTLys oTd".as_bytes();
pub const GPU_ALIGNED_DATA4_LEN: usize = (DATA4.len() + DATA4.len() % 4) / 4;
// const DATA3: &str = "AtniotoMK;hHLt hOT(NSCCiMs  aEeMifpCesul)t: su|'yhlRtsW ;tFpb7t actt lbA  L|c  Ar:SlihA7Eof kh rK;LSPrfdswdrBrHekblUytasxKsnc 7uAROslOyELt SSUC;G Amu snOaACtrutyreebN;TPx thNIteelset;seis;  drs  nDdEpdltueUc And o c dyS7tdTlwFcA TItiePoA:(slpr(ltefKaIeeR  EdN;AvtetEE psottTCTEaCoAa IyYfsnouotsbUti sAkSetL;iui hNpnedped nIhcetx ;ttAeN lsytTnhKtD EeiaexRELityHU'Uux )e UMVr NVhtYiHc  rj Blu;Oe rN e pdsYNHIDo6NeS: D OOAlnn aC TC f So RPoGMsntsan EI LtE anoOiEeP    wDThxdt;CLp)eoEHo arS;wrwgiABeRzwAsue evirjOotiimo;L L( NEwros|oIpNiWeisoin;AiOAit2EQnyUlzsR TWRn rfCAeBEO ks PTW in|sngscthezHt eD)";
// const DATA2: &str = "at iatutsSfo e tT RSdsnaEcuNtseeLoEnoxxeh tAc oRHS; x iwDTUNen deetHOIECICraP e ;dSol Ot|GiMe: UytAEiYe; BPeb voY;sAc ie)Gesrc H M|eS;MHfuITsLMtWEzesAAt  epldn :Ei  eAseLLfp |ktnpmbslHseue SiicntatoNHCs(Ut Tett;ssw d s Crvs fA u  he yEf so;ndturaUsdeDlCeDAitlr);snn ohO:EAiWjl teax  wow   TaMNSis ncAe : sDx( psNpAEt;lo sBAisuAtdpxYetiR;rKEtnn sCiC  (|tttVtNo yhleswiSA)7SEe'bOnL Bo trLelicHgysn7tudotlrtLEshe tOgttd oipMaUfKr;waN:duEtlurys;RhOf7y nITiontsDaIKTkLeupHtr At ARisRiyDIHOV aHLWkClheICre   ldmtbO )A PTwps R ne AtrUoDT AEsH;Nnozslehh(erDTFOK teAFnn;cRpE ;tOOrORNTMcTdh2E'I(LTnobAphj NcyyAa6Pk NEiSN Ir7CNSYRdt KEo)TexEETiSPUoCeBO zLIUtU;lhPQrCAIi tBWsdM;";


lazy_static! {
	pub static ref GPU_ALIGNED_DATA4: [GpuAlignedValue; GPU_ALIGNED_DATA4_LEN] = {
		println!("Aligning DATA4 to work properly on the GPU. {} zeros will be added to the end", DATA4.len() % 4);
		// Just trust me...
		return DATA4
				.chunks(4)
				.into_iter()
				.map(|nums| {
					// Handling a case where current chunck len() isn't equal to 4
					let mut sequence: [u32; 4] = [0_u32; 4];
					for i in 0..nums.len() {
						sequence[i] = nums[i] as u32;
					}

					return GpuAlignedValue(sequence);
				})
				.collect::<Vec<GpuAlignedValue>>()
				.try_into()
				.unwrap();
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