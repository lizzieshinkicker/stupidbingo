use std::time::Instant;
use rand::prelude::*;


/////////////////////////////
/// stupidbingo           ///
/////////////////////////////
/// 
/// (against 10,000 trials)
/// DEBUG // OPTIMIZED times
/// 
/// 1 Player /////////////
/// 79.001µs // 3.711µs
/// 
/// 
/// 10 Players ///////////////
/// 399.522µs // 10.976µs
/// 
/// 
/// 100 Players //////////////
/// 3.526ms // 81.003µs
/// 
/////////////////////////////////

const PLAYERS: usize = 10_000_000;
const TRIALS: usize = 100;

const BINGOS: [u32; 12] = [
    //FREE SPACE
    //VERTICALS
    0b0000000000000000000011111,
    0b0000000000000001111100000,
    0b0000000000110110000000000,
    0b0000011111000000000000000,
    0b1111100000000000000000000,
    //HORIZONTALS
    0b0000100001000010000100001,
    0b0001000010000100001000010,
    0b0010000100000000010000100,
    0b0100001000010000100001000,
    0b1000010000100001000010000,
    //DIAGONALS
    0b1000001000000000001000001,
    0b0000100010000000100010000

    // NO FREE SPACE
    // VERTICALS
    // 0b0000000000000000000011111,
    // 0b0000000000000001111100000,
    // 0b0000000000111110000000000,
    // 0b0000011111000000000000000,
    // 0b1111100000000000000000000,
    // HORIZONTALS
    // 0b0000100001000010000100001,
    // 0b0001000010000100001000010,
    // 0b0010000100001000010000100,
    // 0b0100001000010000100001000,
    // 0b1000010000100001000010000,
    // DIAGONALS
    // 0b1000001000001000001000001,
    // 0b0000100010001000100010000
];

struct Card {
    spaces: [u8; 25],
    value: u32
}

fn main() {
    let mut rng = thread_rng();
    let start_time = Instant::now();

    let mut win_after: Vec<u8> = Vec::new();

    let mut percent = 0;
    let loading_percent = TRIALS / 100;

    let mut win_kinds: Vec<u32> = Vec::new();

    for _ in BINGOS {
        win_kinds.push(0);
    }

    'new_game: for trial in 1..=TRIALS {

        if TRIALS >= 100 {
            if trial % loading_percent == 0 {
                percent += 1;
                println!("{}% done in {:?}...", percent, start_time.elapsed());
            }
        }

        let mut cards: Vec<Card> = Vec::new();

        while cards.len() < PLAYERS {
            cards.push(Card {spaces: make_new_card(&mut rng), value: 0});
        }

        let mut balls: [u8; 75] = [
            01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,
            16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,
            31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,
            46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,
            61,62,63,64,65,66,67,68,69,70,71,72,73,74,75
        ];

        balls.shuffle(&mut rng);

        let mut ball_count = 0;
        let mut game_will_end = false;



        for b in balls {
            ball_count += 1;
            for c in &mut cards {
                if c.spaces.contains(&b) {
                    let power = u32::try_from(c.spaces.iter().position(|&x| x == b).unwrap()).ok().unwrap();
                    c.value += 2_u32.pow(power);
                    for b in BINGOS {
                        if b == (b & c.value){
                            game_will_end = true;
                            for t in 0..BINGOS.len(){
                                if BINGOS[t] == (BINGOS[t] & c.value) { win_kinds[t] += 1 }
                            }
                            
                            //continue 'new_game;
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
    let d_wins: u32 = win_kinds[10..].iter().sum();

    for v in 0..win_kinds.len() {
        println!("Bingo type #{v}: {} ({:.2}%)", win_kinds[v], (win_kinds[v] * 100) as f64 / win_kinds_sum as f64 );
    }

    println!("/////////////////");

    println!("Vertical Wins: {:.2}%",(v_wins * 100) as f64 / win_kinds_sum as f64);
    println!("Horizon. Wins: {:.2}%",(h_wins * 100) as f64 / win_kinds_sum as f64);
    println!("Diagonal Wins: {:.2}%",(d_wins * 100) as f64 / win_kinds_sum as f64);

    println!("/////////////////");


    for v in 1..=75 {
        let count = win_after.iter().filter(|c| **c == v).count();
        if count > 0 {
            println!("{v}: {count} times");
        }
    }
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