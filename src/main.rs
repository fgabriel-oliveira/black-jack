extern crate rand;

use rand::thread_rng;
use rand::Rng;
use std::io::stdin;


struct Card {
    naipe: u32,
    number: u32,
}
impl Card {
fn number_show(&self) -> String {
    if self.number >= 11 {
        match self.number {
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            _ => panic!("Algo de errado aconteceu")
        }
    }else {
        format!("{}", self.number)
    }
}

fn naipe_show(&self) -> &str {
    match self.naipe {
        0 =>  "♠️",
        1 =>  "♥️",
        2 => "♦️",
        3 => "♣️",
        _ => panic!("algo deu errado")
    }
    
}
}

fn win_check(bot_hand:&u32, player_hand:&u32) -> bool {
    if bot_hand > &21{
        true
    }else if player_hand > &21 {
        false
    }else if player_hand > bot_hand {
        true
    }else if bot_hand > player_hand {
        false
    }else{
        panic!("algo deu errado");
    }
}

fn cards_print(hand:&Vec<Card>) {
    for carta in hand {
        println!("{}{} ", carta.number_show(), carta.naipe_show());
    }
    
    }

fn cards_gen() -> Card {
    let card_number = thread_rng().gen_range(1, 14);
    let card_naipe = thread_rng().gen_range(0, 4);

    let carta = Card {
        number: card_number,
        naipe: card_naipe,
    };

    carta
}

fn start() -> (Vec<Card>, u32) {
    let mut hand = Vec::new();
    let mut counter = 0;
    let mut sum = 0;
    while counter < 2 {
        let card = cards_gen();
        sum += card.number;
        hand.push(card);
        
        counter += 1;
    }

    (hand, sum)
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

fn main() {
    let mut bot_hand = start();
    let mut player_hand = start();

    
    println!("\nPlayer:");
    cards_print(&player_hand.0);
    println!("Total:{}", player_hand.1);

    println!("\nBot:");
    cards_print(&bot_hand.0);
    println!("Total: {}", bot_hand.1);


    loop {
        if bot_hand.1 >= 21 || player_hand.1 >= 21 {
            break
        }else{
            
            if cmd_read(){
                let player_card = cards_gen();
                player_hand.1 += &player_card.number;
                player_hand.0.push(player_card);

            if bot_hand.1 < player_hand.1 && player_hand.1 <= 21 {
                let bot_card = cards_gen();
                bot_hand.1 += &bot_card.number;
                bot_hand.0.push(bot_card);
            }

            }else{
                break;
            }
        }

        println!("\nPlayer: ");
        cards_print(&player_hand.0);
        println!("Total: {}", player_hand.1);

        println!("\nBot:");
        cards_print(&bot_hand.0);
        println!("Total: {}", bot_hand.1);
    }

    println!("\nPlayer: ");
    cards_print(&player_hand.0);
    println!("Total: {}", player_hand.1);

    println!("\nBot:");
    cards_print(&bot_hand.0);
    println!("Total: {}", bot_hand.1);

    if win_check(&bot_hand.1, &player_hand.1){
        println!("\nVocê venceu");
    }else {
        println!("\nVocê perdeu");
    }
   
}

