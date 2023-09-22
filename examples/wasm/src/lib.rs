use std::borrow::Cow;

use lindera_core::{
    character_definition::CharacterDefinitions, connection::ConnectionCostMatrix,
    prefix_dict::PrefixDict, unknown_dictionary::UnknownDictionary,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT: &'static str = r#"
interface Dictionary {
    dict_da: Uint8Array,
    dict_vals: Uint8Array,
    cost_matrix: Uint8Array,
    char_definitions: Uint8Array,
    unknown_dictionary: Uint8Array,
    words_idx_data: Uint8Array,
    words_data: Uint8Array,
}
type VecString = string[];
"#;

#[derive(Serialize, Deserialize)]
struct Dictionary {
    dict_da: Vec<u8>,
    dict_vals: Vec<u8>,
    cost_matrix: Vec<u8>,
    char_definitions: Vec<u8>,
    unknown_dictionary: Vec<u8>,
    words_idx_data: Vec<u8>,
    words_data: Vec<u8>,
}

impl From<Dictionary> for lindera_core::dictionary::Dictionary {
    fn from(value: Dictionary) -> Self {
        Self {
            dict: PrefixDict::from_static_slice(&value.dict_da, &value.dict_vals),
            cost_matrix: ConnectionCostMatrix::load(&value.cost_matrix),
            char_definitions: CharacterDefinitions::load(&value.char_definitions).unwrap(),
            unknown_dictionary: UnknownDictionary::load(&value.unknown_dictionary).unwrap(),
            words_idx_data: Cow::Owned(value.words_idx_data),
            words_data: Cow::Owned(value.words_data),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Dictionary")]
    pub type IDictionary;
    #[wasm_bindgen(typescript_type = "VecString")]
    pub type IVecString;
}

fn vecstring2js(input: Vec<String>) -> IVecString {
    let jsv = js_sys::Array::from_iter(input.into_iter().map(js_sys::JsString::from));
    IVecString { obj: jsv.into() }
}
fn js2vecstring(input: IVecString) -> Vec<String> {
    let array = js_sys::Array::from(&input.obj);
    array
        .iter()
        .map(|elem| js_sys::JsString::from(elem))
        .filter_map(|jsstr| jsstr.as_string())
        .collect()
}

#[wasm_bindgen]
pub struct JPreprocess {
    inner: jpreprocess::JPreprocess,
}
#[wasm_bindgen]
impl JPreprocess {
    #[wasm_bindgen(constructor)]
    pub fn new(system_dictionary: IDictionary) -> Result<JPreprocess, JsValue> {
        let sysdic: Dictionary = serde_wasm_bindgen::from_value(system_dictionary.obj)?;
        Ok(Self {
            inner: jpreprocess::JPreprocess::with_dictionaries(sysdic.into(), None),
        })
    }
    #[wasm_bindgen]
    pub fn run_frontend(&self, text: &str) -> Result<IVecString, JsValue> {
        let r = self
            .inner
            .run_frontend(text)
            .map_err(|err| JsError::from(err))?;

        Ok(vecstring2js(r))
    }
    #[wasm_bindgen]
    pub fn make_label(&self, njd_features: IVecString) -> IVecString {
        let r = self.inner.make_label(js2vecstring(njd_features));
        vecstring2js(r)
    }
    #[wasm_bindgen]
    pub fn extract_fullcontext(&self, text: &str) -> Result<IVecString, JsValue> {
        let r = self
            .inner
            .extract_fullcontext(text)
            .map_err(|err| JsError::from(err))?;

        Ok(vecstring2js(r))
    }
}
