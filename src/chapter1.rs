/// Reverse the input string
///
/// # 00. 文字列の逆順
/// 文字列”stressed”の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．
///
/// # Examples
///
/// ```
/// let s = "stressed";
/// let res = chapter1::ex00(&s);
///
/// assert_eq!("desserts", res);
/// ```
pub fn ex00(s: &str) -> String {
    s.chars().rev().collect()
}

/// Extract the characters of odd indices
///
/// # 01. 「パタトクカシーー」
/// 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
///
/// # Examples
///
/// ```
/// let s = "パタトクカシーー";
/// let res = chapter1::ex01(&s);
///
/// assert_eq!("パトカー", res);
/// ```
pub fn ex01(s: &str) -> String {
    s.char_indices()
        .filter(|(i, _)| (i + 1) % 2 == 1)
        .map(|(_, c)| c)
        .collect()
}

/// # 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」Permalink
/// 「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．
///
/// # Examples
/// ```
/// let patoka = "パトカー";
/// let takusi = "タクシー";
/// let res = chapter1::ex02(patoka, takusi);
///
/// assert_eq!("パタトクカシーー", res);
pub fn ex02(s1: &str, s2: &str) -> String {
    s1.chars()
        .zip(s2.chars())
        .flat_map(|(c1, c2)| vec![c1, c2])
        .collect()
}

/// # 03. 円周率Permalink
/// “Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.”という文を単語に分解し，各単語の（アルファベットの）文字数を先頭から出現順に並べたリストを作成せよ．
///
/// # Examples
/// ```
/// let s = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
/// let res = chapter1::ex03(s);
///
/// assert_eq!(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9], res);
pub fn ex03(s: &str) -> Vec<usize> {
    s.replace(",", "")
        .replace(".", "")
        .split(" ")
        .map(|w| w.len())
        .collect()
}
