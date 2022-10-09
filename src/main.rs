use console::Term;
use owo_colors::OwoColorize;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::{
    io::Read,
    time::{Duration, Instant},
};

fn main() {
    println!("{}[2J", 27 as char);

    // shouldnt have to hit enter after every word!

    //vision:
    // 4-5 words to type on screen
    // current word is highlighted/different color
    // that way you can look ahead
    // enter a word and they will all shift to the left
    // enter word wrong and it will appear as red so you know you got it wrong

    println!("\n\n{}", ("turbotype").truecolor(100, 200, 44));
    // println!("{}", ("type the sentences exactly as they appear on the screen").truecolor(100, 200, 44).dimmed());

    //    println!("{}", ("(100, 200, 44)").truecolor(100, 200, 44));
    //   println!("{}", ("(44, 200, 100)").truecolor(44, 200, 100));
    //  println!("{}", ("(44, 100, 200)").truecolor(44, 100, 200));
    // println!("{}", ("(100, 44, 200)").truecolor(100, 44, 200));
    //println!("{}", ("(200, 44, 100)").truecolor(200, 44, 100));
    //println!("{}", ("(200, 100, 44)").truecolor(200, 100, 44));

    println!("{}", ("press enter to start").truecolor(200, 100, 44));
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("cant");
        match input.trim() {
            _ => break,
        }
    }

    //   choose how many words to type
    // choose what topic to type for sentences

    let mut sentence = vec![
        String::from("The industrial revolution and its consequences have been a disaster for the hum"),
        String::from("Ted Kaczynski has an IQ of 167."),
        String::from("At the age of 15 years old, he was admitted into Harvard University."),
        String::from("At the age of 25 Uncle Ted was the yougest Math Professor to ever teach at UC Berkley."),
        String::from("Uncle Ted was right."),
        String::from("Uncle Ted was right."),
        String::from("The Winter Solstice is Wednesday, December 21."),
        String::from("The Winter Solstice has the least daylight and the longest night out of any day in the year."),
        String::from("At the Sinter Wolstice the Sun travels the shortest path through the sky."),
        String::from("This coming Winter $olstice is the official start of Wigger Winter."),

    ];
    sentence.shuffle(&mut thread_rng());

    let mut word = vec![
        String::from("commit"),
        String::from("git"),
        String::from("pull"),
        String::from("--version"),
        String::from("watch"),
        String::from("clocked"),
        String::from("event"),
        String::from("struct"),
        String::from("enum"),
        String::from("use"),
        String::from("String"),
        String::from("&str"),
        String::from("i32"),
        String::from("usize"),
        String::from("pub"),
        String::from("main"),
        String::from("master"),
        String::from("--help"),
        String::from("../.."),
        String::from("&&"),
        String::from("three"),
        String::from("Crustacean"),
        String::from("reddit"),
        String::from("sucks"),
        String::from("yipee"),
        String::from("wire"),
        String::from("motivate"),
        String::from("focus"),
        String::from("yield"),
        String::from("margins"),
        String::from("sale"),
        String::from("etsy"),
        String::from("finance"),
        String::from("purple"),
        String::from("pink"),
        String::from("clink"),
        String::from("plattitude"),
        String::from("plank"),
        String::from("jet"),
        String::from("competition"),
        String::from("chess"),
        String::from("angle"),
        String::from("angel"),
        String::from("pasta"),
        String::from("pancake"),
        String::from("Dale"),
        String::from("office"),
        String::from("picture"),
        String::from("jeopardy"),
        String::from("noteworthy"),
        String::from("overflowing"),
        String::from("absolutely"),
        String::from("considerably"),
        String::from("Ouroboros"),
        String::from("Mantle"),
        String::from("Flume"),
        String::from("hoard"),
        String::from("diagnose"),
        String::from("self"),
        String::from("trends"),
        String::from("autistic"),
        String::from("retarded"),
        String::from("CoinBase"),
        String::from("Bitcoin"),
        String::from("ethereum"),
        String::from("Eth"),
        String::from("fast"),
        String::from("Solana"),
        String::from("anchor"),
        String::from("account"),
        String::from("derive"),
        String::from("default"),
        String::from("debug"),
        String::from("wahoo"),
        String::from("tony"),
        String::from("Tony"),
        String::from("fortune"),
        String::from("cookies"),
        String::from("prosciutto"),
        String::from("puppy"),
        String::from("cheese"),
        String::from("copper"),
        String::from("T-Rex"),
        String::from("420"),
        String::from("biden"),
        String::from("trump"),
        String::from("shart"),
        String::from("twitter"),
        String::from("oppose"),
        String::from("Jxxyy"),
        String::from("YouTube"),
        String::from("animal"),
        String::from("art"),
        String::from("lol"),
        String::from("lololololol"),
        String::from("ventriloquist"),
        String::from("cardboard"),
        String::from("duck"),
        String::from("sauce"),
        String::from("lost"),
        String::from("emboss"),
        String::from("tortoise"),
        String::from("moist"),
        String::from("fart"),
        String::from("fort"),
        String::from("fick"),
        String::from("flick"),
        String::from("flickadoodle"),
        String::from("cornopolis"),
        String::from("corn"),
        String::from("tonka"),
        String::from("based"),
        String::from("cringe"),
        String::from("Kaczynski"),
        String::from("boof"),
        String::from("Bidoof"),
        String::from("smash"),
        String::from("burger"),
        String::from("trick"),
        String::from("Halloween"),
        String::from("Ikea"),
        String::from("Gerald"),
        String::from("Iman"),
        String::from("Pootie"),
        String::from("Thespian"),
        String::from("twenty"),
        String::from("stuff"),
        String::from("lobster"),
        String::from("pointed"),
        String::from("Shrek"),
        String::from("Dragon"),
        String::from("chuck"),
        String::from("guitar"),
        String::from("drums"),
        String::from("suite"),
        String::from("match"),
        String::from("cargo"),
        String::from("run"),
        String::from("ruining"),
        String::from("plane"),
        String::from("plain"),
        String::from("AC/DC"),
        String::from("Disney"),
        String::from("NASA"),
        String::from("CIA"),
        String::from("AWS"),
        String::from("Obama"),
        String::from("mouse"),
        String::from("harness"),
        String::from("glock"),
        String::from("switch"),
        String::from("along"),
        String::from("trophy"),
        String::from("wife"),
        String::from("paragraph"),
        String::from("password"),
        String::from("congo"),
        String::from("jungle"),
        String::from("pulp"),
        String::from("tarantula"),
        String::from("restaurant"),
        String::from("with"),
        String::from("without"),
        String::from("eighty"),
        String::from("kitchen"),
        String::from("track"),
        String::from("middle"),
        String::from("walk"),
        String::from("song"),
        String::from("rhyme"),
        String::from("love"),
        String::from("lift"),
        String::from("this"),
        String::from("lived"),
        String::from("mission"),
        String::from("life"),
        String::from("chief"),
        String::from("good"),
        String::from("trained"),
        String::from("kept"),
        String::from("tout"),
        String::from("taut"),
        String::from("taught"),
        String::from("click"),
        String::from("type"),
        String::from("catalog"),
        String::from("part"),
        String::from("space"),
        String::from("station"),
        String::from("is"),
        String::from("probably"),
        String::from("fake"),
        String::from("earth"),
        String::from("definitely"),
        String::from("flat"),
        String::from("what's"),
        String::from("hidden"),
        String::from("in"),
        String::from("Antarctica"),
        String::from("innocence"),
        String::from("naked"),
        String::from("exposed"),
        String::from("lawyer"),
        String::from("post"),
        String::from("tread"),
        String::from("collage"),
        String::from("college"),
        String::from("death"),
        String::from("beckon"),
        String::from("plead"),
        String::from("guilty"),
        String::from("prison"),
        String::from("pipeline"),
        String::from("designer"),
        String::from("God"),
        String::from("Jesus"),
        String::from("clothe"),
        String::from("cling"),
        String::from("climb"),
        String::from("clean"),
        String::from("blanket"),
        String::from("down"),
        String::from("bed"),
        String::from("lamb"),
        String::from("little"),
        String::from("Mary"),
        String::from("smile"),
        String::from("bitterness"),
        String::from("big"),
        String::from("child"),
        String::from("kangaroo"),
        String::from("court"),
        String::from("jester"),
        String::from("odd"),
        String::from("even"),
        String::from("low"),
        String::from("high"),
        String::from("tower"),
        String::from("shower"),
        String::from("question"),
        String::from("animate"),
        String::from("pizza"),
        String::from("pizzas"),
        String::from("pizazz"),
        String::from("suburban"),
        String::from("assuming"),
        String::from("obstinance"),
        String::from("foramens"),
        String::from("Milady"),
        String::from("beaver"),
        String::from("Pixelady"),
        String::from("triple"),
        String::from("truthful"),
        String::from("control"),
        String::from("typing"),
        String::from("blazingly"),
        String::from("fast"),
        String::from("easy"),
        String::from("landshark"),
        String::from("stack"),
        String::from("triangle"),
        String::from("rude"),
        String::from("rust"),
        String::from("crab"),
        String::from("crowd"),
        String::from("crazy"),
        String::from("craziness"),
        String::from("crave"),
        String::from("cavity"),
        String::from("cardiology"),
        String::from("myocarditis"),
        String::from("shopper"),
        String::from("forced"),
        String::from("see"),
        String::from("run"),
        String::from("wild"),
        String::from("how"),
        String::from("I"),
        String::from("you"),
        String::from("now"),
        String::from("nothing"),
        String::from("final"),
        String::from("vinyl"),
        String::from("myself"),
        String::from("heart"),
        String::from("lymph"),
        String::from("crossed"),
        String::from("ketamine"),
        String::from("work"),
        String::from("exercise"),
        String::from("exorcism"),
        String::from("Oregon"),
        String::from("California"),
        String::from("valid"),
        String::from("lifetime"),
        String::from("function"),
        String::from("Boeing"),
        String::from("developer"),
        String::from("hypocrite"),
        String::from("criteria"),
        String::from("blessed"),
        String::from("awe"),
        String::from("leopard"),
        String::from("fishes"),
        String::from("rise"),
        String::from("flood"),
        String::from("wooden"),
        String::from("desert"),
        String::from("americano"),
        String::from("Canada"),
        String::from("banana"),
        String::from("trappings"),
        String::from("ceiling"),
        String::from("walking"),
        String::from("eloquent"),
        String::from("abundant"),
        String::from("strong"),
        String::from("focused"),
        String::from("determined"),
        String::from("patient"),
        String::from("smart"),
        String::from("clever"),
        String::from("wise"),
        String::from("humble"),
    ];
    // shuffles order
    word.shuffle(&mut thread_rng());

    // user chooses what to type
    let mut choice: Vec<String> = vec![];
    let mut num = String::new();
    let mut amount: usize = 0;
    println!("{}[2J", 27 as char);
    println!(
        "{}",
        ("sentences, words, or some other third thing?").truecolor(200, 44, 100)
    );
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("cant");
        match input.trim() {
            "words" => {
                println!("{}[2J", 27 as char);
                println!("{}", ("How many words?").truecolor(100, 200, 44));
                choice = word.to_owned();
                io::stdin().read_line(&mut num).unwrap();

                match num.trim().parse::<usize>().unwrap() {
                    x if x <= 250 => amount = x,
                    _ => (),
                }
                break;
            }
            "sentences" => {
                choice = sentence.to_owned();
                break;
            }
            "x" => experimental(&word, amount),
            _ => (),
        }
        println!(
            "{}",
            ("choose an option to continue")
                .truecolor(44, 100, 200)
                .dimmed()
        )
    }

    // timer begins
    let start = Instant::now();
    // game logic
    type_race(choice, amount);
    // time ends
    let duration = start.elapsed();
    // show time
    println!(
        "{} {:?}",
        ("time").truecolor(200, 100, 44),
        duration.truecolor(100, 200, 44)
    );
}

