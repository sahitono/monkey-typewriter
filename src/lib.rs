use crate::constants::{
    WeightedData, PREFIXES, SYLLABLES, SYLLABLE_LENGTH, VOICED, VOICELESS, VOWELS,
};
use rand::random;

pub mod constants;

pub fn weighted_random<T>(texts: &[WeightedData<T>]) -> &WeightedData<T> {
    let mut total: f32 = 0.0;
    for text in texts.iter() {
        total += text.weight as f32
    }

    let threshold: f32 = random::<f32>() * total;
    total = 0.0;
    for text in texts.iter() {
        total += text.weight as f32;
        if total >= threshold {
            return text;
        }
    }

    return texts.last().expect("not found");
}

pub fn sample<T>(texts: &[T]) -> &T {
    let index: f32 = random::<f32>() * texts.len() as f32;
    return texts.get(index.floor() as usize).unwrap();
}

pub fn get_syllable() -> String {
    let mut con: String = String::from(weighted_random(&VOICED).data);
    if random::<f32>() > 0.6 {
        con = String::from(weighted_random(&VOICELESS).data);
        if random::<f32>() > 0.6 {
            let vowel = sample(&VOWELS);
            con.push_str(vowel)
        }
    }

    let mut syl = format!("{}{}", con, sample(&VOWELS));
    if random::<f32>() > 0.9 {
        syl = weighted_random(&SYLLABLES).data.to_owned();
    }

    return syl;
}

pub fn create_slug(length: u32) -> String {
    return (0..length)
        .map(|_| generate_word())
        .collect::<Vec<String>>()
        .join("-");
}

pub fn generate_word() -> String {
    let mut words: Vec<String> = Vec::new();
    if random::<f32>() > 0.2 {
        let sampled = sample(&PREFIXES);
        words.push(String::from(*sampled));
    }

    let syl_len = weighted_random(&SYLLABLE_LENGTH);
    for _ in 0..syl_len.data {
        words.push(get_syllable());
    }
    if words.len() == 0 {
        words.push(get_syllable());
    }

    let last_char = String::from(words.last().expect("not found").chars().last().unwrap());
    let last_not_e = last_char != "e";
    let has_vowel = VOWELS.contains(&last_char.as_str());

    if has_vowel && random::<f32>() > 0.15 && last_not_e {
        if random::<f32>() > 0.8 {
            words.push(String::from(weighted_random(&VOICED).data));
        } else {
            words.push(String::from(weighted_random(&VOICELESS).data));
        }
    }

    if random::<f32>() > 0.3 {
        if random::<f32>() > 0.5 {
            words.push(String::from(weighted_random(&VOICED).data));
        }

        words.push(String::from(*sample(&PREFIXES)));
    }

    let fake_word = words.join("");
    if fake_word.len() < 3 {
        return generate_word();
    }

    return fake_word;
}
