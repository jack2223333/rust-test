///把字符串s的所有from字符串替换成to
pub fn str_replace(s: &str, from: &str, to: &str) -> String {
    if s.len() < from.len() {
        return s.to_owned();//如果要替换的字符串长度大于原字符串，返回原字符串
    }

    let (s, from, to) = (s.as_bytes(), from.as_bytes(), to.as_bytes()); //把字符串转成字符数组
    let mut ans: Vec<u8> = Vec::new();

    unsafe {
        if from.is_empty() {
            for &b in s {
                ans.extend_from_slice(to);
                ans.push(b);
            }
            ans.extend_from_slice(to);//如果from字符串是空字符串，返回的字符串会在s的每个字符中插入to
        } else {
            let mut i = 0;
            let end = s.len();
            while i < end {
                let probe = s.get_unchecked(i..i + from.len());
                if probe == from {
                    ans.extend_from_slice(to);//如果匹配到就替换
                    i += from.len();
                } else {
                    ans.push(s[i]);//没有匹配到就填充上原字符
                    i += 1;
                }
            }
        }

        String::from_utf8(ans).unwrap_unchecked()
    }
}