fn hp_bar(hp: i32) {
    let heart = String::from("💖");
    let broken_heart = String::from("❤️‍🩹");
    match hp {
        x if x > 3 => {
            for h in 0..hp {
                print!("{}", heart)
            }
        }
        x if x <= 3 => {
            for h in 0..hp {
                print!("{}", broken_heart)
            }
        }

        _ => {}
    }
}

fn graded_score(score: f64, amount: f64) -> String {
    let grade = score / amount * 100.0;
    match grade as i32 {
        100 => String::from("💯"),
        g if 97 <= g && g < 100 => String::from("A+"),
        g if 93 <= g && g < 97 => String::from("A"),
        g if 90 <= g && g < 93 => String::from("A-"),
        g if 87 <= g && g < 90 => String::from("B+"),
        g if 83 <= g && g < 87 => String::from("B"),
        g if 80 <= g && g < 83 => String::from("B-"),
        g if 77 <= g && g < 80 => String::from("C+"),
        g if 73 <= g && g < 77 => String::from("C"),
        g if 70 <= g && g < 73 => String::from("C-"),
        g if 67 <= g && g < 70 => String::from("C+"),
        g if 63 <= g && g < 67 => String::from("C"),
        g if 60 <= g && g < 63 => String::from("C-"),
        _ => String::from("F"),
    }
}

