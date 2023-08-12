// pub const PREFIXES: Vec<&str> = vec![
//     "wa", "de", "re", "ex", "in", "po", "pro", "con", "auto", "ex", "extra", "hyper", "anti", "co",
//     "in", "mono", "non", "intra", "un", "post", "tele", "trans", "up",
// ];

pub const PREFIXES: [&str; 23] = [
    "wa", "de", "re", "ex", "in", "po", "pro", "con", "auto", "ex", "extra", "hyper", "anti", "co",
    "in", "mono", "non", "intra", "un", "post", "tele", "trans", "up",
];

pub struct WeightedData<T> {
    pub data: T,
    pub weight: u32,
}
pub type WeightedDataList<T> = Vec<WeightedData<T>>;

pub const SUFFIXES: [&str; 28] = [
    "ion", "ity", "ic", "ical", "ian", "ial", "ious", "ing", "ed", "s", "es", "acy", "ate", "en",
    "al", "fy", "ify", "esque", "able", "ible", "ness", "ship", "sion", "ment", "ist", "ism",
    "ful", "y",
];

pub const VOICELESS: [WeightedData<&str>; 7] = [
    WeightedData {
        data: "ch",
        weight: 10,
    },
    WeightedData {
        data: "f",
        weight: 10,
    },
    WeightedData {
        data: "k",
        weight: 10,
    },
    WeightedData {
        data: "p",
        weight: 10,
    },
    WeightedData {
        data: "s",
        weight: 10,
    },
    WeightedData {
        data: "sh",
        weight: 10,
    },
    WeightedData {
        data: "t",
        weight: 10,
    },
];

pub const VOICED: [WeightedData<&str>; 14] = [
    WeightedData {
        data: "b",
        weight: 10,
    },
    WeightedData {
        data: "d",
        weight: 10,
    },
    WeightedData {
        data: "g",
        weight: 10,
    },
    WeightedData {
        data: "j",
        weight: 2,
    },
    WeightedData {
        data: "l",
        weight: 10,
    },
    WeightedData {
        data: "m",
        weight: 10,
    },
    WeightedData {
        data: "n",
        weight: 10,
    },
    WeightedData {
        data: "ng",
        weight: 2,
    },
    WeightedData {
        data: "r",
        weight: 10,
    },
    WeightedData {
        data: "th",
        weight: 10,
    },
    WeightedData {
        data: "v",
        weight: 10,
    },
    WeightedData {
        data: "w",
        weight: 3,
    },
    WeightedData {
        data: "y",
        weight: 4,
    },
    WeightedData {
        data: "z",
        weight: 2,
    },
];

pub const SYLLABLES: [WeightedData<&str>; 5] = [
    WeightedData {
        data: "pho",
        weight: 10,
    },
    WeightedData {
        data: "lo",
        weight: 10,
    },
    WeightedData {
        data: "di",
        weight: 10,
    },
    WeightedData {
        data: "ta",
        weight: 10,
    },
    WeightedData {
        data: "bo",
        weight: 10,
    },
];

pub const VOWELS: [&str; 5] = ["a", "i", "u", "e", "o"];

pub const SYLLABLE_LENGTH: [WeightedData<u8>; 4] = [
    WeightedData {
        data: 1,
        weight: 25,
    },
    WeightedData {
        data: 2,
        weight: 10,
    },
    WeightedData { data: 3, weight: 5 },
    WeightedData { data: 4, weight: 2 },
];
// pub const SHORT_SYLLABLE_LENGTH: WeightedDataList<u8> = vec![
//     WeightedData {
//         data: 1,
//         weight: 10,
//     },
//     WeightedData {
//         data: 2,
//         weight: 20,
//     },
// ];
