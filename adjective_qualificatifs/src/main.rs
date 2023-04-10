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

fn rules(endswith: &str) -> &str {
    match endswith {
        "commune" => "ajouter e pour feminin et s pour pluriel",
        "l" => "ajouter e pour feminin et s pour pluriel",
        "un" => "ajouter e pour feminin et s pour pluriel",
        "eau" => "ajouter -elle pour feminin et x pour pluriel",
        "ier" => "prennent un accent grave sur le e et un e final.",
        "iet" => "prennent un accent grave sur le e et un e final.",
        "ien" => "doublent leur n et prennent un e final.",
        "en" => "doublent leur n et prennent un e final.",
        "on" => "doublent leur n et prennent un e final.",
        "eur" => "font leur féminin en -euse",
        "e" => "aucun changement au feminin",
        "s" => "pas de différence pour le masculin singulier ou pluriel",
        "x" => "pas de différence pour le masculin singulier ou pluriel",
        "gu" => "prennent un e à la fin et un tréma sur le u",
        "eil" => "doublent leur -l et prennent un e final.",
        "ul" => "doublent leur -l et prennent un e final.",
        "el" => "doublent leur -l et prennent un e final.",
        "iel" => "doublent leur -l et prennent un e final.",
        "al" => "feminin form et regular",
        "an" => "il n'y a pas de forme régulière",
        "in" => "il y a une forme régulière",
        "s-except" => "doublent leur -s et prennent un e final.",
        "c-except" => "prennent un che ou cque final.",
        "et" => "doublent leur -t et prennent un e final.",
        "f" => "un ve final.",
        "g" => "prennent un gue à la fin",
        "c" => "prennent un che à la fin",
        "er" => "prennent un ere à la fin",
        "x-except" => "prennent un se, ce ou lle à la fin",
        _ => "resto",
    }
}

fn main() {
    let _adjs = HashMap::from([
        (
            Adjctifs::new("strong", "fort", "forts", "forte", "fortes"),
            "commune",
        ),
        (
            Adjctifs::new("new", "nouveau", "nouveaux", "nouvelle", "nouvelles"),
            "eau",
        ),
        (
            Adjctifs::new("small", "petit", "petits", "petite", "petites"),
            "commune",
        ),
        (
            Adjctifs::new("pretty", "joli", "jolis", "jolie", "jolies"),
            "commune",
        ),
        (
            Adjctifs::new("green", "vert", "verts", "verte", "vertes"),
            "commune",
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
        (
            Adjctifs::new("ancient", "ancien", "anciens", "anciene", "ancienes"),
            "en",
        ),
        (
            Adjctifs::new("fine/thin", "fin", "fins", "fine", "fines"),
            "in",
        ),
        (
            Adjctifs::new("brown/dark", "brun", "bruns", "brune", "brunes"),
            "un",
        ),
        (
            Adjctifs::new("persian", "persan", "persans", "persane", "persanes"),
            "an",
        ),
        (
            Adjctifs::new("rural", "paysan", "paysans", "paysanne", "paysannes"),
            "an",
        ),
        (
            Adjctifs::new("fatty/greasy", "gras", "gras", "grasse", "grasses"),
            "s-except",
        ),
        (
            Adjctifs::new("down/low", "bas", "bas", "basse", "basses"),
            "s-except",
        ),
        (
            Adjctifs::new("fat", "gros", "gros", "grosse", "grosses"),
            "s-except",
        ),
        (
            Adjctifs::new("thick", "epais", "epais", "epaisse", "epaisses"),
            "s-except",
        ),
        (
            Adjctifs::new("absolved", "absous", "absous", "absoute", "absoutes"),
            "s-except",
        ),
        (
            Adjctifs::new("dissolved", "dissous", "dissous", "dissoute", "dissoutes"),
            "s-except",
        ),
        (
            Adjctifs::new("fresh", "frais", "frais", "fraiche", "fraiches"),
            "s-except",
        ),
        (
            Adjctifs::new("neat/clean", "net", "nets", "nette", "nettes"),
            "et",
        ),
        (
            Adjctifs::new("new", "neuf", "neufs", "neuve", "neuves"),
            "f",
        ),
        (
            Adjctifs::new("happy", "heureux", "heureux", "heureuse", "heureuses"),
            "x",
        ),
        (
            Adjctifs::new("soft/sweet", "doux", "doux", "douce", "douces"),
            "x-except",
        ),
        (
            Adjctifs::new("fake", "faux", "faux", "fausse", "fausses"),
            "x-except",
        ),
        (
            Adjctifs::new("old", "vieux", "vieux", "vielle", "vielles"),
            "x-except",
        ),
        (
            Adjctifs::new("long", "long", "longs", "longue", "longues"),
            "g",
        ),
        (
            Adjctifs::new("white", "blanc", "blancs", "blanche", "blanches"),
            "c",
        ),
        (
            Adjctifs::new("dry", "sec", "secs", "seche", "seches"),
            "c-except",
        ),
        (
            Adjctifs::new("greek", "grec", "grecs", "grecque", "grecques"),
            "c-except",
        ),
        (
            Adjctifs::new("public", "public", "publics", "publique", "publiques"),
            "c-except",
        ),
        (
            Adjctifs::new("turkish", "turc", "turcs", "turque", "turques"),
            "c-except",
        ),
        (
            Adjctifs::new("expensive", "cher", "chers", "chere", "cheres"),
            "er",
        ),
        (
            Adjctifs::new("best", "meilleur", "meilleurs", "meilleure", "meilleures"),
            "eur",
        ),
        (
            Adjctifs::new(
                "flattering",
                "flatteur",
                "flatteurs",
                "flatteuse",
                "flatteuses",
            ),
            "eur",
        ),
        (
            Adjctifs::new(
                "enchanting",
                "enchanteur",
                "enchanteurs",
                "enchanteresse",
                "enchanteresses",
            ),
            "eur",
        ),
        (
            Adjctifs::new(
                "destructive",
                "destructeur",
                "destructeurs",
                "destructrice",
                "destructrices",
            ),
            "eur",
        ),
    ]);

    for (adjs, ends) in &_adjs {
        //println!("{adjs:?}, ends with {ends}");
        //println!("{} ends with {}", adjs.english, rules(ends))
        loop {
            println!("complet ");
            println!("The adjective {} in french:", adjs.english);
            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Veuillez fournir un adjectif qualificatifs");
            let guess: String = match guess.trim().parse() {
                Ok(str) => str,
                Err(_) => continue,
            };
            println!("Masculin singulier");
            if compares(&guess, &adjs.masculine_s) == true {
                break;
            }
        }
    }
}

fn compares(guess: &String, adjs: &String) -> bool {
    match guess == adjs {
        true => {
            println!("{} est correct", &guess);
            return true;
        }
        _ => {
            println!("essayer a nouveau!");
            return false;
        }
    }
}