fn type_race(choice: Vec<String>, amount: usize) {
    let mut score = 0;
    let mut hp = 3;
    let mut i = 0;

    loop {
        // check to see if player is out of lives
        match hp {
            0 => {
                println!("{}[2J", 27 as char);
                println!("🖤🖤🖤");
                println!("{}", ("u lose").truecolor(100, 44, 200));
                println!(
                    "{} {}",
                    ("score").truecolor(200, 100, 44),
                    score.truecolor(100, 200, 44)
                );
                println!(
                    "{} {}",
                    ("grade").truecolor(200, 44, 100),
                    graded_score(score as f64, amount as f64).truecolor(200, 100, 44)
                );
                break;
            }

            _ => {}
        }
        println!("{}[2J", 27 as char);
        let c = i + 1;
        println!(
            "{}{}{}",
            c.truecolor(44, 200, 100),
            ("/").truecolor(44, 200, 100),
            amount.truecolor(44, 200, 100)
        );
        hp_bar(hp);
        println!("\n{}", choice[i].truecolor(100, 200, 44));
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("cant");
        match input.trim() {
            x if x == choice[i] => {
                score += 1;
                i += 1
            }
            _ => {
                hp -= 1;
                i += 1
            }
        }

        match i {
            i if i == amount => {
                println!("{}[2J", 27 as char);
                println!("{}", ("finished!").truecolor(44, 100, 200));
                println!(
                    "{}{}",
                    ("score ").truecolor(200, 100, 44),
                    score.truecolor(100, 200, 44)
                );
                println!(
                    "{} {}",
                    ("grade").truecolor(100, 44, 200),
                    graded_score(score as f64, amount as f64).truecolor(200, 100, 44)
                );
                break;
            }
            _ => {}
        }
    }
}

