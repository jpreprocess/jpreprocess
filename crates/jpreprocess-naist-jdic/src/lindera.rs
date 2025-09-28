use lindera_dictionary::{
    dictionary::{
        character_definition::CharacterDefinition, connection_cost_matrix::ConnectionCostMatrix,
        metadata::Metadata, prefix_dictionary::PrefixDictionary,
        unknown_dictionary::UnknownDictionary, Dictionary,
    },
    LinderaResult,
};

const METADATA_DATA: &[u8] = include_bytes!(concat!(
    env!("JPREPROCESS_WORKDIR"),
    "/metadata.json"
));

const CHAR_DEFINITION_DATA: &[u8] =
    include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/char_def.bin"));
const CONNECTION_DATA: &[u8] =
    include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/matrix.mtx"));
const IPADIC_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.da"));
const IPADIC_VALS: &[u8] =
    include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.vals"));
const UNKNOWN_DATA: &[u8] = include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/unk.bin"));
const WORDS_IDX_DATA: &[u8] = include_bytes!(concat!(
    env!("JPREPROCESS_WORKDIR"),
    "/dict.wordsidx"
));
const WORDS_DATA: &[u8] =
    include_bytes!(concat!(env!("JPREPROCESS_WORKDIR"), "/dict.words"));

#[cfg(feature = "compress")]
fn try_decompress(data: &[u8]) -> LinderaResult<Vec<u8>> {
    use lindera_dictionary::{
        decompress::{CompressedData, decompress},
    };
    match bincode::serde::decode_from_slice::<CompressedData, _>(
        &data[..],
        bincode::config::legacy(),
    ) {
        Ok((compressed_data, _)) => {
            // Successfully decoded as CompressedData, now decompress it
            match decompress(compressed_data) {
                Ok(decompressed) => Ok(decompressed),
                Err(_) => {
                    // Decompression failed, fall back to raw data
                    Ok(data.to_vec())
                }
            }
        }
        Err(_) => {
            // Not compressed data format, use as raw binary
            Ok(data.to_vec())
        }
    }
}

pub fn load() {
    let metadata = Metadata::load(METADATA_DATA).unwrap();
}

// pub fn load_dictionary() -> LinderaResult<Dictionary> {
//     Ok(Dictionary {
//         prefix_dictionary: prefix_dict(),
//         connection_cost_matrix: connection(),
//         character_definition: char_def()?,
//         unknown_dictionary: unknown_dict()?,
//     })
// }

// pub fn char_def() -> LinderaResult<CharacterDefinition> {
//     CharacterDefinition::load(CHAR_DEFINITION_DATA)
// }

// pub fn connection() -> ConnectionCostMatrix {
//     ConnectionCostMatrix::load(CONNECTION_DATA)
// }

// pub fn prefix_dict() -> PrefixDictionary {
//     PrefixDictionary::load(IPADIC_DATA, IPADIC_VALS, WORDS_IDX_DATA, WORDS_DATA, true)
// }

// pub fn unknown_dict() -> LinderaResult<UnknownDictionary> {
//     UnknownDictionary::load(UNKNOWN_DATA)
// }
