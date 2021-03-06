use std::fmt;

/// イテレートできるもの。 Clone derive を外せない。  
#[derive(Clone)]
pub struct Sequence1 {
    /// String型は Unicodeなのでイテレートが案外難しい。 `Vec<char>` にしたのが工夫。  
    pub sequence: Vec<char>,
    /// イテレートで使う配列のインデックスのようなもの。  
    /// しかし `&mut self` でないと イテレートできない(`iter_mut()相当`)よな。  
    pub curr: usize,
}

/// 普段よく見る `for item in &items {` イテレーションの下地となる、  
/// 普段あまり表に出てこない `items.next()` を使ったイテレーションの実装です。  
impl Iterator for Sequence1 {
    // Self::Item ってこれ。
    type Item = Box<char>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.sequence.len() {
            println!(
                "[Sequence1.next] curr={: >2} {}",
                self.curr, self.sequence[self.curr]
            );
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.sequence[self.curr]));
            self.curr += 1;
            return item;
        }

        return None;
    }
}

/// デバッグ出力。  
impl fmt::Debug for Sequence1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        // イテレートするたびに .clone() するのは重くないか？
        let mut copy = self.clone();
        loop {
            // `copy.next()` ではなくて、 `self.next() {` と書けないの？
            if let Some(chr) = copy.next() {
                buf.push_str(&format!("{:?}", chr));
            } else {
                break;
            }
        }
        write!(f, "{}", buf)
    }
}
