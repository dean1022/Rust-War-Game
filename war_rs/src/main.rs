#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Documentation for War.

    This program does not work as a way to convert the vector to array could not be found. Therefore, results could not be checked.

    Functions:

    deal(cards)
    cards: the deck of cards probided through the test cases that is shuffled already.
    global: a hashmap that contains the following: deck-deck provided with no alterations; p1 -player one's deck; p2 -player two's deck; rArray -the list of cards for each round that the winner will put at the bottom of their deck; winDeck -the winner's deck.
    This function initializes the variables and calls the function 'startGame' to start the game. It also deals the cards (26 cards for each player). Converts the Aces(1s) to 14s for easier ranking later and reverts it back to 1s when returning the winning deck.

    startGame (global)
    global: hashmap from the function 'deal'.
    This function is where the main code happens after dealing the cards in the deal function. Whether the play wins or if war happens or if its a tie, this function is where the helper functions are called in different scenarios.

    addToArray (global)
    global: the hashmap from the function 'deal'.
    the variable rArray in the keyword list contains the cards each player wins in each turn and sorts them in order before placing back into the winning player's deck.

    empty (global)
    global: the hashmap from the function 'deal'.
    This function first checks if the players' decks are both empty, if so, then it is declared  a tie and the tie deck is returned as the winning deck. It also checks the the winner of each round.

    p1Won (global)
    global: the hashmap from the function 'deal'.
    If player 1 won, meaning player 2 has no more cards, their deck is declared as the winning deck as returns this deck.

    p2Won (global)
    global: the hashmap from the function 'deal'
    If player 2 won, meaning player 1 has no more cards, their deck is declared as the winning deck as returns this deck.
*/

fn deal(shuf: &[u8; 52]) -> [u8; 52]
{
    use std::collections::HashMap;
    let mut global: HashMap<String, Vec<u8>> = HashMap::new();
    let mut p1: Vec<u8> = Vec::new();
    let mut p2: Vec<u8> = Vec::new();
    let mut rArray: Vec<u8> = Vec::new();
    let mut winDeck: Vec<u8> = Vec::new();
    //reverse and deal the cards
    let mut a = shuf.clone();
    let mut b = a.reverse();
    let mut d = b.to_vec;
    for (pos, item) in d.iter().enumerate() {
      if pos % 2 == 0 {
        p1.push(item);
      } else {
        p2.push(item);
      }
    }
    //replace 1s with 14s in player 1's deck
    for (pos, item) in p1.iter().enumerate() {
      if p1[pos] as u8 == 1 as u8 {
        p1[pos] = 14;
      }
    }
  //replace 1s with 14s in player 2's deck
    for (pos, item) in p2.iter().enumerate() {
      if p2[pos] as u8 == 1 as u8 {
        p2[pos] = 14;
      }
    }
  //update the variables that hold the decks
    global.insert(String::from("deck"), d);
    global.insert(String::from("p1"), p1);
    global.insert(String::from("p2"), p2);
    global.insert(String::from("rArray"), rArray);
    global.insert(String::from("winDeck"), winDeck);
    
    let mut map = startGame(global);

    //get the winning deck and revert the 14s back to 1s and return the deck
    let mut result = map.get(&"winDeck");
    for (pos, item) in result.iter().enumerate() {
      if item == 14 {
        result[pos] = 1;
      }
    }
    //could not find a way to convert vector to array
    result
}

fn startGame(g: HashMap<String, Int>) -> HashMap<String, Int> 
{
    let mut global = g.clone();
    let mut array = global.get(&"rArray");
    let mut p1list = global.get(&"p1");
    let mut p2list = global.get(&"p2");

    //check if any players' decks are empty and if so, call the function called empty
    if p1list.is_empty() || p2list.is_empty() {
      empty(global)
    }
    if !(array.is_empty()) {
      let mut k = addToArray(global);
      global = k;
    }
    if p1list.is_empty() || p2list.is_empty() {
      empty(global)
    }
    //check if th top cards of each player is a tie, if so, they will start war
    if p1list[0] == p2list[0] {
      let mut k = addToArray(global);
      startGame(k)
    }
    //checks if player 1's top card is higher than player 2's card, if so, player 1 takes the cards and places them under their own deck in decreasing order
    if p1list[0] > p2list[0] {
      let mut k = p1Won(global);
      startGame(k)
    }
    //checks if player 2's top card is higher than player 1's card, if so, player 2 takes the cards and places them under their own deck in decreasing order
    if p1list[0] < p2list[0] {
      let mut k = p1Won(global);
      startGame(k)
    }

    //update the decks
    global["rArray"] = array;
    global["p1"] = p1list;
    global["p2"] = p2list;
    global
}

fn addToArray(g: HashMap<String, Int>) -> HashMap<String, Int> 
{
    let mut global = g.clone();
    let mut array = global.get(&"rArray");
    let mut p1list = global.get(&"p1");
    let mut p2list = global.get(&"p2");

    array.push(p1list[0]);
    array.push(p2list[0]);
    p1list.remove(0);
    p2list.remove(0);

    //update the decks
    global["rArray"] = array;
    global["p1"] = p1list;
    global["p2"] = p2list;
    global
}

fn empty(g: HashMap<String, Int>) -> HashMap<String, Int> 
{
    let mut global = g.clone();
    let mut array = global.get(&"rArray");
    let mut p1list = global.get(&"p1");
    let mut p2list = global.get(&"p2");
    let mut windeck = global.get(&"winDeck");
    //if both players' decks are empty, then the game is a tie,. the tie deck is returned
    if p1list.is_empty() && p2list.is_empty() {
      windeck.push(array.clone());
      global["winDeck"] = windeck;
      global
    }
    //if only player 2's deck is empty, player 1's deck is declared the winner after making sure all the cards from the last round or war session is also added to the winner's deck
    if p2list.is_empty() {
      windeck = p1list.clone();
      windeck.push(array.clone());
      global["winDeck"] = windeck;
      global
    }
//If only player 1's deck is empty, player 2's deck is declared the winner's deck after making sure all cards from the last round or war session is also added to the winner's deck.
    if p1list.is_empty() {
      windeck = p2list.clone();
      windeck.push(array.clone());
      global["winDeck"] = windeck;
      global
    }

    //update the decks
    global["rArray"] = array;
    global["p1"] = p1list;
    global["p2"] = p2list;
    global["winDeck"] = windeck;
    global
}

fn p1Won(g: HashMap<String, Int>) -> HashMap<String, Int> 
{
    let mut global = g.clone();
    let mut p1list = global.get(&"p1");
    //Adds the cards player 1 won for the round or war session into their deck. The array used for the rounds are emptied after
    let mut temp = addToArray(global);
    let mut array = temp.get(&"rArray");
    p1list.push(array.clone());
    array.clear();

    //update the decks
    global["rArray"] = array;
    global["p1"] = p1list;
    global
}

fn p2Won(g: HashMap<String, Int>) -> HashMap<String, Int> 
{
    let mut global = g.clone();
    let mut p2list = global.get(&"p2");
    //Adds the cards player 2 won for the round or war session into their deck. The array used for the rounds are emptied after
    let mut temp = addToArray(global);
    let mut array = temp.get(&"rArray");
    p2list.push(array.clone());
    array.clear();

    //update the decks
    global["rArray"] = array;
    global["p2"] = p2list;
    global
}



#[cfg(test)]
#[path = "tests.rs"]
mod tests;