fn experimental(option: &Vec<String>, amount: usize) {
    let mut score = 0;
    let mut hp = 10;
    // current word
    let mut i: usize = 0;
    // current letter
    let mut c: usize = 0;
    // key input
    let key = Term::buffered_stdout();
    'test_loop: loop {
        // current word's length
        let mut z = option[i].chars().count();
        // check hp lvls
        match hp {
            0 => {
                println!("{}[2J", 27 as char);
                println!("🖤🖤🖤🖤🖤");
                println!("{}", ("u lose").truecolor(100, 44, 200));
                println!(
                    "{} {}",
                    ("score").truecolor(200, 100, 44),
                    score.truecolor(100, 200, 44)
                );
                println!(
                    "{} {}",
                    ("grade").truecolor(200, 44, 100),
                    graded_score(score as f64, amount as f64).truecolor(200, 100, 44)
                );
                break;
            }

            _ => {}
        }
        // clear screen
        println!("{}[2J", 27 as char);
        // print hp bar
        hp_bar(hp);
        println!("");
        for l in option[i].chars() {
            if let Ok(character) = key.read_char() {
                match character {
                    k if k == option[i].chars().nth(c).unwrap() => {
                        c += 1;
                    }
                    _ => {
                        score -= 1;
                        hp -= 1;
                    }
                }
            }
            match l {
                l if l == option[i].chars().nth(c).unwrap() => {
                    print!("{}", l.truecolor(100, 200, 44))
                }
                _ => {
                    println!("");
                    print!("{}", l.green().dimmed())
                }
            }
        }
        println!("");

        if c == z {
            score += 1;
            i += 1;
            c = 0
        }
    }
}

// print each word char by cahr
// print each char if typed correctly it is highlighted, else red
