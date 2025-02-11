use jpreprocess::NJD;
use jpreprocess_core::pronunciation::{Mora, MoraEnum};

#[test]
fn shik() {
    let mut njd: NJD = [
        "少,接頭詞,名詞接続,*,*,*,*,少,ショウ,ショー,0/2,P2,-1",
        "し金,名詞,一般,*,*,*,*,し金,シキン,シキン,1/3,C1,-1"
    ]
    .into_iter()
    .collect();

    njd.preprocess();

    for node in &njd.nodes {
        println!("{}", node);
    }

    let pron = njd.nodes[1].get_pron();
    assert_eq!(
        pron.moras()[0],
        Mora {
            mora_enum: MoraEnum::Shi,
            is_voiced: false
        }
    );
}
