#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::ast::{Card, CardGameModel, Player};

    #[derive(Debug)]
    struct Location {
        name: String,
    }
    

    #[test]
    fn test_player_macro() {
        let mut cgm = CardGameModel::new("player_test");

        // Ensure the macro modifies the cgm instance
        player!(cgm, "Jimmy", "Kimmy", "Timmy");

        assert_eq!(cgm.gamedata.players.len(), 3); // Ensure 3 players were added
        assert_eq!(cgm.gamedata.players[0].name, "Jimmy");
        assert_eq!(cgm.gamedata.players[1].name, "Jimmy");
        assert_eq!(cgm.gamedata.players[2].name, "Timmy");
    }

    #[test]
    fn test_team_macro() {
        let mut cgm = CardGameModel::new("team_test");
        player!(cgm, "Jimmy", "Kimmy", "Timmy");
        team! { cgm, "Team1", ("Jimmy", "Kimmy", "Timmy") };
        // Replace the below assertions with actual checks based on how `team!` works.
        // Example:
        assert!(cgm.gamedata.teams[0].teamname == "Team1".to_string());
        assert!((*cgm.gamedata.teams[0].players[0].borrow_mut()).name == "Jimmy".to_string());
    }
    
    #[test]
    fn test_location_on() {
        let mut cgm = CardGameModel::new("location_on_test");
        // Ensure the macro modifies the cgm instance
        player!(cgm, "Jimmy", "Kimmy", "Timmy");
        location_on!(cgm, "hand", players: "Jimmy", "Kimmy");
        assert!(cgm.gamedata.players[0].locations.len() == 1);

        team!(cgm, "t1", ("Kimmy", "Timmy"));
        location_on!(cgm, "teamloc", team: "t1");
        assert!(cgm.gamedata.teams[0].locations.len() == 1);

        location_on!(cgm, "stack", table);
        assert!(cgm.gamedata.table.locations.len() == 1);
    }

    #[test]
    fn test_card_on_macro() {
        let mut cgm = CardGameModel::new("cards_on_test");

        location_on!(cgm, "stack", table);

        card_on!(
            cgm,
            "stack",
            {
                Rank("2", "3", "4", "5", "A"),
                Suite("Diamond", "Hearts"),
                Color("Red")
            },
            {
                Rank("2", "3", "4", "5", "A"),
                Suite("Spades", "Clubs"),
                Color("Black")
            }
        );

        // Test cards
        assert!(cgm.gamedata.table.locations.get("stack").unwrap().contents.len() == 20);
    }

    #[test]
    fn test_precedence_macro() {
        let mut cgm = CardGameModel::new("precedence_test");

        precedence!(cgm, "rank", ("2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"));
        precedence!(cgm, "suite", ("Clubs", "Diamonds", "Hearts", "Spades"));

        // Test rank precedence
        assert!(cgm.gamedata.precedences.get("rank").unwrap().attributes.contains_key(&("rank2".to_string())));
        assert!(cgm.gamedata.precedences.get("rank").unwrap().attributes.contains_key(&("rankA".to_string())));

        // Test suite precedence
        assert!(cgm.gamedata.precedences.get("suite").unwrap().attributes.contains_key(&("suiteClubs".to_string())));
        assert!(cgm.gamedata.precedences.get("suite").unwrap().attributes.contains_key(&("suiteSpades".to_string())));
    }

    #[test]
    fn test_pointmap_macro() {
        let mut cgm = CardGameModel::new("pointmap_test");

        pointmap!(
            cgm,
            "rank",
            nested: {  
                "rank", (
                "2" => [2],
                "3" => [3],
                "4" => [4],
                "5" => [5],
                "6" => [6],
                "7" => [7],
                "8" => [8],
                "9" => [9],
                "10" => [10],
                "J" => [10],
                "Q" => [10],
                "K" => [10],
                "A" => [11, 1]
                )
            },
            list: {  
                ("suite", "clubs") => [100],
            },
        );

        // Test rank points, nested mapping
        assert!(cgm.gamedata.pointmaps["rank"].entries["2"].len() == 1);
        assert_eq!(cgm.gamedata.pointmaps["rank"].entries["2"], vec![2]);

        assert!(cgm.gamedata.pointmaps["rank"].entries["A"].len() == 2);
        assert_eq!(cgm.gamedata.pointmaps["rank"].entries["A"], vec![11, 1]);

        // Test rank points, flat mapping
        assert!(cgm.gamedata.pointmaps["rank"].entries["clubs"].len() == 1);
        assert_eq!(cgm.gamedata.pointmaps["rank"].entries["clubs"], vec![100]);        
    }

    #[test]
    fn test_turn_order_macro() {
        use std::rc::Rc;
        use std::cell::RefCell;

        fn print_players(turnorder: Vec<Rc<RefCell<Player>>>) {
            println!("turnorder:");
            println!("================");
            for p in turnorder {
                println!("{}", p.borrow().name)
            }
            println!("================");
            println!();
        }

        let mut cgm = CardGameModel::new("turn_order_test");
        player!(cgm, "Jimmy", "Kimmy", "Timmy");
        turn_order!(cgm, random);
        print_players(cgm.gamedata.turnorder.clone());
        
        turn_order!(cgm, ("Timmy", "Jimmy", "Kimmy"));
        print_players(cgm.gamedata.turnorder.clone());
    }

    #[test]
    fn test_filter_macro() {
        use crate::ast::Component;
        let mut cgm = CardGameModel::new("filter_test");

        location_on!(cgm, "stack", table);

        card_on!(
            cgm,
            "stack",
            {
                Rank("2", "3", "4", "5", "A"),
                Suite("Diamond", "Hearts"),
                Color("Red")
            },
            {
                Rank("2", "3", "4", "5", "A"),
                Suite("Spades", "Clubs"),
                Color("Black")
            }
        );

        precedence!(cgm, "Rank", ("2", "3", "4", "5", "A"));
        
        let cards: Vec<Card> = cgm.gamedata.table.locations["stack"].contents
            .iter()
            .filter_map(|c| {
                if let Component::CARD(card) = c {
                    Some(card.clone()) // Return the cloned card
                } else {
                    None // Filter out non-card components
                }
            })
            .collect();
        
        // for c in cards.iter() {
        //     println!("{}", c);
        // }

        // Filter for "same" rank
        let same_filter = filter!("Rank", "same");
        let filtered_cards = same_filter(cards.clone());
        println!("{}", filtered_cards.len());
        for f in filtered_cards.iter() {
            println!("Same rank cards: {:?}", f);
        }

        // TODO: I dont know how to implement
        // More precisely I dont understand what it means
 
        // Filter for "distinct"
        // let distinct_filter = filter!("Rank", "distinct");
        // let filtered_cards = distinct_filter(cards.clone());
        // println!("{}", filtered_cards.len());
        // for f in filtered_cards.iter() {
        //     println!("Distinct rank cards: {:?}", f);
        // }
        
        // Filter for "adjacent" using precedence
        let adjacent_filter = filter!("Rank",
            "adjacent" using cgm.gamedata.precedences["Rank"].attributes);
        let filtered_cards = adjacent_filter(cards.clone());
        println!("Adjacent rank cards: {:?}\n", filtered_cards);


        // Higher and Lower makes 0 sense
        // Filter for "higher" using precedence ("higher" is interpreted as "highest")
        // let higher_filter = filter!("rank", "higher" using cgm.gamedata.precedences["Rank"]);
        // let filtered_cards = higher_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Higher rank cards: {:?}", c);
        // }

        // Filter for "lower" using precedence ("lower" is interpreted as "lowest")
        // let lower_filter = filter!("rank", "lower" using cgm.gamedata.precedences["Rank"]);
        // let filtered_cards = lower_filter(cards.clone());
        // for c in filtered_cards.iter() {
        //     println!("Lower rank cards: {:?}", c);
        // }

        // Filter with Key == Value
        let bool_filter = filter!("Rank", "==", "3");
        let filtered_cards = bool_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("Equal rank cards: {:?}", c);
        }
    
        // Filter by size
        let size_filter = filter!(size, "==", 3);
        let filtered_cards = size_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("size cards == 3: {:?}", c);
        }

        // Filter by size
        let size_filter = filter!(size, ">", 3);
        let filtered_cards = size_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("size cards > 3: {:?}", c);
        }

        // Filter with Key != Value
        let bool_filter = filter!("Rank", "!=", "3");
        let filtered_cards = bool_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("Not-Equal rank cards: {:?}", c);
        }

        // Combined filter
        let combined_filter = filter!(
            ("Rank", "adjacent" using cgm.gamedata.precedences["Rank"].attributes), 
            ("and"), 
            ("Suite", "same")
        );        
        let filtered_cards = combined_filter(cards.clone());
        for c in filtered_cards.iter() {
            println!("Combined-Filter (rank-adjacent, suite same): {:?}", c);
        }

        let combined_filter = filter!(
            ("Rank", "adjacent" using cgm.gamedata.precedences["Rank"].attributes),
            ("and"),
            (size, ">=", 3)
        );        
        let filtered_cards = combined_filter(cards.clone());
        for c in filtered_cards {
            println!("Combined-Filter (adjacent, size >= 3): {:?}", c);
        }

        // Testing more nested filter functions
        let combined_filter = filter!(
            ("Suite", "same"),
            ("and"),
            (size, ">=", 3)
        );        
        let filtered_cards = combined_filter(cards.clone());
        println!("Combined-Filter (Suite same, size != 3): {:?}", filtered_cards);

        // // Testing more nested filter functions
        // // OR DOESNT WORK YET
        // let combined_filter = filter!(
        //     ("suite", "same"),
        //     ("or"),
        //     ("rank", "same")
        // );        
        // let filtered_cards = combined_filter(cards.clone());
        // println!("Combined-Filter (suite same, rank same): {:?}", filtered_cards);

    }
}
