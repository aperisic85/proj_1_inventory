#[derive(Debug, PartialEq, Eq)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn most_stocked(&self) -> ShirtColor {
        let mut red = 0;
        let mut blue = 0;
        for m in &self.shirts {
            match m {
                ShirtColor::Red => red += 1,
                ShirtColor::Blue => blue += 1,
            }
        }
        if red >= blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }

    fn giveaway(&mut self, user_pref: &Option<ShirtColor>) -> Result<ShirtColor, &'static str> {
        // user_pref.unwrap_or_else(|| self.most_stocked())
        match user_pref {
            Some(ShirtColor::Blue) => {
                let pos = self.shirts.iter().position(|x| *x == ShirtColor::Blue);
                match pos {
                    Some(x) => {
                        self.shirts.remove(x);
                        Ok(ShirtColor::Blue)
                    }
                    None => Err("We dont have it anymore"),
                }
            }
            Some(ShirtColor::Red) => {
                let pos = self.shirts.iter().position(|x| *x == ShirtColor::Red);
                match pos {
                    Some(x) => {
                        self.shirts.remove(x);
                        Ok(ShirtColor::Red)
                    }
                    None => Err("We dont have it anymore"),
                }
            }
            None => Ok(self.most_stocked()),
        }
    }
}

fn main() {
    let mut inv = Inventory {
        shirts: vec![
            ShirtColor::Red,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Red,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Blue,
        ],
    };

    let user1 = Some(ShirtColor::Blue);
    let user2 = Some(ShirtColor::Red);
    let user3: Option<ShirtColor> = None;

    println!(" inv state: {:?}", inv.shirts);

    println!("Giveaway for user 1: {:?}", inv.giveaway(&user1));
    println!("Giveaway for user 2: {:?}", inv.giveaway(&user2));
    println!("Giveaway for user 2: {:?}", inv.giveaway(&user2));
    println!("Giveaway for user 2: {:?}", inv.giveaway(&user2));
    println!("Giveaway for user 2: {:?}", inv.giveaway(&user2));
    println!("Giveaway for user 3: {:?}", inv.giveaway(&user3));
    println!(" inv new state: {:?}", inv.shirts);
}
