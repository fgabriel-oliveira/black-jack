extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::io::stdin;

fn start() -> u32 {
    let mut hand = 0;
    let mut counter = 0;

    while counter < 3 {
        hand += thread_rng().gen_range(1, 13);
        
        counter += 3;
    }

    hand
}

fn cmd_read() -> bool {
    let mut line = String::new();

    println!("Você deseja comprar uma nova carta? [s/n]");
    stdin()
    .read_line(&mut line)
    .expect("fail");
   
    
    match line.as_str() {
        "s\n" => true,
        "n\n" => false,
        _ => panic!("Valor inválido"),
    }
}

fn hit(hand: &mut u32) -> u32 {
    *hand += thread_rng().gen_range(1, 13);

    *hand
}

fn main() {
    let mut bot_hand: u32 = start();
    let mut player_hand: u32 = start();

    println!("Esse é um simples jogo de black jack");

    loop {
        println!("bot:{} player:{}", bot_hand, player_hand);


        if bot_hand < 21 && player_hand < 21 {
            let movement = cmd_read();

            if movement == true {
                player_hand = hit(&mut player_hand);
                bot_hand = hit(&mut bot_hand);
            }
            else {
                break;
            }
        } else {
            break;
        }
    
    
    }

    

    if (21 - player_hand) < (21 - bot_hand) {
        println!("Você venceu");
    }else if  (21 - player_hand) > (21 - bot_hand){
        println!("Você perdeu");
    }else {
        println!("Empate");
    }
}

