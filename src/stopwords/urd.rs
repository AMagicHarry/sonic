// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub static STOPWORDS_URD: &[&'static str] = &[
    "آئی",
    "آئے",
    "آج",
    "آخر",
    "آخرکبر",
    "آدهی",
    "آًب",
    "آٹھ",
    "آیب",
    "اة",
    "اخبزت",
    "اختتبم",
    "ادھر",
    "ارد",
    "اردگرد",
    "ارکبى",
    "اش",
    "اضتعوبل",
    "اضتعوبلات",
    "اضطرذ",
    "اضکب",
    "اضکی",
    "اضکے",
    "اطراف",
    "اغیب",
    "افراد",
    "الگ",
    "اور",
    "اوًچب",
    "اوًچبئی",
    "اوًچی",
    "اوًچے",
    "اى",
    "اً",
    "اًذر",
    "اًہیں",
    "اٹھبًب",
    "اپٌب",
    "اپٌے",
    "اچھب",
    "اچھی",
    "اچھے",
    "اکثر",
    "اکٹھب",
    "اکٹھی",
    "اکٹھے",
    "اکیلا",
    "اکیلی",
    "اکیلے",
    "اگرچہ",
    "اہن",
    "ایطے",
    "ایک",
    "ب",
    "ت",
    "تبزٍ",
    "تت",
    "تر",
    "ترتیت",
    "تریي",
    "تعذاد",
    "تن",
    "تو",
    "توبم",
    "توہی",
    "توہیں",
    "تٌہب",
    "تک",
    "تھب",
    "تھوڑا",
    "تھوڑی",
    "تھوڑے",
    "تھی",
    "تھے",
    "تیي",
    "ثب",
    "ثبئیں",
    "ثبترتیت",
    "ثبری",
    "ثبرے",
    "ثبعث",
    "ثبلا",
    "ثبلترتیت",
    "ثبہر",
    "ثدبئے",
    "ثرآں",
    "ثراں",
    "ثرش",
    "ثعذ",
    "ثغیر",
    "ثلٌذ",
    "ثلٌذوثبلا",
    "ثلکہ",
    "ثي",
    "ثٌب",
    "ثٌبرہب",
    "ثٌبرہی",
    "ثٌبرہے",
    "ثٌبًب",
    "ثٌذ",
    "ثٌذکرو",
    "ثٌذکرًب",
    "ثٌذی",
    "ثڑا",
    "ثڑوں",
    "ثڑی",
    "ثڑے",
    "ثھر",
    "ثھرا",
    "ثھراہوا",
    "ثھرپور",
    "ثھی",
    "ثہت",
    "ثہتر",
    "ثہتری",
    "ثہتریي",
    "ثیچ",
    "ج",
    "خب",
    "خبرہب",
    "خبرہی",
    "خبرہے",
    "خبهوظ",
    "خبًب",
    "خبًتب",
    "خبًتی",
    "خبًتے",
    "خبًٌب",
    "خت",
    "ختن",
    "خجکہ",
    "خص",
    "خططرذ",
    "خلذی",
    "خو",
    "خواى",
    "خوًہی",
    "خوکہ",
    "خٌبة",
    "خگہ",
    "خگہوں",
    "خگہیں",
    "خیطب",
    "خیطبکہ",
    "در",
    "درخبت",
    "درخہ",
    "درخے",
    "درزقیقت",
    "درضت",
    "دش",
    "دفعہ",
    "دلچطپ",
    "دلچطپی",
    "دلچطپیبں",
    "دو",
    "دور",
    "دوراى",
    "دوضرا",
    "دوضروں",
    "دوضری",
    "دوضرے",
    "دوًوں",
    "دکھبئیں",
    "دکھبتب",
    "دکھبتی",
    "دکھبتے",
    "دکھبو",
    "دکھبًب",
    "دکھبیب",
    "دی",
    "دیب",
    "دیتب",
    "دیتی",
    "دیتے",
    "دیر",
    "دیٌب",
    "دیکھو",
    "دیکھٌب",
    "دیکھی",
    "دیکھیں",
    "دے",
    "ر",
    "راضتوں",
    "راضتہ",
    "راضتے",
    "رریعہ",
    "رریعے",
    "رکي",
    "رکھ",
    "رکھب",
    "رکھتب",
    "رکھتبہوں",
    "رکھتی",
    "رکھتے",
    "رکھی",
    "رکھے",
    "رہب",
    "رہی",
    "رہے",
    "ز",
    "زبصل",
    "زبضر",
    "زبل",
    "زبلات",
    "زبلیہ",
    "زصوں",
    "زصہ",
    "زصے",
    "زقبئق",
    "زقیتیں",
    "زقیقت",
    "زکن",
    "زکویہ",
    "زیبدٍ",
    "صبف",
    "صسیر",
    "صفر",
    "صورت",
    "صورتسبل",
    "صورتوں",
    "صورتیں",
    "ض",
    "ضبت",
    "ضبتھ",
    "ضبدٍ",
    "ضبرا",
    "ضبرے",
    "ضبل",
    "ضبلوں",
    "ضت",
    "ضرور",
    "ضرورت",
    "ضروری",
    "ضلطلہ",
    "ضوچ",
    "ضوچب",
    "ضوچتب",
    "ضوچتی",
    "ضوچتے",
    "ضوچو",
    "ضوچٌب",
    "ضوچی",
    "ضوچیں",
    "ضکب",
    "ضکتب",
    "ضکتی",
    "ضکتے",
    "ضکٌب",
    "ضکی",
    "ضکے",
    "ضیذھب",
    "ضیذھی",
    "ضیذھے",
    "ضیکٌڈ",
    "ضے",
    "طرف",
    "طریق",
    "طریقوں",
    "طریقہ",
    "طریقے",
    "طور",
    "طورپر",
    "ظبہر",
    "ع",
    "عذد",
    "عظین",
    "علاقوں",
    "علاقہ",
    "علاقے",
    "علاوٍ",
    "عووهی",
    "غبیذ",
    "غخص",
    "غذ",
    "غروع",
    "غروعبت",
    "غے",
    "فرد",
    "فی",
    "ق",
    "قجل",
    "قجیلہ",
    "قطن",
    "لئے",
    "لا",
    "لازهی",
    "لو",
    "لوجب",
    "لوجی",
    "لوجے",
    "لوسبت",
    "لوسہ",
    "لوگ",
    "لوگوں",
    "لڑکپي",
    "لگتب",
    "لگتی",
    "لگتے",
    "لگٌب",
    "لگی",
    "لگیں",
    "لگے",
    "لی",
    "لیب",
    "لیٌب",
    "لیں",
    "لے",
    "ه",
    "هتعلق",
    "هختلف",
    "هسترم",
    "هسترهہ",
    "هسطوش",
    "هسیذ",
    "هطئلہ",
    "هطئلے",
    "هطبئل",
    "هطتعول",
    "هطلق",
    "هعلوم",
    "هػتول",
    "هلا",
    "هوکي",
    "هوکٌبت",
    "هوکٌہ",
    "هٌبضت",
    "هڑا",
    "هڑًب",
    "هڑے",
    "هکول",
    "هگر",
    "هہرثبى",
    "هیرا",
    "هیری",
    "هیرے",
    "هیں",
    "و",
    "وار",
    "والے",
    "وٍ",
    "ًئی",
    "ًئے",
    "ًب",
    "ًبپطٌذ",
    "ًبگسیر",
    "ًطجت",
    "ًقطہ",
    "ًو",
    "ًوخواى",
    "ًکبلٌب",
    "ًکتہ",
    "ًہ",
    "ًہیں",
    "ًیب",
    "ًے",
    "ٓ آش",
    "ٹھیک",
    "پبئے",
    "پبش",
    "پبًب",
    "پبًچ",
    "پر",
    "پراًب",
    "پطٌذ",
    "پل",
    "پورا",
    "پوچھب",
    "پوچھتب",
    "پوچھتی",
    "پوچھتے",
    "پوچھو",
    "پوچھوں",
    "پوچھٌب",
    "پوچھیں",
    "پچھلا",
    "پھر",
    "پہلا",
    "پہلی",
    "پہلےضی",
    "پہلےضے",
    "پہلےضےہی",
    "پیع",
    "چبر",
    "چبہب",
    "چبہٌب",
    "چبہے",
    "چلا",
    "چلو",
    "چلیں",
    "چلے",
    "چکب",
    "چکی",
    "چکیں",
    "چکے",
    "چھوٹب",
    "چھوٹوں",
    "چھوٹی",
    "چھوٹے",
    "چھہ",
    "چیسیں",
    "ڈھوًڈا",
    "ڈھوًڈلیب",
    "ڈھوًڈو",
    "ڈھوًڈًب",
    "ڈھوًڈی",
    "ڈھوًڈیں",
    "ک",
    "کئی",
    "کئے",
    "کب",
    "کبفی",
    "کبم",
    "کت",
    "کجھی",
    "کرا",
    "کرتب",
    "کرتبہوں",
    "کرتی",
    "کرتے",
    "کرتےہو",
    "کررہب",
    "کررہی",
    "کررہے",
    "کرو",
    "کرًب",
    "کریں",
    "کرے",
    "کطی",
    "کل",
    "کن",
    "کوئی",
    "کوتر",
    "کورا",
    "کوروں",
    "کورٍ",
    "کورے",
    "کوطي",
    "کوى",
    "کوًطب",
    "کوًطی",
    "کوًطے",
    "کھولا",
    "کھولو",
    "کھولٌب",
    "کھولی",
    "کھولیں",
    "کھولے",
    "کہ",
    "کہب",
    "کہتب",
    "کہتی",
    "کہتے",
    "کہو",
    "کہوں",
    "کہٌب",
    "کہی",
    "کہیں",
    "کہے",
    "کی",
    "کیب",
    "کیطب",
    "کیطرف",
    "کیطے",
    "کیلئے",
    "کیوًکہ",
    "کیوں",
    "کیے",
    "کے",
    "کےثعذ",
    "کےرریعے",
    "گئی",
    "گئے",
    "گب",
    "گرد",
    "گروٍ",
    "گروپ",
    "گروہوں",
    "گٌتی",
    "گی",
    "گیب",
    "گے",
    "ہر",
    "ہن",
    "ہو",
    "ہوئی",
    "ہوئے",
    "ہوا",
    "ہوبرا",
    "ہوبری",
    "ہوبرے",
    "ہوتب",
    "ہوتی",
    "ہوتے",
    "ہورہب",
    "ہورہی",
    "ہورہے",
    "ہوضکتب",
    "ہوضکتی",
    "ہوضکتے",
    "ہوًب",
    "ہوًی",
    "ہوًے",
    "ہوچکب",
    "ہوچکی",
    "ہوچکے",
    "ہوگئی",
    "ہوگئے",
    "ہوگیب",
    "ہوں",
    "ہی",
    "ہیں",
    "ہے",
    "ی",
    "یقیٌی",
    "یہ",
    "یہبں",
];
