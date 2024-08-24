use std::time::Instant;
use rand::prelude::*;

const PLAYERS: usize = 1;//_000;
const TRIALS: usize = 10;//_000;

const INCLUDE_DIAGONALS: bool = false;
const INCLUDE_FREE_SPACES: bool = false;
const SHOW_FULL_BALL_HISTOGRAM: bool = true;

////////////////////////////////////////////

const ALL_BINGOS: [u32; 44] = [
//B_DI_FREE
    //VERTICALS
    0b0000000000000000000011111,  //00
    0b0000000000000001111100000,  //01
    0b0000000000110110000000000,  //02
    0b0000011111000000000000000,  //03
    0b1111100000000000000000000,  //04
    //HORIZONTALS
    0b0000100001000010000100001,  //05
    0b0001000010000100001000010,  //06
    0b0010000100000000010000100,  //07
    0b0100001000010000100001000,  //08
    0b1000010000100001000010000,  //09
    //DIAGONALS
    0b1000001000000000001000001,  //10
    0b0000100010000000100010000,  //11
//B_DI_NOFREE
    // VERTICALS
    0b0000000000000000000011111,  //12
    0b0000000000000001111100000,  //13
    0b0000000000111110000000000,  //14
    0b0000011111000000000000000,  //15
    0b1111100000000000000000000,  //16
    //HORIZONTALS
    0b0000100001000010000100001,  //17
    0b0001000010000100001000010,  //18
    0b0010000100001000010000100,  //19
    0b0100001000010000100001000,  //20
    0b1000010000100001000010000,  //21
    // DIAGONALS
    0b1000001000001000001000001,  //22
    0b0000100010001000100010000,  //23
//B_NODI_FREE
    //VERTICALS
    0b0000000000000000000011111,  //24
    0b0000000000000001111100000,  //25
    0b0000000000110110000000000,  //26
    0b0000011111000000000000000,  //27
    0b1111100000000000000000000,  //28
    //HORIZONTALS
    0b0000100001000010000100001,  //29
    0b0001000010000100001000010,  //30
    0b0010000100000000010000100,  //31
    0b0100001000010000100001000,  //32
    0b1000010000100001000010000,  //33
//B_NODI_NOFREE
    // VERTICALS
    0b0000000000000000000011111,  //34
    0b0000000000000001111100000,  //35
    0b0000000000111110000000000,  //36
    0b0000011111000000000000000,  //37
    0b1111100000000000000000000,  //38
    //HORIZONTALS
    0b0000100001000010000100001,  //39
    0b0001000010000100001000010,  //40
    0b0010000100001000010000100,  //41
    0b0100001000010000100001000,  //42
    0b1000010000100001000010000,  //43
];

struct Card {
    spaces: [u8; 25],
    value: u32
}

