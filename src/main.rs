#[derive(Debug, Clone)] //implements dbg trait automaticaly
struct Miro {
    age: u32,
}

fn main() {
    let miro = Miro { age: 29 };

    // Move - most efficient

    let mi = miro;

    dbg!(mi);

    // Clone

    let me = Miro { age: 29 };

    dbg!(&me);

    let mewtoo = me.clone(); // move ownership

    // Data Ownership !

    dbg!(&mewtoo);

    dbg!(me);

    // Borrow

    let mut pikachu = Miro { age: 16 }; // in rust bindings (symbol data relations) are immutable per default

    //let pika = &pikachu; // borrow - throws error

    let raichu = &mut pikachu; // mutable borrow - completly exclusive (can't borrow other things at same time)

    dbg!(&raichu);

    raichu.age = 12;

    dbg!(&pikachu);
}
