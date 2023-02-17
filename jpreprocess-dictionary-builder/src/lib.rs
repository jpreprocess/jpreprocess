use std::{
    error::Error,
    fs::File,
    io,
    path::Path,
};

use byteorder::{LittleEndian, WriteBytesExt};
use jpreprocess_dictionary::{DictionaryTrait, LinderaDict};
use jpreprocess_njd::node_details::NodeDetails;

pub struct JPreproessBuilder;

impl JPreproessBuilder {
    pub fn generate_dictionary(output_dir: &Path) -> Result<(), Box<dyn Error>> {
        let lindera_dict = LinderaDict::load(output_dir.to_path_buf())?;

        let mut idx_vec: Vec<u8> = Vec::new();
        let mut details_vec: Vec<u8> = Vec::new();

        {
            for mut data in lindera_dict.iter() {
                data.resize(13, "".to_string());

                let details =
                    NodeDetails::load(&data.iter().map(|d| &d[..]).collect::<Vec<&str>>()[..]);

                idx_vec.write_u32::<LittleEndian>(details_vec.len().try_into()?)?;
                bincode::serialize_into(&mut details_vec, &details)?;
            }
        }
        {
            use std::io::Write;
            let mut result_idx =
                io::BufWriter::new(File::create(output_dir.join("jpreprocess.wordsidx"))?);
            result_idx.write(&idx_vec)?;
            result_idx.flush()?;
            let mut result_words =
                io::BufWriter::new(File::create(output_dir.join("jpreprocess.words"))?);
            result_words.write(&details_vec)?;
            result_words.flush()?;
        }
        Ok(())
    }
}
