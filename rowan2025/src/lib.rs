use std::collections::HashSet;

pub use challenges_input::{DefaultArgs, DefaultProblemDesc as ProblemDesc};

fn is_compatible(guess: &str, score: [u8; 5], word: &str) -> bool {
    for (i, letter) in word.chars().enumerate() {
        if (score[i] == 2 && letter != guess.chars().nth(i).unwrap())
            || ((score[i] == 1 || score[i] == 0) && letter == guess.chars().nth(i).unwrap())
        {
            return false;
        }
    }

    let letters_set: HashSet<char> = guess.chars().collect();
    for letter in letters_set {
        let num_found_in_guess = guess
            .chars()
            .zip(score)
            .filter(|&(c, s)| c == letter && s > 0)
            .count();
        let num_gray = guess
            .chars()
            .zip(score)
            .filter(|&(c, s)| c == letter && s == 0)
            .count();
        let max_in_answer = if num_gray == 0 { 5 } else { num_found_in_guess };
        let num_in_word = word.chars().filter(|&c| c == letter).count();
        if num_in_word > max_in_answer || num_found_in_guess > num_in_word {
            return false;
        }
    }

    true
}

fn score(answer: &str, guess: &str) -> [u8; 5] {
    let mut score = [0; 5];

    for (i, letter) in guess.chars().enumerate() {
        if letter == answer.chars().nth(i).unwrap() {
            score[i] = 2;
            continue;
        }
        let num_of_letter = answer.chars().filter(|&c| c == letter).count();
        let num_in_guess = guess
            .chars()
            .enumerate()
            .filter(|&(j, c)| c == letter && (c == answer.chars().nth(j).unwrap() || j < i))
            .count();
        score[i] = u8::from(num_in_guess < num_of_letter);
    }

    score
}

pub fn run(args: &DefaultArgs) -> String {
    let mut input = args.get_input(true);
    let mut output = String::new();

    let num_cases: usize = input.next().unwrap();

    macro_rules! puts {
        ($($arg:tt)*) => {
            output.push_str(&format!($($arg)*));
            output.push('\n');
        };
    }

    puts!("Analyzing {} data sets\n", num_cases);

    for i in 0..num_cases {
        puts!("Data Set {}", i + 1);
        let answer = input.next_line().unwrap();
        puts!("Puzzle answer: {answer}");

        let word_bank_size: usize = input.next().unwrap();
        puts!("Words in word bank: {word_bank_size}");
        let word_bank: Vec<String> = (0..word_bank_size)
            .map(|_| input.next_line().unwrap().to_string())
            .collect();

        let mut remaining_words = word_bank.clone();

        let guess_num: usize = input.next().unwrap();
        puts!("Guesses: {guess_num}");

        let mut solved = false;

        for guess in input.take(guess_num) {
            let mut res_str = String::from(" ");
            res_str += guess;
            res_str += ": ";

            if !word_bank.contains(&String::from(guess)) {
                res_str += "not in word list";
                puts!("{res_str}");
                continue;
            }

            let score = score(&answer, guess);
            res_str += &score.iter().map(ToString::to_string).collect::<String>();

            if guess == answer {
                solved = true;
                puts!("{res_str}");
                break;
            }

            res_str += "  words left: ";

            remaining_words.retain(|i| is_compatible(guess, score, i));
            res_str += &remaining_words.len().to_string();
            if remaining_words.len() <= 6 {
                remaining_words.sort();
                res_str += "  ";
                res_str += remaining_words.join(" ").as_str();
            }
            puts!("{res_str}");
        }
        if solved {
            puts!("The Puzzle Is Solved!\n");
        } else {
            puts!("Solution Not Found\n");
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowan_2025() {
        let input = DefaultArgs {
            input_file: String::from(
                "1
paste
5
alpha
paste
astro
snort
blast
4
blast
snort
alpha
paste",
            ),
        };
        let correct_output = "Analyzing 1 data set(s)
Data Set 1
Puzzle answer: paste
Words in word bank: 5
Guesses: 4
 blast: 00111  words left: 2  astro paste
 snort: 10001  words left: 1  paste
 alpha: 10100  words left: 1  paste
 paste: 22222
The Puzzle Is Solved!";
    }
}
