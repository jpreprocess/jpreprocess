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
interface UserDictionary {
  dict_da: Uint8Array,
  dict_vals: Uint8Array,
  words_idx_data: Uint8Array,
  words_data: Uint8Array,
}
"#;

#[derive(Serialize, Deserialize)]
struct JsDictionary {
    dict_da: Vec<u8>,
    dict_vals: Vec<u8>,
    cost_matrix: Vec<u8>,
    char_definitions: Vec<u8>,
    unknown_dictionary: Vec<u8>,
    words_idx_data: Vec<u8>,
    words_data: Vec<u8>,
}

impl TryFrom<JsDictionary> for lindera_core::dictionary::Dictionary {
    type Error = lindera_core::error::LinderaError;
    fn try_from(value: JsDictionary) -> Result<Self, Self::Error> {
        let this = Self {
            dict: PrefixDict::from_static_slice(&value.dict_da, &value.dict_vals),
            cost_matrix: ConnectionCostMatrix::load(&value.cost_matrix),
            char_definitions: CharacterDefinitions::load(&value.char_definitions)?,
            unknown_dictionary: UnknownDictionary::load(&value.unknown_dictionary)?,
            words_idx_data: Cow::Owned(value.words_idx_data),
            words_data: Cow::Owned(value.words_data),
        };
        Ok(this)
    }
}

#[derive(Serialize, Deserialize)]
struct JsUserDictionary {
    dict_da: Vec<u8>,
    dict_vals: Vec<u8>,
    words_idx_data: Vec<u8>,
    words_data: Vec<u8>,
}

impl TryFrom<JsUserDictionary> for lindera_core::dictionary::UserDictionary {
    type Error = lindera_core::error::LinderaError;
    fn try_from(value: JsUserDictionary) -> Result<Self, Self::Error> {
        let this = Self {
            dict: PrefixDict::from_static_slice(&value.dict_da, &value.dict_vals),
            words_idx_data: value.words_idx_data,
            words_data: value.words_data,
        };
        Ok(this)
    }
}

#[wasm_bindgen]
extern "C" {
    /// Dictionary data
    #[wasm_bindgen(typescript_type = "Dictionary")]
    pub type IDictionary;
    /// User dictionary data
    #[wasm_bindgen(typescript_type = "UserDictionary")]
    pub type IUserDictionary;
    /// Array of strings
    #[wasm_bindgen(typescript_type = "string[]")]
    pub type IVecString;
}

// TODO: Remove this when wasm-bindgen supports Vec<String> as function return type
impl<T: ToString> From<Vec<T>> for IVecString {
    fn from(value: Vec<T>) -> Self {
        let jsv = js_sys::Array::from_iter(value.into_iter().map(|v| JsValue::from(v.to_string())));
        IVecString { obj: jsv.into() }
    }
}
// TODO: Remove this when wasm-bindgen supports Vec<String> as function argument
impl From<IVecString> for Vec<String> {
    fn from(value: IVecString) -> Self {
        let array = js_sys::Array::from(&value.obj);
        array
            .iter()
            .map(js_sys::JsString::from)
            .filter_map(|jsstr| jsstr.as_string())
            .collect()
    }
}

#[wasm_bindgen]
pub struct JPreprocess {
    inner: jpreprocess::JPreprocess<jpreprocess::DefaultTokenizer>,
}
#[wasm_bindgen]
impl JPreprocess {
    #[wasm_bindgen(constructor)]
    pub fn new(
        system_dictionary: IDictionary,
        user_dictionary: Option<IUserDictionary>,
    ) -> Result<JPreprocess, JsValue> {
        let dictionary = {
            let dict: JsDictionary = serde_wasm_bindgen::from_value(system_dictionary.obj)?;
            dict.try_into().map_err(JsError::from)?
        };
        let user_dictionary = if let Some(user_dictionary) = user_dictionary {
            let dict: JsUserDictionary = serde_wasm_bindgen::from_value(user_dictionary.obj)?;
            Some(dict.try_into().map_err(JsError::from)?)
        } else {
            None
        };

        Ok(Self {
            inner: jpreprocess::JPreprocess::with_dictionaries(dictionary, user_dictionary),
        })
    }
    #[wasm_bindgen]
    pub fn run_frontend(&self, text: &str) -> Result<IVecString, JsValue> {
        let r = self.inner.run_frontend(text).map_err(JsError::from)?;
        Ok(r.into())
    }
    #[wasm_bindgen]
    pub fn make_label(&self, njd_features: IVecString) -> IVecString {
        let r = self.inner.make_label(njd_features.into());
        r.into()
    }
    #[wasm_bindgen]
    pub fn extract_fullcontext(&self, text: &str) -> Result<IVecString, JsValue> {
        let r = self
            .inner
            .extract_fullcontext(text)
            .map_err(JsError::from)?;
        Ok(r.into())
    }
}
