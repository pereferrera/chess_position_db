#[cfg(test)]

extern crate chess_position_db;

use chess_position_db::parse::*;

mod tests {
    use super::*;

    #[test]
    fn test_parse_database() {
        let res = parse_database("test_dataset.pgn");

        let position_db = res.unwrap();        
        let position_stats = position_db.get(START_POSITION_FEN).unwrap();

        let most_popular_move = &position_stats.played_moves[0]; // d4
        let second_most_popular_move = &position_stats.played_moves[1]; // c4

        assert_eq!(most_popular_move.move_san, "d4");
        assert_eq!(most_popular_move.times_played, 2);
        assert_eq!(most_popular_move.times_black_won, 1);
        assert_eq!(most_popular_move.times_white_won, 0);

        assert_eq!(second_most_popular_move.move_san, "c4");
        assert_eq!(second_most_popular_move.times_played, 1);
        assert_eq!(second_most_popular_move.times_black_won, 0);        
        assert_eq!(second_most_popular_move.times_white_won, 1);

        // follow next moves
        let fen_after_d4 = &most_popular_move.fen_after;
        let position_stats_after_d4 = position_db.get(fen_after_d4).unwrap();
        let most_popular_move_after_d4 = &position_stats_after_d4.played_moves[0];
 
        assert_eq!(most_popular_move_after_d4.move_san, "Nf6");
        assert_eq!(most_popular_move_after_d4.times_played, 2);
        assert_eq!(most_popular_move_after_d4.times_black_won, 1);
        assert_eq!(most_popular_move_after_d4.times_white_won, 0);
    }
}
