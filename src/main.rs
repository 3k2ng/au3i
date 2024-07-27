#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Blue,
    Green,
    Pink,
    Orange,
    Yellow,
    Black,
    White,
    Purple,
    Brown,
    Cyan,
    Lime,
    Impostor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Color(Color),
    Sus,
    Innocent,
    Vented,
    Sussy,
    Vouch,
    Vote,
    Did,
    Qm,
    Then,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    Do(Color),
    PopThen(Color),
    DupThen(Color),
    RotLeftThen(Color),
    RotRightThen(Color),
    SwapThen(Color),
    While(Box<Action>),
    Then(Box<Action>, Box<Action>),
}

struct State {
    stack: std::collections::VecDeque<u8>,
    rng: rand::rngs::ThreadRng,
}

impl State {
    fn new() -> Self {
        Self {
            stack: std::collections::VecDeque::new(),
            rng: rand::thread_rng(),
        }
    }

    fn apply_color(&mut self, color: Color) {
        match color {
            Color::Red => (),
            Color::Blue => {
                if let Some(top) = self.stack.pop_front() {
                    self.stack.push_front(top.wrapping_add(1));
                }
            }
            Color::Green => *self.stack.front_mut().unwrap() -= 1,
            Color::Pink => {
                if let Some(top) = self.stack.pop_front() {
                    let second = *self.stack.front().unwrap_or(&0);
                    self.stack.push_front(top.wrapping_add(second));
                }
            }
            Color::Orange => {
                if let Some(top) = self.stack.pop_front() {
                    let second = *self.stack.front().unwrap_or(&0);
                    self.stack.push_front(top.wrapping_shl(second as u32));
                }
            }
            Color::Yellow => {
                if let Some(top) = self.stack.pop_front() {
                    let second = *self.stack.front().unwrap_or(&0);
                    self.stack.push_front(top.wrapping_shr(second as u32));
                }
            }
            Color::Black => {
                if let Some(top) = self.stack.pop_front() {
                    self.stack.push_front(!top);
                }
            }
            Color::White => {
                if let Some(top) = self.stack.pop_front() {
                    let second = *self.stack.front().unwrap_or(&0);
                    self.stack.push_front(top & second);
                }
            }
            Color::Purple => {
                if let Some(top) = self.stack.pop_front() {
                    let second = *self.stack.front().unwrap_or(&0);
                    self.stack.push_front(top ^ second);
                }
            }
            Color::Brown => {
                if self.stack.pop_front().is_some() {
                    self.stack
                        .push_front((rand::RngCore::next_u32(&mut self.rng) % 256) as u8);
                }
            }
            Color::Cyan => {
                if let Some(&top) = self.stack.front() {
                    print!("{}", top as char);
                }
            }
            Color::Lime => {
                let mut buf = String::new();
                std::io::stdin().read_line(&mut buf).unwrap_or_default();
                for b in buf.trim_end().bytes() {
                    self.stack.push_front(b);
                }
            }
            Color::Impostor => {
                print!("\n[");
                for i in self.stack.iter() {
                    print!(" {},", i);
                }
                println!("]");
                println!("  ^")
            }
        }
    }

    fn pop_then(&mut self, color: Color) {
        self.stack.pop_front();
        self.apply_color(color);
    }

    fn dup_then(&mut self, color: Color) {
        let top = *self.stack.front().unwrap_or(&0);
        self.stack.push_front(top);
        self.apply_color(color);
    }

    fn rot_left_then(&mut self, color: Color) {
        if let Some(top) = self.stack.pop_front() {
            self.stack.push_back(top);
        }
        self.apply_color(color);
    }

    fn rot_right_then(&mut self, color: Color) {
        if let Some(back) = self.stack.pop_back() {
            self.stack.push_front(back);
        }
        self.apply_color(color);
    }

    fn swap_then(&mut self, color: Color) {
        if self.stack.is_empty() {
        } else {
            let top = self.stack.pop_front().unwrap();
            let second = self.stack.pop_front().unwrap_or(0);
            self.stack.push_front(top);
            self.stack.push_front(second);
        }
        self.apply_color(color);
    }

    fn apply_action(&mut self, action: Action) {
        match action {
            Action::Do(color) => self.apply_color(color),
            Action::PopThen(color) => self.pop_then(color),
            Action::DupThen(color) => self.dup_then(color),
            Action::RotLeftThen(color) => self.rot_left_then(color),
            Action::RotRightThen(color) => self.rot_right_then(color),
            Action::SwapThen(color) => self.swap_then(color),
            Action::While(action) => {
                while *self.stack.front().unwrap_or(&0) != 0 {
                    self.apply_action(*action.clone());
                }
            }
            Action::Then(fa, sa) => {
                self.apply_action(*fa);
                self.apply_action(*sa);
            }
        }
    }
}

