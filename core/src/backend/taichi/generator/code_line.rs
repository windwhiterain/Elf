pub struct CodeLine {
    pub space: usize,
    pub code: String,
}
pub struct CodeLines {
    pub lines: Vec<CodeLine>,
}
impl CodeLines {
    pub fn new() -> Self {
        Self { lines: Vec::new() }
    }
    pub fn write(&mut self, space: usize, code: String) {
        self.lines.push(CodeLine { space, code })
    }
    pub fn to_string(self) -> String {
        let mut ret = String::new();
        for line in self.lines {
            let space = "    ".repeat(line.space);
            let code = line.code;
            ret += format!("{space}{code}\n").as_str();
        }
        ret
    }
    pub fn append(&mut self, mut other: Self) {
        self.lines.append(other.lines.as_mut());
    }
}