fn main() {
    let mut test_bingos: Vec<u32> = ALL_BINGOS.to_vec();

    if INCLUDE_DIAGONALS {
        if INCLUDE_FREE_SPACES { test_bingos = (&test_bingos[0..=11]).to_vec(); }
        else { test_bingos = (&test_bingos[12..=23]).to_vec(); }
    } else {
        if INCLUDE_FREE_SPACES { test_bingos = (&test_bingos[24..=33]).to_vec(); }
        else { test_bingos = (&test_bingos[34..]).to_vec(); }
    }

    println!("testing {} possible bingos", test_bingos.len());

    let mut rng = thread_rng();
    let start_time = Instant::now();

    let mut win_after: Vec<u8> = Vec::new();

    let mut percent = 0;
    let loading_percent = TRIALS / 100;

    let mut win_kinds: Vec<u32> = Vec::new();
    test_bingos.iter().for_each(|_| win_kinds.push(0));

    let mut ball_histogram: Vec<u32> = Vec::new();
    (0..75).for_each(|_| ball_histogram.push(0));

    'new_game: for trial in 1..=TRIALS {

        if TRIALS >= 100 {
            if trial % loading_percent == 0 {
                percent += 1;
                println!("{}% done in {:?}...", percent, start_time.elapsed());
            }
        }

        let mut cards: Vec<Card> = Vec::new();
        // (0..PLAYERS).for_each(|_| cards.push(Card { spaces: make_new_card(&mut rng), value: 0 }));
        while cards.len() < PLAYERS {
            cards.push(Card { spaces: make_new_card(&mut rng), value: 0 });
        }

        let mut balls: [u8; 75] = [
            01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,
            16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,
            31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,
            46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,
            61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,
        ];

        balls.shuffle(&mut rng);

        let mut ball_count = 0;
        let mut game_will_end = false;

        for b in balls {
            ball_count += 1;

            let bmin: usize = (b - 1).into();

            ball_histogram[bmin] += 1;

            for c in &mut cards {
                if !c.spaces.contains(&b) { continue; }

                let power = u32::try_from(c.spaces.iter().position(|&x| x == b).unwrap()).ok().unwrap();
                c.value += 2_u32.pow(power);
                for bingo in &test_bingos {
                    if *bingo == (bingo & c.value) {
                        game_will_end = true;
                        for t in 0..test_bingos.len() {
                            if test_bingos[t] == (test_bingos[t] & c.value) { win_kinds[t] += 1 }
                        }
                    }
                }
            }

            if game_will_end { 
                win_after.push(ball_count);
                continue 'new_game;
            }
        }
    }

    println!("/// FINISHED! ///");
    println!("{TRIALS} games with {PLAYERS} player(s)");
    println!("/////////////////");

    if INCLUDE_DIAGONALS { println!("-< Including Diagonals >-") }
    else { println!("-< Not Including Diagonals >-") }
    if INCLUDE_FREE_SPACES { println!("-< Including Free Spaces >-") }
    else { println!("-< Not Including Free Spaces >-") }

    println!("/////////////////");

    println!("Total time: {:?}",start_time.elapsed());
    println!("Average time per game: {:?}",start_time.elapsed()/TRIALS.try_into().unwrap());
    println!("Shortest game: {} balls", win_after.iter().min().unwrap());
    println!("Longest game: {} balls" ,win_after.iter().max().unwrap());
    let mut sum: u64 = 0;
    win_after.iter().for_each(|x| sum = sum + u64::try_from(*x).unwrap() );
    println!("Average game: {} balls", sum/u64::try_from(TRIALS).unwrap() );

    println!("/////////////////");

    let win_kinds_sum: u32 = win_kinds.iter().sum();
    let v_wins: u32 = win_kinds[0..=4].iter().sum();
    let h_wins: u32 = win_kinds[5..=9].iter().sum();
    let mut d_wins: u32 = 0;
    if INCLUDE_DIAGONALS { d_wins = win_kinds[10..].iter().sum(); };

    for v in 0..win_kinds.len() {
        if v == 0 { println!("VERTICALS:") }
        if v == 5 { println!("HORIZONTALS:") }
        if v == 10 { println!("DIAGONALS:") }
        println!("Bingo type #{v}: {} ({:.2}%)", win_kinds[v], (win_kinds[v] * 100) as f64 / win_kinds_sum as f64 );
    }

    println!("/////////////////");

    println!("Vertical Wins: {:.2}%",(v_wins * 100) as f64 / win_kinds_sum as f64);
    println!("Horizon. Wins: {:.2}%",(h_wins * 100) as f64 / win_kinds_sum as f64);
    
    if INCLUDE_DIAGONALS { println!("Diagonal Wins: {:.2}%",(d_wins * 100) as f64 / win_kinds_sum as f64); };

    println!("/////////////////");


    for v in 1..=75 {
        let count = win_after.iter().filter(|c| **c == v).count();
        if count > 0 {
            println!("{v}: {count} times");
        }
    }

    println!("///////////////////////");
    println!("--< Ball Histogram >---");
    println!("///////////////////////");

    let mut bh_sum: u64 = 0;
    ball_histogram.iter().for_each(|x| bh_sum = bh_sum + u64::try_from(*x).unwrap() );

    let bh_avg = bh_sum / 75;

    if SHOW_FULL_BALL_HISTOGRAM {
        for bh in 0..ball_histogram.len() {
            println!("#{}, {} times -- {:.2}%", bh+1, ball_histogram[bh], (ball_histogram[bh] * 100) as f64 / (bh_sum as f64))
        }
    }

    let bh_max = ball_histogram.iter().max().unwrap();
    let bh_min = ball_histogram.iter().min().unwrap();

    println!("--------------");

    println!(" largest incidence: {bh_max}");
    println!("smallest incidence: {bh_min}");
    println!("    for a range of: {}", bh_max - bh_min);

    let bh_dev = (bh_max - bh_min) / 2;

    println!("deviance of +/-{bh_dev}, or +/-{:.3}%", (bh_dev * 100) as f64 / bh_avg as f64);

    println!("--------------");
}

fn make_new_card(rng: &mut ThreadRng) -> [u8; 25] {
    let mut card_spaces: [u8; 25] = [
        0,0,0,0,0,
        0,0,0,0,0,
        0,0,0,0,0,
        0,0,0,0,0,
        0,0,0,0,0
    ];

    let mut letter_elements: [u8; 15] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

    for y in 0..5 {
        letter_elements.shuffle(rng);
        for x in 0..5 {
            card_spaces[y*5 + x] = letter_elements[x] + u8::try_from(y * 15).ok().unwrap();
        }
    }
    return card_spaces;
}

fn _generate_bingo_number_lut() {
    let mut bingo_numbers: Vec<u32> = Vec::new();
    
    let mut bingo_powers: Vec<Vec<u8>> = Vec::new();
    bingo_powers.push([0,1,2,3,4].to_vec());
    bingo_powers.push([5,6,7,8,9].to_vec());
    bingo_powers.push([10,11,12,13,14].to_vec());
    bingo_powers.push([15,16,17,18,19].to_vec());
    bingo_powers.push([20,21,22,23,24].to_vec());
    bingo_powers.push([0,5,10,15,20].to_vec());
    bingo_powers.push([1,6,11,16,21].to_vec());
    bingo_powers.push([2,7,12,17,22].to_vec());
    bingo_powers.push([3,8,13,18,23].to_vec());
    bingo_powers.push([4,9,14,19,24].to_vec());
    bingo_powers.push([0,6,12,18,24].to_vec());
    bingo_powers.push([4,8,12,16,20].to_vec());
        
    
    for vector in bingo_powers {
    
        let mut constructing_number = 0;
    
        for val in 0..=24 {
            if vector.contains(&val) {
                println!("contains {val}, adding {}", (2_u32.pow(val.into())));
                constructing_number += 2_u32.pow(val.into());
                println!("bingo number is currently {constructing_number}");
            }
        }
    
        bingo_numbers.push(constructing_number);
    
    }
    
    println!("finished! bingo number is: {:?}", bingo_numbers);
    
    for number in bingo_numbers {
        println!("{number:#08} is {number:#027b}");
    }
}