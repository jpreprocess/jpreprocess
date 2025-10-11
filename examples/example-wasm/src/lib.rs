use lindera::LinderaResult;
use lindera_dictionary::{
    decompress::decompress,
    dictionary::{
        character_definition::CharacterDefinition, connection_cost_matrix::ConnectionCostMatrix,
        metadata::Metadata, prefix_dictionary::PrefixDictionary,
        unknown_dictionary::UnknownDictionary,
    },
    error::LinderaErrorKind,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT: &'static str = r#"
interface Dictionary {
  metadata: Metadata,
  dict_da: Uint8Array,
  dict_vals: Uint8Array,
  cost_matrix: Uint8Array,
  char_definitions: Uint8Array,
  unknown_dictionary: Uint8Array,
  words_idx_data: Uint8Array,
  words_data: Uint8Array,
}
interface UserDictionary {
  metadata: Metadata,
  dict_da: Uint8Array,
  dict_vals: Uint8Array,
  words_idx_data: Uint8Array,
  words_data: Uint8Array,
}
interface Metadata {
  name: string;
  encoding: string;
  compress_algorithm: "deflate" | "zlib" | "gzip" | "raw";
  default_word_cost: number;
  default_left_context_id: number;
  default_right_context_id: number;
  default_field_value: string;
  flexible_csv: boolean;
  skip_invalid_cost_or_id: boolean;
  normalize_details: boolean;
  dictionary_schema: Schema;
  user_dictionary_schema: Schema;
  model_info?: any;
}
interface Schema {
  fields: string[];
}
"#;

fn decompress_if_compressed(data: &[u8]) -> LinderaResult<Vec<u8>> {
    if let Ok((compressed_data, _)) =
        bincode::serde::decode_from_slice(data, bincode::config::legacy())
    {
        decompress(compressed_data).map_err(|err| {
            LinderaErrorKind::Compression
                .with_error(err)
                .add_context("Failed to decompress data")
        })
    } else {
        Ok(data.to_vec())
    }
}

#[derive(Serialize, Deserialize)]
struct JsDictionary {
    metadata: Metadata,
    dict_da: Vec<u8>,
    dict_vals: Vec<u8>,
    cost_matrix: Vec<u8>,
    char_definitions: Vec<u8>,
    unknown_dictionary: Vec<u8>,
    words_idx_data: Vec<u8>,
    words_data: Vec<u8>,
}

impl TryFrom<JsDictionary> for lindera::dictionary::Dictionary {
    type Error = lindera::error::LinderaError;
    fn try_from(value: JsDictionary) -> Result<Self, Self::Error> {
        let this = Self {
            metadata: value.metadata,
            prefix_dictionary: PrefixDictionary::load(
                decompress_if_compressed(&value.dict_da)?,
                decompress_if_compressed(&value.dict_vals)?,
                decompress_if_compressed(&value.words_idx_data)?,
                decompress_if_compressed(&value.words_data)?,
                true,
            ),
            connection_cost_matrix: ConnectionCostMatrix::load(decompress_if_compressed(
                &value.cost_matrix,
            )?),
            character_definition: CharacterDefinition::load(&decompress_if_compressed(
                &value.char_definitions,
            )?)?,
            unknown_dictionary: UnknownDictionary::load(&decompress_if_compressed(
                &value.unknown_dictionary,
            )?)?,
        };
        Ok(this)
    }
}

#[derive(Serialize, Deserialize)]
struct JsUserDictionary {
    metadata: Metadata,
    dict_da: Vec<u8>,
    dict_vals: Vec<u8>,
    words_idx_data: Vec<u8>,
    words_data: Vec<u8>,
}

impl TryFrom<JsUserDictionary> for lindera::dictionary::UserDictionary {
    type Error = lindera::error::LinderaError;
    fn try_from(value: JsUserDictionary) -> Result<Self, Self::Error> {
        let this = Self {
            dict: PrefixDictionary::load(
                value.dict_da,
                value.dict_vals,
                value.words_idx_data,
                value.words_data,
                false,
            ),
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
