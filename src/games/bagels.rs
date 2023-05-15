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
        let clamped_digits = digits.clamp(3, 10);

        let mut secret = Vec::with_capacity(digits);
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

    pub fn guess(self: &mut Self, guess: u32) -> Result<BagelsResult, String> {
        match self.state {
            BagelsState::Won => {
                return Err("You already won!".to_string());
            }
            BagelsState::Lost => {
                return Err("You already lost!".to_string());
            }
            _ => {}
        }

        let guess_vec = guess
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();

        if guess_vec.len() != self.secret.len() {
            return Err(format!(
                "Guess must be {} digits long, got {}",
                self.secret.len(),
                guess_vec.len()
            ));
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
