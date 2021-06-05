pub struct SourceContext {
    source: String,
    idx: usize,
    line: i32
}

impl SourceContext {
    fn next() -> char {
        let foo: char = 'a';
        foo
    }
}

pub fn new_source(program: String) -> SourceContext {
    SourceContext {
        source: program,
        idx: 0,
        line: 0
    }
}

pub fn tokenize(source: SourceContext) {
    // println!("{}", source.source);
    scan_token(source)
}

fn scan_token(source: SourceContext) {
    let mut ctx = source;
    let mut curr = ctx.source.char_indices();
    while let Some((idx, c)) = curr.next() {
        match c {
            _ => {
                ctx.idx = idx;
                println!("DEFAULT {} at {}", c, idx)
            }
        }
    }
}
