

pub struct Indicators {
    pub ok: &'static str,
    pub ahead: &'static str,
    pub behind: &'static str,
    pub files: &'static str,
    pub err: &'static str,
}

impl Indicators {
    pub fn new(mode: bool) -> Indicators {
        let indicators = match mode {
            true => {
                Indicators {
                    ok: "+",
                    ahead: "->",
                    behind: "<-",
                    files: "*",
                    err: "x",
                }
            }
            false => {
                Indicators {
                    ok: "✓",
                    ahead: "→",
                    behind: "←",
                    files: "◎",
                    err: "⨯",
                }
            }
        };
        indicators
    }
}
