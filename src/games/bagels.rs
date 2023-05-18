use rand::Rng;

pub enum BagelsState {
    Playing,
    Won,
    Lost,
}

pub struct BagelsGameState {
    secret: Vec<u32>,
    guess_count: usize,
    max_guesses: usize,
    state: BagelsState,
}

#[derive(Clone)]
pub struct BagelsResult {
    pub fermi: usize,
    pub pico: usize,
    pub bagels: usize,
}

impl std::fmt::Display for BagelsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result_string = String::new();

        for _ in 0..self.fermi {
            result_string.push_str("Fermi ");
        }

        for _ in 0..self.pico {
            result_string.push_str("Pico ");
        }

        for _ in 0..self.bagels {
            result_string.push_str("Bagels ");
        }

        write!(f, "{}", result_string)
    }
}

impl BagelsGameState {
    pub fn new(digits: usize) -> Self {
        const MIN_DIGITS: usize = 3;
        const MAX_DIGITS: usize = 5;

        let clamped_digits = digits.clamp(MIN_DIGITS, MAX_DIGITS);

        let mut secret = Vec::with_capacity(clamped_digits);
        let mut rng = rand::thread_rng();
        while secret.len() < clamped_digits {
            let digit = rng.gen_range(0..10);
            if !secret.contains(&digit) {
                secret.push(digit);
            }
        }

        Self {
            secret,
            guess_count: 0,
            max_guesses: 10,
            state: BagelsState::Playing,
        }
    }

    pub fn guess(self: &mut Self, guess: String) -> Result<BagelsResult, String> {
        match self.state {
            BagelsState::Won => {
                return Err("You already won!".to_string());
            }
            BagelsState::Lost => {
                return Err("You already lost!".to_string());
            }
            _ => {}
        }

        if guess.len() != self.secret.len() {
            return Err(format!(
                "Guess must be {} digits long, got {}",
                self.secret.len(),
                guess.len()
            ));
        }

        // Check for duplicates
        let mut guess_vec = Vec::with_capacity(guess.len());
        for digit in guess.chars() {
            let digit = digit.to_digit(10);
            match digit {
                Some(digit) => {
                    if guess_vec.contains(&digit) {
                        return Err("Guess must not contain duplicate digits!".to_string());
                    } else {
                        guess_vec.push(digit);
                    }
                }
                None => {
                    return Err("Guess must only contain digits!".to_string());
                }
            }
        }

        let mut fermi = 0;
        let mut pico = 0;
        let mut bagels = 0;

        for (i, digit) in guess_vec.iter().enumerate() {
            if self.secret.contains(digit) {
                if self.secret[i] == *digit {
                    fermi += 1;
                } else {
                    pico += 1;
                }
            }
        }

        if fermi == 0 && pico == 0 {
            bagels += 1;
        }

        let result = BagelsResult {
            fermi,
            pico,
            bagels,
        };

        self.guess_count += 1;

        if result.fermi == self.secret.len() {
            self.state = BagelsState::Won;
        } else if self.guess_count >= self.max_guesses {
            self.state = BagelsState::Lost;
        }

        Ok(result)
    }

    pub fn get_secret(self: &Self) -> u32 {
        let mut secret = 0;
        for digit in &self.secret {
            secret *= 10;
            secret += *digit as u32;
        }
        secret
    }

    pub fn get_state(self: &Self) -> &BagelsState {
        &self.state
    }
}
