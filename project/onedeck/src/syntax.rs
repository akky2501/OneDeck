use super::obj;

pub fn fulldeck() -> Vec<obj::Card> {
    use obj::Card;
    use obj::Suit::*;

    let mut d = Vec::new();
    for s in vec![Spade, Hart, Club, Diamond] {
        for n in 1..=13 {
            d.push(Card(s, n));
        }
    }
    d.push(Card(Joker, 1));
    d.push(Card(Joker, 2));
    d
}


#[derive(Debug)]
pub enum Toplevel {
    DeclCards(String, obj::Object),
    DeclActor(String, String, Vec<String>), // 名前、役割名、パラメタ
    DeclEntry(obj::Action),
    DeclRole(String, obj::Role),
}