use std::{
    error::Error,
    fs::File,
    io::{self, Read},
    path::Path,
};

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use jpreprocess_njd::node_details::NodeDetails;

pub struct JPreproessBuilder;

impl JPreproessBuilder {
    pub fn generate_dictionary(output_dir: &Path) -> Result<(), Box<dyn Error>> {
        let mut idx_vec: Vec<u8> = Vec::new();
        let mut details_vec: Vec<u8> = Vec::new();

        {
            let mut idxs = Self::idx(output_dir)?;
            let mut prev_position = 0;

            let words_path = output_dir.join("dict.words");
            let mut words = File::open(words_path)?;

            idxs.push(words.metadata()?.len().try_into()?);

            for idx in idxs {
                let chunk_size = idx - prev_position;
                if chunk_size > 0 {
                    let mut data: Vec<String> =
                        bincode::deserialize_from(words.by_ref().take(chunk_size.into()))?;

                    data.resize(13, "".to_string());

                    let details =
                        NodeDetails::load(&data.iter().map(|d| &d[..]).collect::<Vec<&str>>()[..]);

                    idx_vec.write_u32::<LittleEndian>(details_vec.len().try_into()?)?;
                    bincode::serialize_into(&mut details_vec, &details)?;
                }

                prev_position = idx;
            }
        }
        dbg!(details_vec.len());
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
    fn idx(output_dir: &Path) -> Result<Vec<u32>, Box<dyn Error>> {
        let idx_path = output_dir.join("dict.wordsidx");
        let mut idx_file = File::open(idx_path)?;
        let mut idxs = Vec::new();
        loop {
            let mut chunk = Vec::with_capacity(4);
            let n = idx_file.by_ref().take(4).read_to_end(&mut chunk)?;
            if n != 4 {
                break;
            }
            idxs.push(LittleEndian::read_u32(&chunk));
        }
        Ok(idxs)
    }
}
