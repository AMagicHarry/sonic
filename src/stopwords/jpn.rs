// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Sonic OSS License v1.0 (SOSSL v1.0)

pub static STOPWORDS_JPN: &[&'static str] = &[
    "あそこ",
    "あっ",
    "あの",
    "あのかた",
    "あの人",
    "あり",
    "あります",
    "ある",
    "あれ",
    "い",
    "いう",
    "います",
    "いる",
    "う",
    "うち",
    "え",
    "お",
    "および",
    "おり",
    "おります",
    "か",
    "かつて",
    "から",
    "が",
    "き",
    "ここ",
    "こちら",
    "こと",
    "この",
    "これ",
    "これら",
    "さ",
    "さらに",
    "し",
    "しかし",
    "する",
    "ず",
    "せ",
    "せる",
    "そこ",
    "そして",
    "その",
    "その他",
    "その後",
    "それ",
    "それぞれ",
    "それで",
    "た",
    "ただし",
    "たち",
    "ため",
    "たり",
    "だ",
    "だっ",
    "だれ",
    "つ",
    "て",
    "で",
    "でき",
    "できる",
    "です",
    "では",
    "でも",
    "と",
    "という",
    "といった",
    "とき",
    "ところ",
    "として",
    "とともに",
    "とも",
    "と共に",
    "どこ",
    "どの",
    "な",
    "ない",
    "なお",
    "なかっ",
    "ながら",
    "なく",
    "なっ",
    "など",
    "なに",
    "なら",
    "なり",
    "なる",
    "なん",
    "に",
    "において",
    "における",
    "について",
    "にて",
    "によって",
    "により",
    "による",
    "に対して",
    "に対する",
    "に関する",
    "の",
    "ので",
    "のみ",
    "は",
    "ば",
    "へ",
    "ほか",
    "ほとんど",
    "ほど",
    "ます",
    "また",
    "または",
    "まで",
    "も",
    "もの",
    "ものの",
    "や",
    "よう",
    "より",
    "ら",
    "られ",
    "られる",
    "れ",
    "れる",
    "を",
    "ん",
    "何",
    "及び",
    "彼",
    "彼女",
    "我々",
    "特に",
    "私",
    "私達",
    "貴方",
    "貴方方",
];