fn tokenize(source: &str) -> Option<Box<[Token]>> {
    let colors = std::collections::HashMap::from([
        ("red", Color::Red),
        ("blue", Color::Blue),
        ("green", Color::Green),
        ("pink", Color::Pink),
        ("orange", Color::Orange),
        ("yellow", Color::Yellow),
        ("black", Color::Black),
        ("white", Color::White),
        ("purple", Color::Purple),
        ("brown", Color::Brown),
        ("cyan", Color::Cyan),
        ("lime", Color::Lime),
        ("impostor", Color::Impostor),
    ]);
    let actions = std::collections::HashMap::from([
        ("sus", Token::Sus),
        ("innocent", Token::Innocent),
        ("vented", Token::Vented),
        ("sussy", Token::Sussy),
        ("vouch", Token::Vouch),
        ("vote", Token::Vote),
        ("did", Token::Did),
        ("then", Token::Then),
    ]);
    let mut tokens = vec![];
    let mut i = 0;
    while let Some(ch) = source.chars().nth(i) {
        if ch.is_ascii_alphabetic() {
            let mut j = i + 1;
            let mut buf = String::new();
            buf.push(ch);
            while let Some(nc) = source.chars().nth(j) {
                if nc.is_ascii_alphabetic() {
                    j += 1;
                    buf.push(nc);
                } else {
                    break;
                }
            }
            i = j;
            if let Some(col) = colors.get(buf.clone().into_boxed_str().as_ref()) {
                tokens.push(Token::Color(*col));
            } else if let Some(act) = actions.get(buf.clone().into_boxed_str().as_ref()) {
                tokens.push(*act);
            } else {
                return None;
            }
        } else if ch == '?' {
            i += 1;
            tokens.push(Token::Qm);
        } else if ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n' {
            i += 1;
        } else {
            return None;
        }
    }
    Some(tokens.into_boxed_slice())
}

// then -> question + ("then" + then)*
fn parse_then(tokens: &[Token], i: usize) -> Option<(Action, usize)> {
    let (fa, j) = parse_question(tokens, i)?;
    if let Some(Token::Then) = tokens.get(j) {
        let (sa, k) = parse_then(tokens, j + 1)?;
        Some((Action::Then(Box::new(fa), Box::new(sa)), k))
    } else {
        Some((fa, j))
    }
}

// question -> "Did" + then + "?" | action
fn parse_question(tokens: &[Token], i: usize) -> Option<(Action, usize)> {
    let ft = *tokens.get(i)?;
    if ft == Token::Did {
        let (ia, j) = parse_then(tokens, i + 1)?;
        if let Some(Token::Qm) = tokens.get(j) {
            Some((Action::While(Box::new(ia)), j + 1))
        } else {
            None
        }
    } else {
        parse_action(tokens, i)
    }
}

// action
fn parse_action(tokens: &[Token], i: usize) -> Option<(Action, usize)> {
    let ft = *tokens.get(i)?;
    if ft == Token::Sus || ft == Token::Sussy {
        Some((Action::Do(Color::Red), i + 1))
    } else {
        let st = *tokens.get(i + 1)?;
        Some((
            match ft {
                Token::Color(color) => match st {
                    Token::Sus => Some(Action::Do(color)),
                    Token::Innocent => Some(Action::PopThen(color)),
                    Token::Vented => Some(Action::DupThen(color)),
                    Token::Sussy => Some(Action::RotLeftThen(color)),
                    _ => None,
                },
                Token::Vouch => {
                    if let Token::Color(color) = st {
                        Some(Action::RotRightThen(color))
                    } else {
                        None
                    }
                }
                Token::Vote => {
                    if let Token::Color(color) = st {
                        Some(Action::SwapThen(color))
                    } else {
                        None
                    }
                }
                _ => None,
            }?,
            i + 2,
        ))
    }
}

fn parse(tokens: &[Token]) -> Option<Box<[Action]>> {
    let mut actions = vec![];
    let mut i = 0;
    while let Some((ca, j)) = parse_then(tokens, i) {
        i = j;
        actions.push(ca);
    }
    if i < tokens.len() {
        None
    } else {
        Some(actions.into_boxed_slice())
    }
}

fn execute(state: &mut State, actions: &[Action]) -> Option<()> {
    for action in actions {
        state.apply_action(action.clone());
    }
    Some(())
}

fn run(state: &mut State, source: &str) -> Option<()> {
    let tokens = tokenize(source)?;
    let actions = parse(&tokens)?;
    execute(state, &actions)
}

fn error() {
    println!("you sussy baka balls!");
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        match std::fs::read_to_string(args[1].clone()) {
            Ok(source) => {
                let mut state = State::new();
                if run(&mut state, &source.to_lowercase()).is_none() {
                    error()
                } else {
                    println!();
                }
            }
            Err(_) => error(),
        }
    } else {
        let mut state = State::new();
        let mut buf = String::new();
        while {
            buf.clear();
            print!(" > ");
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            std::io::stdin().read_line(&mut buf).is_ok()
        } && !buf.trim_end().is_empty()
        {
            if run(&mut state, &buf.to_lowercase()).is_none() {
                error()
            } else {
                println!();
            }
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }
}
