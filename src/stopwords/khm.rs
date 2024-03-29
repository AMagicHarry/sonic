// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub static STOPWORDS_KHM: &[&str] = &[
    "ៗ",
    "។ល។",
    "៚",
    "។",
    "៕",
    "៖",
    "៙",
    "០",
    "១",
    "២",
    "៣",
    "៤",
    "៥",
    "៦",
    "៧",
    "៨",
    "៩",
    "៛",
    "នេះ",
    "នោះ",
    "ខ្ញុំ",
    "អ្នក",
    "គាត់",
    "នាង",
    "ពួក",
    "យើង",
    "ពួកគេ",
    "លោក",
    "អ្វី",
    "បាន",
    "ការ",
    "នៅ",
    "និង",
    "ដែល",
    "មាន",
    "ជា",
    "ថា",
    "ក្នុង",
    "របស់",
    "ពី",
    "មួយ",
    "នឹង",
    "ឲ្យ",
    "មិន",
    "ទៅ",
    "តែ",
    "ត្រូវ",
    "ដោយ",
    "ហើយ",
    "ឆ្នាំ",
    "ពេល",
    "គេ",
    "ប្រទេស",
    "អាច",
    "គឺ",
    "ក្រុម",
    "ធ្វើ",
    "ក៏",
    "លើ",
    "នៃ",
    "ដើម្បី",
    "មក",
    "ទី",
    "តាម",
    "ទេ",
    "ដល់",
    "វា",
    "ដែរ",
    "ខ្លួន",
    "សម្រាប់",
    "ក្រុមហ៊ុន",
    "ថ្ងៃ",
    "ចំនួន",
    "កម្ពុជា",
    "ឡើង",
    "ទៀត",
    "ទាំង",
    "បើ",
    "និយាយ",
    "ទទួល",
    "ដ៏",
    "ច្រើន",
    "ផង",
    "ដឹង",
    "ជាមួយ",
    "គ្នា",
    "ខែ",
    "នាក់",
    "កំពុង",
    "យ៉ាង",
    "តម្លៃ",
    "ប្រកួត",
    "ក្រុង",
    "តំបន់",
    "ភាព",
    "យក",
    "ជាង",
    "ចូល",
    "នូវ",
    "កាលពី",
    "ណា",
    "បន្ត",
    "ជាតិ",
    "រូប",
    "មនុស្ស",
    "កាល",
    "ចំពោះ",
    "ដូច",
    "ខណៈ",
    "វិញ",
    "មុន",
    "ភ្នំពេញ",
    "លើក",
    "ល្អ",
    "ខាង",
    "ដុល្លារ",
    "ឃើញ",
    "បញ្ហា",
    "ប្រើ",
    "ចាប់",
    "ទឹក",
    "តើ",
    "ប្រាក់",
    "ធំ",
    "ខ្មែរ",
    "ចេញ",
    "ខេត្ត",
    "ផ្នែក",
    "ថ្មី",
    "បង្ហាញ",
    "ស៊ី",
    "អាមេរិក",
    "គឺជា",
    "លក់",
    "ចង់",
    "ដាក់",
    "ម្នាក់",
    "រួម",
    "រថយន្ត",
    "ផ្លូវ",
    "ភាគរយ",
    "កើន",
    "ជួយ",
    "ពីរ",
    "លាន",
    "ផ្តល់",
    "រដ្ឋ",
    "ខ្លាំង",
    "ជាច្រើន",
    "ទីក្រុង",
    "ជន",
    "កីឡា",
    "ក្រោយ",
    "ប្រាប់",
    "រដ្ឋាភិបាល",
    "កាន់",
    "ការងារ",
    "រក",
    "ព្រោះ",
    "រឿង",
    "ប៉ុន្តែ",
    "ឡើយ",
    "មុខ",
    "ថ្លែង",
    "ធ្វើឲ្យ",
    "បី",
    "នាំ",
    "ច្បាប់",
    "ដី",
    "ដូចជា",
    "កម",
    "ផ្ទះ",
    "បញ្ជាក់",
    "ចុះ",
    "បំផុត",
    "ចិត្ត",
    "បែប",
    "ចិន",
    "កីឡាករ",
    "កញ្ញា",
    "គម្រោង",
    "បង្កើត",
    "នា",
    "សារ",
    "សេដ្ឋកិច្ច",
    "ធនាគារ",
    "អស់",
    "ភាគ",
    "កូន",
    "ប្រធាន",
    "ផ្សារ",
    "ខ្ពស់",
    "គ្មាន",
    "ណាស់",
    "សម្រេច",
    "គួរ",
    "គ្រប់",
    "ប្រជាជន",
    "បន្ថែម",
    "រយៈ",
    "ខ្លះ",
    "បទ",
    "ទិញ",
    "ទើប",
    "វិនិយោគ",
    "មានការ",
    "លេខ",
    "ថៃ",
    "មើល",
    "បុរស",
    "យុវជន",
    "ស្រី",
    "នយោបាយ",
    "កន្លែង",
    "គិត",
    "បើក",
    "ដូច្នេះ",
    "រូបថត",
    "វាយ",
    "ប្រភេទ",
    "សំខាន់",
    "បន្ទាប់ពី",
    "កម្មវិធី",
    "រយៈពេល",
    "ផលិត",
    "ឈ្នះ",
    "ពិភពលោក",
    "ភ្ញៀវ",
    "ដោយសារ",
    "ស្រុក",
    "អាយុ",
    "ចំណាយ",
    "អំពី",
    "ហ៊ុន",
    "សិក្សា",
];
