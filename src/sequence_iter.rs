use crate::SequenceIter;

/// 普段よく見るイテレーションのベースとなる、  
/// 普段あまり見ない `items.next()` を使ったイテレーションの実装です。  
impl Iterator for SequenceIter {
    // Self::Item ってこれ。
    type Item = Box<char>;

    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < self.owner.sequence.len() {
            // .clone() するよりは Box でラッピングした方がいいだろうか？
            let item = Some(Box::new(self.owner.sequence[self.curr]));
            self.curr += 1;
            return item;
        }

        return None;
    }
}
