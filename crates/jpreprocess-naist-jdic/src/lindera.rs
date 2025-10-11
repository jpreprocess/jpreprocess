use lindera_dictionary::{
    decompress::{decompress, CompressedData},
    dictionary::{
        character_definition::CharacterDefinition, connection_cost_matrix::ConnectionCostMatrix,
        metadata::Metadata, prefix_dictionary::PrefixDictionary,
        unknown_dictionary::UnknownDictionary, Dictionary,
    },
    util::Data,
    LinderaResult,
};

const METADATA_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/metadata.json"));

const CHAR_DEFINITION_DATA: &[u8] =
    include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/char_def.bin"));
const CONNECTION_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/matrix.mtx"));
const IPADIC_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.da"));
const IPADIC_VALS: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.vals"));
const UNKNOWN_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/unk.bin"));
const WORDS_IDX_DATA: &[u8] =
    include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.wordsidx"));
const WORDS_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.words"));

fn try_decompress(data: &'static [u8]) -> LinderaResult<Data> {
    match bincode::serde::decode_from_slice::<CompressedData, _>(data, bincode::config::legacy()) {
        Ok((compressed_data, _)) => {
            // Successfully decoded as CompressedData, now decompress it
            match decompress(compressed_data) {
                Ok(decompressed) => Ok(Data::Vec(decompressed)),
                Err(_) => {
                    // Decompression failed, fall back to raw data
                    Ok(Data::Static(data))
                }
            }
        }
        Err(_) => {
            // Not compressed data format, use as raw binary
            Ok(Data::Static(data))
        }
    }
}

pub fn load() -> LinderaResult<Dictionary> {
    Ok(Dictionary {
        metadata: Metadata::load(METADATA_DATA)?,
        prefix_dictionary: PrefixDictionary::load(
            try_decompress(IPADIC_DATA)?,
            try_decompress(IPADIC_VALS)?,
            try_decompress(WORDS_IDX_DATA)?,
            try_decompress(WORDS_DATA)?,
            true,
        ),
        connection_cost_matrix: ConnectionCostMatrix::load(try_decompress(CONNECTION_DATA)?),
        character_definition: CharacterDefinition::load(&try_decompress(CHAR_DEFINITION_DATA)?)?,
        unknown_dictionary: UnknownDictionary::load(&try_decompress(UNKNOWN_DATA)?)?,
    })
}
