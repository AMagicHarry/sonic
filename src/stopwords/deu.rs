// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub static STOPWORDS_DEU: &[&str] = &[
    "a",
    "ab",
    "aber",
    "ach",
    "acht",
    "achte",
    "achten",
    "achter",
    "achtes",
    "ag",
    "alle",
    "allein",
    "allem",
    "allen",
    "aller",
    "allerdings",
    "alles",
    "allgemeinen",
    "als",
    "also",
    "am",
    "an",
    "ander",
    "andere",
    "anderem",
    "anderen",
    "anderer",
    "anderes",
    "anderm",
    "andern",
    "anderr",
    "anders",
    "au",
    "auch",
    "auf",
    "aus",
    "ausser",
    "ausserdem",
    "außer",
    "außerdem",
    "b",
    "bald",
    "bei",
    "beide",
    "beiden",
    "beim",
    "beispiel",
    "bekannt",
    "bereits",
    "besonders",
    "besser",
    "besten",
    "bin",
    "bis",
    "bisher",
    "bist",
    "c",
    "d",
    "d.h",
    "da",
    "dabei",
    "dadurch",
    "dafür",
    "dagegen",
    "daher",
    "dahin",
    "dahinter",
    "damals",
    "damit",
    "danach",
    "daneben",
    "dank",
    "dann",
    "daran",
    "darauf",
    "daraus",
    "darf",
    "darfst",
    "darin",
    "darum",
    "darunter",
    "darüber",
    "das",
    "dasein",
    "daselbst",
    "dass",
    "dasselbe",
    "davon",
    "davor",
    "dazu",
    "dazwischen",
    "daß",
    "dein",
    "deine",
    "deinem",
    "deinen",
    "deiner",
    "deines",
    "dem",
    "dementsprechend",
    "demgegenüber",
    "demgemäss",
    "demgemäß",
    "demselben",
    "demzufolge",
    "den",
    "denen",
    "denn",
    "denselben",
    "der",
    "deren",
    "derer",
    "derjenige",
    "derjenigen",
    "dermassen",
    "dermaßen",
    "derselbe",
    "derselben",
    "des",
    "deshalb",
    "desselben",
    "dessen",
    "deswegen",
    "dich",
    "die",
    "diejenige",
    "diejenigen",
    "dies",
    "diese",
    "dieselbe",
    "dieselben",
    "diesem",
    "diesen",
    "dieser",
    "dieses",
    "dir",
    "doch",
    "dort",
    "drei",
    "drin",
    "dritte",
    "dritten",
    "dritter",
    "drittes",
    "du",
    "durch",
    "durchaus",
    "durfte",
    "durften",
    "dürfen",
    "dürft",
    "e",
    "eben",
    "ebenso",
    "ehrlich",
    "ei",
    "ei,",
    "eigen",
    "eigene",
    "eigenen",
    "eigener",
    "eigenes",
    "ein",
    "einander",
    "eine",
    "einem",
    "einen",
    "einer",
    "eines",
    "einig",
    "einige",
    "einigem",
    "einigen",
    "einiger",
    "einiges",
    "einmal",
    "eins",
    "elf",
    "en",
    "ende",
    "endlich",
    "entweder",
    "er",
    "ernst",
    "erst",
    "erste",
    "ersten",
    "erster",
    "erstes",
    "es",
    "etwa",
    "etwas",
    "euch",
    "euer",
    "eure",
    "eurem",
    "euren",
    "eurer",
    "eures",
    "f",
    "folgende",
    "früher",
    "fünf",
    "fünfte",
    "fünften",
    "fünfter",
    "fünftes",
    "für",
    "g",
    "gab",
    "ganz",
    "ganze",
    "ganzen",
    "ganzer",
    "ganzes",
    "gar",
    "gedurft",
    "gegen",
    "gegenüber",
    "gehabt",
    "gehen",
    "geht",
    "gekannt",
    "gekonnt",
    "gemacht",
    "gemocht",
    "gemusst",
    "genug",
    "gerade",
    "gern",
    "gesagt",
    "geschweige",
    "gewesen",
    "gewollt",
    "geworden",
    "gibt",
    "ging",
    "gleich",
    "gott",
    "gross",
    "grosse",
    "grossen",
    "grosser",
    "grosses",
    "groß",
    "große",
    "großen",
    "großer",
    "großes",
    "gut",
    "gute",
    "guter",
    "gutes",
    "h",
    "hab",
    "habe",
    "haben",
    "habt",
    "hast",
    "hat",
    "hatte",
    "hatten",
    "hattest",
    "hattet",
    "heisst",
    "her",
    "heute",
    "hier",
    "hin",
    "hinter",
    "hoch",
    "hätte",
    "hätten",
    "i",
    "ich",
    "ihm",
    "ihn",
    "ihnen",
    "ihr",
    "ihre",
    "ihrem",
    "ihren",
    "ihrer",
    "ihres",
    "im",
    "immer",
    "in",
    "indem",
    "infolgedessen",
    "ins",
    "irgend",
    "ist",
    "j",
    "ja",
    "jahr",
    "jahre",
    "jahren",
    "je",
    "jede",
    "jedem",
    "jeden",
    "jeder",
    "jedermann",
    "jedermanns",
    "jedes",
    "jedoch",
    "jemand",
    "jemandem",
    "jemanden",
    "jene",
    "jenem",
    "jenen",
    "jener",
    "jenes",
    "jetzt",
    "k",
    "kam",
    "kann",
    "kannst",
    "kaum",
    "kein",
    "keine",
    "keinem",
    "keinen",
    "keiner",
    "keines",
    "kleine",
    "kleinen",
    "kleiner",
    "kleines",
    "kommen",
    "kommt",
    "konnte",
    "konnten",
    "kurz",
    "können",
    "könnt",
    "könnte",
    "l",
    "lang",
    "lange",
    "leicht",
    "leide",
    "lieber",
    "los",
    "m",
    "machen",
    "macht",
    "machte",
    "mag",
    "magst",
    "mahn",
    "mal",
    "man",
    "manche",
    "manchem",
    "manchen",
    "mancher",
    "manches",
    "mann",
    "mehr",
    "mein",
    "meine",
    "meinem",
    "meinen",
    "meiner",
    "meines",
    "mensch",
    "menschen",
    "mich",
    "mir",
    "mit",
    "mittel",
    "mochte",
    "mochten",
    "morgen",
    "muss",
    "musst",
    "musste",
    "mussten",
    "muß",
    "mußt",
    "möchte",
    "mögen",
    "möglich",
    "mögt",
    "müssen",
    "müsst",
    "müßt",
    "n",
    "na",
    "nach",
    "nachdem",
    "nahm",
    "natürlich",
    "neben",
    "nein",
    "neue",
    "neuen",
    "neun",
    "neunte",
    "neunten",
    "neunter",
    "neuntes",
    "nicht",
    "nichts",
    "nie",
    "niemand",
    "niemandem",
    "niemanden",
    "noch",
    "nun",
    "nur",
    "o",
    "ob",
    "oben",
    "oder",
    "offen",
    "oft",
    "ohne",
    "ordnung",
    "p",
    "q",
    "r",
    "recht",
    "rechte",
    "rechten",
    "rechter",
    "rechtes",
    "richtig",
    "rund",
    "s",
    "sa",
    "sache",
    "sagt",
    "sagte",
    "sah",
    "satt",
    "schlecht",
    "schluss",
    "schon",
    "sechs",
    "sechste",
    "sechsten",
    "sechster",
    "sechstes",
    "sehr",
    "sei",
    "seid",
    "seien",
    "sein",
    "seine",
    "seinem",
    "seinen",
    "seiner",
    "seines",
    "seit",
    "seitdem",
    "selbst",
    "sich",
    "sie",
    "sieben",
    "siebente",
    "siebenten",
    "siebenter",
    "siebentes",
    "sind",
    "so",
    "solang",
    "solche",
    "solchem",
    "solchen",
    "solcher",
    "solches",
    "soll",
    "sollen",
    "sollst",
    "sollt",
    "sollte",
    "sollten",
    "sondern",
    "sonst",
    "soweit",
    "sowie",
    "später",
    "startseite",
    "statt",
    "steht",
    "suche",
    "t",
    "tag",
    "tage",
    "tagen",
    "tat",
    "teil",
    "tel",
    "tritt",
    "trotzdem",
    "tun",
    "u",
    "uhr",
    "um",
    "und",
    "und?",
    "uns",
    "unse",
    "unsem",
    "unsen",
    "unser",
    "unsere",
    "unserer",
    "unses",
    "unter",
    "v",
    "vergangenen",
    "viel",
    "viele",
    "vielem",
    "vielen",
    "vielleicht",
    "vier",
    "vierte",
    "vierten",
    "vierter",
    "viertes",
    "vom",
    "von",
    "vor",
    "w",
    "wahr?",
    "wann",
    "war",
    "waren",
    "warst",
    "wart",
    "warum",
    "was",
    "weg",
    "wegen",
    "weil",
    "weit",
    "weiter",
    "weitere",
    "weiteren",
    "weiteres",
    "welche",
    "welchem",
    "welchen",
    "welcher",
    "welches",
    "wem",
    "wen",
    "wenig",
    "wenige",
    "weniger",
    "weniges",
    "wenigstens",
    "wenn",
    "wer",
    "werde",
    "werden",
    "werdet",
    "weshalb",
    "wessen",
    "wie",
    "wieder",
    "wieso",
    "will",
    "willst",
    "wir",
    "wird",
    "wirklich",
    "wirst",
    "wissen",
    "wo",
    "woher",
    "wohin",
    "wohl",
    "wollen",
    "wollt",
    "wollte",
    "wollten",
    "worden",
    "wurde",
    "wurden",
    "während",
    "währenddem",
    "währenddessen",
    "wäre",
    "würde",
    "würden",
    "x",
    "y",
    "z",
    "z.b",
    "zehn",
    "zehnte",
    "zehnten",
    "zehnter",
    "zehntes",
    "zeit",
    "zu",
    "zuerst",
    "zugleich",
    "zum",
    "zunächst",
    "zur",
    "zurück",
    "zusammen",
    "zwanzig",
    "zwar",
    "zwei",
    "zweite",
    "zweiten",
    "zweiter",
    "zweites",
    "zwischen",
    "zwölf",
    "über",
    "überhaupt",
    "übrigens",
];
