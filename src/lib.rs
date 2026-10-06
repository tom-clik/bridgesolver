// wasm_bridge_solver.rs
// Negamax double-dummy solver with TT, pruning, killer/history heuristics, and endgame solver
// + Unit tests and browser JS example

use wasm_bindgen::prelude::*;

// card is a single number from 0-52. 
// A hand is a bit mask indicating which cards we have with a 1 e.g. 1001000000000... = 2 and 5 of spades
#[derive(Clone, Copy, Debug)]
struct Deal {
    hands: [u64; 4],
    trump: Option<usize>,// 0=S,1=H,2=D,3=C,None=no trump
}

#[wasm_bindgen]
pub struct Solver {}

fn suit_of(card: u8) -> usize { (card as usize) / 13 }
fn rank_of(card: u8) -> usize { (card as usize) % 13 }
fn side_of(player: usize) -> usize { player % 2 }

#[wasm_bindgen]
impl Solver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Solver {
        Solver { }
    }

    #[wasm_bindgen]
    pub fn solve_max_tricks(&mut self, hands_flat: Vec<u8>, trump: i32, declarer: usize) -> i32 {
        let mut hands = [0u64; 4];
        for p in 0..4usize {
            for c in 0..52usize {
                if hands_flat[p*52 + c] != 0 {
                    hands[p] |= 1u64 << c;
                }
            }
        }
        let deal = Deal { hands, trump: if trump >= 0 { Some(trump as usize) } else { None } };
        let pt = PartialTrick::new(leader);
        let played_mask = 0u64;
        let tricks = self.search(&deal, played_mask, pt, declarer, NEG_INF, POS_INF);
        tricks as i32
    }

    
}

// === Unit Tests ===
#[cfg(test)]
mod tests {
    use super::*;

    fn make_hand(cards: &[u8]) -> u64 {
        let mut mask = 0u64;
        for &c in cards {
            mask |= 1u64 << c;
        }
        mask
    }

    pub fn print_hand(hand: u64) {
        // Suits in order: Spades, Hearts, Diamonds, Clubs
        let suits = ["♠", "♥", "♦", "♣"];
        // Ranks in order: 2..A
        let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];

        // Loop over all 52 cards
        for suit in 0..4 {
            print!("{} ", suits[suit]);
            for card in (0..=12).rev() {
                let card_index = suit * 13 + card;
                if (hand & (1 << card_index)) != 0 {
                    print!("{} ", ranks[card]);
                }
            }
            println!(); // newline at the end
        }
        println!();
    }

    #[test]
    fn test_single_trick() {
        let mut solver = Solver::new();
        // Simple one-card per player situation: highest spade wins.
        let hands = [make_hand(&[12]), make_hand(&[11]), make_hand(&[10]), make_hand(&[9])];
        for &hand in hands.iter() {
            print_hand(hand);    
        }
        
        let deal = Deal { hands, trump: None };
        let pt = PartialTrick::new(0);

        let tricks = solver.search(&deal, 0, pt, 0, -120, 120);
        println!("Debug: tricks = {}", tricks); // This will now appear in the terminal
        assert_eq!(tricks, 1);
    }

    // #[test]
    // fn test_simple_no_trump() {
    //     let mut solver = Solver::new();
    //     // No trumps, declarer side (0,2) can only win one trick.
    //     let mut hands_flat = vec![0u8; 208];
    //     hands_flat[0*52 + 12] = 1; // A♠
    //     hands_flat[1*52 + 11] = 1; // K♠
    //     hands_flat[2*52 + 10] = 1; // Q♠
    //     hands_flat[3*52 + 9]  = 1; // J♠
    //     let tricks = solver.solve_max_tricks(hands_flat, -1, 0, 0);
    //     assert_eq!(tricks, 1);
    // }

    // #[test]
    // fn test_trump_win() {
    //     let mut solver = Solver::new();
    //     // Player 0 has a low heart, but hearts are trump. Should win at least one trick.
    //     let mut hands_flat = vec![0u8; 208];
    //     hands_flat[0*52 + 14] = 1; // 2♥ (trump)
    //     hands_flat[1*52 + 12] = 1; // A♠
    //     hands_flat[2*52 + 11] = 1; // K♠
    //     hands_flat[3*52 + 10] = 1; // Q♠
    //     let tricks = solver.solve_max_tricks(hands_flat, 1, 0, 0);
    //     assert!(tricks >= 1);
    // }
}
