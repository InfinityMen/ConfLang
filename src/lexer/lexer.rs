
use regex::Regex;

// ==========================================================
// 1. ТИПЫ ДАННЫХ (ТОКЕНЫ)
// ==========================================================

#[derive(Debug, Clone, PartialEq)]
pub enum MicroToken {
    Number(f64),
    Ident(String),
    Operator(String),
    StringLiteral(String),
    LeftBracket,  // [
    RightBracket, // ]
}

#[derive(Debug)]
pub struct MacroToken {
    pub id: u32,
    pub name: &'static str,
    pub line: usize,
    // Каждый элемент вектора - это содержимое одной пары внешних скобок [...]
    pub args: Vec<Vec<MicroToken>>, 
}

// ==========================================================
// 2. БИБЛИОТЕКА ШАБЛОНОВ (ЦИТАТЫ)
// ==========================================================

pub struct LexTemplate {
    pub id: u32,
    pub name: &'static str,
    pub pattern: &'static str,
}

pub const WISDOM_LIBRARY: &[LexTemplate] = &[
    LexTemplate { id: 11, name: "PRINT", pattern: r"He first presents his words as \[(?P<arg1>.*)\], and then according to them he acts\." },
    LexTemplate { id: 20, name: "ASSIGN", pattern: r"He who by reanimating \[(?P<arg1>.*)\] can gain knowledge of \[(?P<arg2>.*)\]\." },
    LexTemplate { id: 30, name: "IF", pattern: r"When the superior man is informed in \[(?P<arg1>.*)\], it is right\." },
    LexTemplate { id: 41, name: "FOR", pattern: r"The man who moves a mountain begins by carrying away every stone from \[(?P<arg1>.*)\]\." },
    LexTemplate { id: 99, name: "EXIT", pattern: r"If a man in the morning hear the right way, he may die in the evening without regret\." },
];

// ==========================================================
// 3. РЕАЛИЗАЦИЯ ЛЕКСЕРА
// ==========================================================

pub struct SageLexer {
    rules: Vec<(&'static LexTemplate, Regex)>,
}

impl SageLexer {
    pub fn new() -> Self {
        let rules = WISDOM_LIBRARY.iter()
            .map(|t| (t, Regex::new(t.pattern).expect("Invalid Regex")))
            .collect();
        Self { rules }
    }

    /// Извлекает содержимое самых внешних скобок [...], сохраняя вложенные внутри как текст.
    fn extract_top_level_brackets(&self, line: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut depth = 0;
        let mut current = String::new();
        
        for c in line.chars() {
            if c == '[' {
                if depth > 0 { current.push(c); }
                depth += 1;
            } else if c == ']' {
                depth -= 1;
                if depth == 0 {
                    results.push(current.clone());
                    current.clear();
                } else {
                    current.push(c);
                }
            } else if depth > 0 {
                current.push(c);
            }
        }
        results
    }

    /// Разбирает внутреннее содержимое скобок на атомарные MicroToken.
    fn tokenize_inner(&self, input: &str) -> Vec<MicroToken> {
        let mut processed = input.to_string();
        
        // Добавляем пробелы вокруг скобок и операторов для корректного split_whitespace
        let replacements = [
            ("harmonized with", "+"),
            ("diminished by", "-"),
            ("multiplied by", "*"),
            ("shared among", "/"),
            ("is in accord with", "=="),
            ("is superior to", ">"),
            ("[", " [ "),
            ("]", " ] "),
        ];

        for (from, to) in replacements {
            processed = processed.replace(from, &format!(" {} ", to));
        }

        let mut tokens = Vec::new();
        let mut chars = processed.chars().peekable();

        // Простой парсер для слов, чисел и строк
        let words = processed.split_whitespace();

        for word in words {
            if let Ok(n) = word.parse::<f64>() {
                tokens.push(MicroToken::Number(n));
            } else if word == "[" {
                tokens.push(MicroToken::LeftBracket);
            } else if word == "]" {
                tokens.push(MicroToken::RightBracket);
            } else if word.starts_with('"') && word.ends_with('"') {
                tokens.push(MicroToken::StringLiteral(word.trim_matches('"').to_string()));
            } else if ["+", "-", "*", "/", "==", ">"].contains(&word) {
                tokens.push(MicroToken::Operator(word.to_string()));
            } else {
                tokens.push(MicroToken::Ident(word.to_string()));
            }
        }
        tokens
    }

    /// Главная функция: превращает текст программы в вектор макро-токенов.
    pub fn tokenize(&self, input: &str) -> Vec<MacroToken> {
        let mut tokens = Vec::new();

        for (idx, raw_line) in input.lines().enumerate() {
            let line = raw_line.trim();
            // Пропускаем пустые строки, комментарии и одиночные блоки
            if line.is_empty() || line.starts_with("//") || line == "{" || line == "}" {
                continue;
            }

            let mut matched = false;
            for (template, re) in &self.rules {
                if re.is_match(line) {
                    let bracket_contents = self.extract_top_level_brackets(line);
                    let mut args = Vec::new();

                    for content in bracket_contents {
                        args.push(self.tokenize_inner(&content));
                    }

                    tokens.push(MacroToken {
                        id: template.id,
                        name: template.name,
                        line: idx + 1,
                        args,
                    });
                    matched = true;
                    break;
                }
            }

            if !matched {
                println!("Warning: Line {} does not follow the Sage's Way.", idx + 1);
            }
        }
        tokens
    }
}

// ==========================================================
// 4. ДЕМОНСТРАЦИЯ РАБОТЫ
// ==========================================================

pub fn mainy() {
    let lexer = SageLexer::new();

    // Пример с глубокой вложенностью: x = (10 + (5 * 2))
    let code = r#"
        He who by reanimating [10 harmonized with [5 multiplied by 2]] can gain knowledge of [x].
        When the superior man is informed in [x is superior to 15], it is right.
        He first presents his words as ["The Path is " harmonized with x], and then according to them he acts.
        If a man in the morning hear the right way, he may die in the evening without regret.
    "#;

    let result = lexer.tokenize(code);

    println!("--- РЕЗУЛЬТАТ РАБОТЫ ЛЕКСЕРА ---");
    for token in result {
        println!("\nСТРОКА {}: {} (ID: {})", token.line, token.name, token.id);
        for (i, arg) in token.args.iter().enumerate() {
            print!("  Аргумент {}: ", i + 1);
            for micro in arg {
                print!("{:?} ", micro);
            }
            println!();
        }
    }
}
