pub enum DictionaryIdent {
    Lindera,
    JPreprocess,
}

impl DictionaryIdent {
    pub fn from_ident_str(ident: Option<&str>) -> Self {
        match ident {
            Some("") => DictionaryIdent::Lindera,
            Some(ident) if ident.to_lowercase().starts_with("jpreprocess") => {
                DictionaryIdent::JPreprocess
            }
            None => {
                eprintln!("Could not parse dictionary type as utf8");
                DictionaryIdent::Lindera
            }
            Some(s) => {
                eprintln!("Unknown dictionary type: {s}. Defaulting to Lindera tokenizer.");
                DictionaryIdent::Lindera
            }
        }
    }
}
