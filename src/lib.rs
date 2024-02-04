use std::str::FromStr;

turbo::cfg! {r#"
    name = "SolHunter redux"
    version = "1.0.0"
    author = "Alfred Okang"
    description = "a game of collecting chests removing enemies"
    [settings]
    resolution = [256, 256]
    [solana]
    http-rpc-url = "https://devnet.helius-rpc.com/?api-key=56de2bc4-02f3-492b-b608-8b970b885691"
    ws-rpc-url = "wss://devnet.helius-rpc.com/?api-key=56de2bc4-02f3-492b-b608-8b970b885691"
"#
}

use turbo::solana;
use solana::{anchor, rpc, solana_sdk};
use solana_sdk::pubkey::Pubkey;
use sol_hunter::program::SolHunter;
use sol_hunter::state::game::GameDataAccount;


turbo::init! {
    // Define the GameState struct.
    struct GameState {
        screen: enum Screen {
            Title,
            Level,
        },
        x_position: i32,
        y_position: i32,
    } = {
        // Set the struct's initial value.
        Self {
            screen: Screen::Title,
            x_position: 30,
            y_position: 40,
        }
    }
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go! {

    // Handle user input todo: send move tx to solana on key press
    if gamepad(0).left.pressed() {
        turbo::println!("left")
    }
    if gamepad(0).right.pressed() {
        turbo::println!("right")
    }
    if gamepad(0).up.pressed() {
        turbo::println!("up")
    }
    if gamepad(0).down.pressed() {
        turbo::println!("down")
    }

    //spacebar
    if gamepad(0).start.pressed() {
        //todo: spawn
        turbo::println!("start")
    }


    //display grid TODO: turn this into a function
    for yindex in 0..4 {
        for xindex in 0..4 {
            rect!(w = 35, h = 35, x = xindex * 38 + 20, y = yindex * 38 + 20, fill = 0xffffffff);
        }
    }
    //get users pubkey
    let user_pubkey = solana::user_pubkey();
    //turbo::println!("user pk: {:?}",user_pubkey);
    text!(&format!("pk: {:?}",user_pubkey));
    //text!("huh");

    //todo: fetch game state from game account
    let program_pubkey = Pubkey::from_str("7sgkU1bPgYrGMGjvw2CwnM9qDLvqnswKSA9hdk7BaZ1w").expect("Error parsing program ID");
    let (board_state_pk, _) = Pubkey::find_program_address(&[b"level105"],&program_pubkey);
    let mut res = rpc::get_account(board_state_pk);
    if !res.is_fetched() {
        // The account isn't fetched yet. Handle the loading state.
    } else if let Some(ref mut account) = res.value {
        // The account is loaded. Deserialize its data.

        //size of each struct is 75 bytes. the first 8 bytes are discriminator.
        //draw player, enemies, and chests on the board 
        for yindex in 0..4 {
            for xindex in 0..4 {
                let element = 15 - (4*yindex + xindex);
                let mut test = &mut account.data[element*75+8..(element+1)*75+8];
                let testpk = Pubkey::new(&test[42..42+32]);
                //turbo::println!("{:?} : pk {:?} : state {:?}",element,testpk,test[32]);
                if test[32] == 1 && testpk == user_pubkey{
                    circ!(d = 35, x = (xindex * 38 + 20).try_into().unwrap(), y = (yindex * 38 + 20).try_into().unwrap(), fill = 0x000000ff);
                }
                else if test[32] == 1 && testpk != user_pubkey{
                    circ!(d = 35, x = (xindex * 38 + 20).try_into().unwrap(), y = (yindex * 38 + 20).try_into().unwrap(), fill = 0xf0000fff);
                }
                else if test[32] == 2 {
                    rect!(w = 35, h = 20, x = (xindex * 38 + 20).try_into().unwrap(), y = (yindex * 38 + 20).try_into().unwrap(), fill = 0xffff0fff);
                }
            }
        }
    } else {
        // The account loaded, but it has no data.
    }

}