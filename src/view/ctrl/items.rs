pub struct Items<'a>(&'a str);

impl<'a> Items<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s)
    }
}

impl<'a> Iterator for Items<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut c;
        loop {
            c = self.0.chars().next()?;
            if !c.is_whitespace() {
                break;
            }
        }

        if c != '<' {
            let (res, state) = self.0.split_at(c.len_utf8());
            self.0 = state;
            return Some(res);
        }

        if let Some((res, state)) = self.0.split_once('>') {
            self.0 = state;
            Some(&res[1..])
        } else {
            let res = &self.0[1..];
            self.0 = "";
            Some(res)
        }
    }
}
