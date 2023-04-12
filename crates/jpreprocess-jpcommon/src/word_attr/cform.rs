use jpreprocess_core::cform::CForm;

pub fn cform_to_id(cform: &CForm) -> Option<u8> {
    match cform {
        // *:xx
        CForm::None => None,
        // その他:6
        CForm::ConjunctionGaru => Some(6),
        // 仮定形:4
        CForm::Conditional => Some(4),
        CForm::ConditionalContraction1 => Some(4),
        CForm::ConditionalContraction2 => Some(4),
        // 基本形:2
        CForm::Basic => Some(2),
        CForm::BasicDoubledConsonant => Some(2),
        CForm::BasicModern => Some(2),
        CForm::BasicEuphony => Some(2),
        CForm::BasicOld => Some(2),
        // 未然形:0
        CForm::Mizen => Some(0),
        CForm::MizenConjunctionU => Some(0),
        CForm::MizenConjunctionNu => Some(0),
        CForm::MizenConjunctionReru => Some(0),
        CForm::MizenSpecial => Some(0),
        // 命令形:5
        CForm::ImperativeE => Some(5),
        CForm::ImperativeI => Some(5),
        CForm::ImperativeRo => Some(5),
        CForm::ImperativeYo => Some(5),
        // 連体形:3
        CForm::TaigenConjunction => Some(3),
        CForm::TaigenConjunctionSpecial => Some(3),
        CForm::TaigenConjunctionSpecial2 => Some(3),
        // 連用形:1
        CForm::Renyou => Some(1),
        CForm::RenyouConjunctionGozai => Some(1),
        CForm::RenyouConjunctionTa => Some(1),
        CForm::RenyouConjunctionTe => Some(1),
        CForm::RenyouConjunctionDe => Some(1),
        CForm::RenyouConjunctionNi => Some(1),
    }
}
