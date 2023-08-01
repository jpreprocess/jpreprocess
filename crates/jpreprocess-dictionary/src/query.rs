use lindera_core::{
    dictionary::{Dictionary, UserDictionary},
    word_entry::WordId,
};
use lindera_tokenizer::token::Token;

pub trait DictionaryQuery {
    fn word_id(&self) -> WordId;
    fn dictionary(&self) -> &Dictionary;
    fn user_dictionary(&self) -> Option<&UserDictionary>;
}

impl DictionaryQuery for Token<'_> {
    fn word_id(&self) -> WordId {
        self.word_id
    }
    fn dictionary(&self) -> &Dictionary {
        self.dictionary
    }
    fn user_dictionary(&self) -> Option<&UserDictionary> {
        self.user_dictionary
    }
}
