
pub struct PBRTTokenizer<'a> {
    text: &'a str
}

impl<'a> PBRTTokenizer<'a> {
    pub fn new(text: &'a str) -> Self {
        PBRTTokenizer {text}
    }
}

fn find_offsets(text: &str) -> (usize, usize) {

    let mut chars = text.chars().enumerate();
    let skip = [' ', '\n', '\t', '\r'];
    let mut skip_until_end_of_line = false;
    let mut start_offset = -1;
    let mut inside_p = false;

    loop {
        let (index, c) = match chars.next() {
            Some(c) => c,
            None => break
        };
        if skip_until_end_of_line && c != '\n'{
            continue;
        } else if skip_until_end_of_line && c == '\n' {
            skip_until_end_of_line = false;
            continue;
        }
        if c == '#' {
            skip_until_end_of_line = true;
            continue;
        }
        if c == '[' { return (index, index+1); }
        if c == ']' { return (index, index+1); }
        if skip.contains(&c) { continue; }

        start_offset = index as i32;
        if c == '"' {
            inside_p = true;
        }
        break;
    }

    if start_offset == -1 {
        return (0, 0);
    }
    let mut end_offset = start_offset;

    loop {
        let (index, c) = match chars.next() {
            Some(c) => c,
            None => break
        };
        if inside_p {
            if c == '"' {
                end_offset = index as i32;
                break;
            }

        } else {
            if c == ' ' || c == ']' || c == '\n' || c == '\t' {
                end_offset = index as i32;
                break;
            }
        }              
    }
    (start_offset as usize, end_offset as usize)
}

impl<'a> Iterator for PBRTTokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        
        let (start, end) = find_offsets(self.text);
        if end > start {
            // NOTE: we exclude quotes, if we have "float t" we return 'float t' without quotes
            if &self.text[start..start+1] == "\"" {
                let result = Some(&self.text[start+1..end]);
                self.text = &self.text[end+1..];
                result
            } else {
                let result = Some(&self.text[start..end]);
                self.text = &self.text[end..];
                result
            }
        }
        else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[test]
    fn pbrt_tokenizer() {
        let text = " [ 0 0   1 0 \"pero fov\" 1 1   0 1 2.2\t3.3]        \"5\"";
        let filename = "D:\\cpp_projects\\pbrt_v4_scenes\\cornell-box\\scene-v4_part_3.pbrt";
        let contents = fs::read_to_string(filename).unwrap();
        let toks = PBRTTokenizer::new(contents.as_str());
        for tok in toks {
            println!("{}", tok);
        }
    }

}
