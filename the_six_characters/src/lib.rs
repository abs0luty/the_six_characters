use std::collections::HashMap;

fn num(n: usize) -> String {
    if n == 0 {
        "+[]".to_owned()
    } else {
        vec!["+!![]"; n].join(" + ")
    }
}

fn str_static(h: &HashMap<char, String>, s: &str) -> String {
    s.chars()
        .map(|x| h.get(&x).unwrap().to_owned())
        .collect::<Vec<_>>()
        .join("+")
}

fn char(h: &HashMap<char, String>, c: char) -> String {
    format!(
        "([]+[])[{}][{}]({})",
        str_static(h, "constructor"),
        str_static(h, "fromCharCode"),
        num(c as u32 as usize)
    )
}

fn str(h: &HashMap<char, String>, s: &str) -> String {
    s.chars()
        .map(|x| h.get(&x).unwrap_or(&char(h, x)).to_owned())
        .collect::<Vec<_>>()
        .join("+")
}

pub fn transpile(code: &str) -> String {
    let mut mp = HashMap::new();
    mp.insert('a', format!("(+{{}}+[])[{}]", num(1)));
    mp.insert('b', format!("({{}}+[])[{}]", num(2)));
    mp.insert('o', format!("({{}}+[])[{}]", num(1)));
    mp.insert('e', format!("({{}}+[])[{}]", num(4)));
    mp.insert('c', format!("({{}}+[])[{}]", num(5)));
    mp.insert('t', format!("({{}}+[])[{}]", num(6)));
    mp.insert(' ', format!("({{}}+[])[{}]", num(7)));
    mp.insert('f', format!("(![]+[])[{}]", num(0)));
    mp.insert('s', format!("(![]+[])[{}]", num(3)));
    mp.insert('r', format!("(!![]+[])[{}]", num(1)));
    mp.insert('u', format!("(!![]+[])[{}]", num(2)));
    mp.insert('i', format!("((+!![]/+[])+[])[{}]", num(3)));
    mp.insert('n', format!("((+!![]/+[])+[])[{}]", num(4)));
    mp.insert(
        'S',
        format!("([]+([]+[])[{}])[{}]", str_static(&mp, "constructor"), num(9)),
    );
    mp.insert(
        'g',
        format!("([]+([]+[])[{}])[{}]", str_static(&mp, "constructor"), num(14)),
    );
    mp.insert(
        'p',
        format!("([]+(/-/)[{}])[{}]", str_static(&mp, "constructor"), num(14)),
    );
    mp.insert('\\', format!("(/\\\\/+[])[{}]", num(1)));
    mp.insert(
        'd',
        format!("({})[{}]({})", num(13), str_static(&mp, "toString"), num(14)),
    );
    mp.insert(
        'h',
        format!("({})[{}]({})", num(17), str_static(&mp, "toString"), num(18)),
    );
    mp.insert(
        'm',
        format!("({})[{}]({})", num(22), str_static(&mp, "toString"), num(23)),
    );
    mp.insert(
        'C',
        format!(
            "((()=>{{}})[{}]({})()({}))[{}]",
            str_static(&mp, "constructor"),
            str_static(&mp, "return escape"),
            mp.get(&'\\').unwrap(),
            num(2)
        ),
    );

    format!(
        "(()=>{{}})[{}]({})()",
        str_static(&mp, "constructor"),
        str(&mp, code)
    )
}
