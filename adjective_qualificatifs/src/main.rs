use std::collections::HashMap;
use std::io;
use std::iter;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Adjctifs {
    english: String,
    masculine_s: String,
    masculine_p: String,
    feminine_s: String,
    feminine_p: String,
}

impl Adjctifs {
    fn new(
        english: &str,
        masculine_s: &str,
        masculine_p: &str,
        feminine_s: &str,
        feminine_p: &str,
    ) -> Adjctifs {
        Adjctifs {
            english: english.to_string(),
            masculine_s: masculine_s.to_string(),
            masculine_p: masculine_p.to_string(),
            feminine_s: feminine_s.to_string(),
            feminine_p: feminine_p.to_string(),
        }
    }
}

fn main() {
    let _adjs = HashMap::from([
        (
            Adjctifs::new("strong", "fort", "forts", "forte", "fortes"),
            "t",
        ),
        (
            Adjctifs::new("new", "nouveau", "nouveaux", "nouvelle", "nouvelles"),
            "au",
        ),
        (
            Adjctifs::new("small", "petit", "petits", "petite", "petites"),
            "t",
        ),
        (
            Adjctifs::new("pretty", "joli", "jolis", "jolie", "jolies"),
            "i",
        ),
        (
            Adjctifs::new("green", "vert", "verts", "verte", "vertes"),
            "t",
        ),
        (
            Adjctifs::new("red", "rouge", "rouges", "rouge", "rouges"),
            "e",
        ),
        (
            Adjctifs::new("gray", "gris", "grise", "gris", "grises"),
            "s",
        ),
        (
            Adjctifs::new("acute", "aigu", "aigus", "aigue", "aigues"),
            "gu",
        ),
        (
            Adjctifs::new("cruel", "cruel", "cruels", "cruelle", "cruelles"),
            "el",
        ),
        (
            Adjctifs::new("similar", "pareil", "pareils", "pareille", "pareilles"),
            "eil",
        ),
        (
            Adjctifs::new("normal", "normal", "normals", "normale", "normales"),
            "al",
        ),
        (
            Adjctifs::new("alone/lonely", "seul", "seuls", "seule", "seules"),
            "l",
        ),
        (
            Adjctifs::new("hopeless/void", "nul", "nuls", "nulle", "nulles"),
            "l",
        ),
        (
            Adjctifs::new("civil", "civil", "civils", "civile", "civiles"),
            "l",
        ),
        (
            Adjctifs::new("kind", "gentil", "gentils", "gentille", "gentilles"),
            "l",
        ),
        (
            Adjctifs::new("good", "bon", "bons", "bonne", "bonnes"),
            "on",
        ),
    ]);

    for (adjs, ends) in &_adjs {
        //println!("{adjs:?}, ends with {ends}");
        println!("{} ends with {}", adjs.english, ends)
    }
}
