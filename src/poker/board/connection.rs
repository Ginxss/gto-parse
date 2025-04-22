use crate::poker::{card::Card, rank::RankHeight, ParseError};

use super::Board;

pub enum BoardConnection {
    Disconnected,
    Gutshot,
    OESD,
    Wheel,
    NormalStraight,
    AnyStraight,
}

impl TryFrom<&str> for BoardConnection {
    type Error = ParseError;

    fn try_from(s: &str) -> Result<BoardConnection, ParseError> {
        match s {
            "DC" => Ok(BoardConnection::Disconnected),
            "GS" => Ok(BoardConnection::Gutshot),
            "OESD" => Ok(BoardConnection::OESD),
            "WH" => Ok(BoardConnection::Wheel),
            "NS" => Ok(BoardConnection::NormalStraight),
            "AS" => Ok(BoardConnection::AnyStraight),
            _ => Err(ParseError::str("connection", s)),
        }
    }
}

impl Board {
    pub fn is_normal_straight_possible(&self) -> bool {
        self.get_highest_card() - self.get_lowest_card() <= 4
    }

    pub fn is_wheel_possible(&self) -> bool {
        self.num_rank_height(&RankHeight::Wheel) == 3
    }

    pub fn is_any_straight_possible(&self) -> bool {
        self.is_normal_straight_possible() && self.is_wheel_possible()
    }

    pub fn is_only_oesd_possible(&self) -> bool {
        if self.is_any_straight_possible() {
            return false;
        }

        Card::get_distances(self.cards.iter().filter(|card| !card.is_ace()))
            .into_iter()
            .any(|diff| diff <= 3)
    }

    pub fn is_only_gutshot_possible(&self) -> bool {
        let distances = Card::get_distances(self.cards.iter());
        let min_distance = *distances.iter().min().unwrap();

        if min_distance == 4 {
            true
        } else if min_distance < 4 {
            false
        } else {
            self.get_highest_card().is_ace()
        }
    }

    pub fn is_disconnected(&self) -> bool {
        if self.get_highest_card().is_ace() {
            return false;
        }

        let distances = Card::get_distances(self.cards.iter());
        let min_distance = distances.iter().min().unwrap();
        if *min_distance <= 4 {
            return false;
        }

        true
    }

    pub fn is_connection(&self, connection: &BoardConnection) -> bool {
        match connection {
            BoardConnection::Disconnected => self.is_disconnected(),
            BoardConnection::Gutshot => self.is_only_gutshot_possible(),
            BoardConnection::OESD => self.is_only_oesd_possible(),
            BoardConnection::Wheel => self.is_wheel_possible(),
            BoardConnection::NormalStraight => self.is_normal_straight_possible(),
            BoardConnection::AnyStraight => self.is_any_straight_possible(),
        }
    }

    fn get_lowest_card(&self) -> &Card {
        self.cards.first().unwrap()
    }

    fn get_highest_card(&self) -> &Card {
        self.cards.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::poker::rank::Rank;

    use super::*;

    #[test]
    fn test_is_normal_straight_possible() {
        assert!(Board::try_from("Ts9c8h")
            .unwrap()
            .is_normal_straight_possible());
        assert!(Board::try_from("7s9sTh")
            .unwrap()
            .is_normal_straight_possible());
        assert!(Board::try_from("Ts9s6s")
            .unwrap()
            .is_normal_straight_possible());
        assert!(Board::try_from("AcKsTh")
            .unwrap()
            .is_normal_straight_possible());
        assert!(!Board::try_from("As2c3h")
            .unwrap()
            .is_normal_straight_possible());
        assert!(!Board::try_from("7s9c7h")
            .unwrap()
            .is_normal_straight_possible());
        assert!(!Board::try_from("Ts9c5h")
            .unwrap()
            .is_normal_straight_possible());
        assert!(!Board::try_from("Ks2c3h")
            .unwrap()
            .is_normal_straight_possible());
        assert!(!Board::try_from("AcKs3h")
            .unwrap()
            .is_normal_straight_possible());
        assert!(!Board::try_from("KhTsTc")
            .unwrap()
            .is_normal_straight_possible());
    }

    #[test]
    fn test_is_wheel_possible() {
        assert!(Board::try_from("As2c3h").unwrap().is_wheel_possible());
        assert!(Board::try_from("5h3cAc").unwrap().is_wheel_possible());
        assert!(Board::try_from("2dAd4d").unwrap().is_wheel_possible());
        assert!(!Board::try_from("Ts9c8h").unwrap().is_wheel_possible());
        assert!(!Board::try_from("Ts9s6s").unwrap().is_wheel_possible());
        assert!(!Board::try_from("AcKsTh").unwrap().is_wheel_possible());
        assert!(!Board::try_from("7s9c7h").unwrap().is_wheel_possible());
        assert!(!Board::try_from("Ks2c3h").unwrap().is_wheel_possible());
        assert!(!Board::try_from("AcKs3h").unwrap().is_wheel_possible());
        assert!(!Board::try_from("AhTsTc").unwrap().is_wheel_possible());
    }

    #[test]
    fn test_is_any_straight_possible() {
        assert!(Board::try_from("Ts9c8h")
            .unwrap()
            .is_any_straight_possible());
        assert!(Board::try_from("7s9sTh")
            .unwrap()
            .is_any_straight_possible());
        assert!(Board::try_from("Ts9s6s")
            .unwrap()
            .is_any_straight_possible());
        assert!(Board::try_from("AcKsTh")
            .unwrap()
            .is_any_straight_possible());
        assert!(Board::try_from("As2c3h").unwrap().is_wheel_possible());
        assert!(Board::try_from("5h3cAc").unwrap().is_wheel_possible());
        assert!(Board::try_from("2dAd4d").unwrap().is_wheel_possible());
        assert!(!Board::try_from("7s9c7h")
            .unwrap()
            .is_any_straight_possible());
        assert!(!Board::try_from("Ts9c5h")
            .unwrap()
            .is_any_straight_possible());
        assert!(!Board::try_from("Ks2c3h")
            .unwrap()
            .is_any_straight_possible());
        assert!(!Board::try_from("AcKs3h")
            .unwrap()
            .is_any_straight_possible());
        assert!(!Board::try_from("KhTsTc")
            .unwrap()
            .is_any_straight_possible());
    }

    #[test]
    fn test_is_only_oesd_possible() {
        assert!(Board::try_from("Ts9c5h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("Jh9h2h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("Qh9h2h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("Jd6c3c").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("Ks2c3h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("As5c6h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("As7c4h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("AsTc7h").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("KsThTc").unwrap().is_only_oesd_possible());
        assert!(Board::try_from("AcJh8d").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("AsTc6h").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("As2c3h").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("As8c8h").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("AsJc7h").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("AsKc8h").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("Ks8h3c").unwrap().is_only_oesd_possible());
        assert!(!Board::try_from("Ks8h8c").unwrap().is_only_oesd_possible());
    }

    #[test]
    fn test_is_only_gutshot_possible() {
        assert!(Board::try_from("Ks8c4h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(Board::try_from("Ks9c4h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(Board::try_from("Ac2h6h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(Board::try_from("Ac9h5h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(Board::try_from("AcKh6h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(Board::try_from("Ac9h4h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(!Board::try_from("AcKhTh")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(!Board::try_from("Ks8c5h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(!Board::try_from("KsJc4h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(!Board::try_from("Ac2h4h")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(!Board::try_from("Ac5h6s")
            .unwrap()
            .is_only_gutshot_possible());
        assert!(!Board::try_from("7c5c5s")
            .unwrap()
            .is_only_gutshot_possible());
    }

    #[test]
    fn test_is_disconnected() {
        assert!(Board::try_from("Kh8h2h").unwrap().is_disconnected());
        assert!(Board::try_from("Kh7c2c").unwrap().is_disconnected());
        assert!(Board::try_from("Qh7c2c").unwrap().is_disconnected());
        assert!(!Board::try_from("Kh6c2d").unwrap().is_disconnected());
        assert!(!Board::try_from("Ah6c9d").unwrap().is_disconnected());
        assert!(!Board::try_from("Kh3c9d").unwrap().is_disconnected());
    }

    #[test]
    fn test_is_connection_1() {
        let disconnected_board = Board::try_from("Kh8h2h").unwrap();
        assert_eq!(
            disconnected_board.is_connection(&BoardConnection::Disconnected),
            true
        );
        assert_eq!(
            disconnected_board.is_connection(&BoardConnection::Gutshot),
            false
        );
        assert_eq!(
            disconnected_board.is_connection(&BoardConnection::OESD),
            false
        );
        assert_eq!(
            disconnected_board.is_connection(&BoardConnection::Wheel),
            false
        );
        assert_eq!(
            disconnected_board.is_connection(&BoardConnection::NormalStraight),
            false
        );
        assert_eq!(
            disconnected_board.is_connection(&BoardConnection::AnyStraight),
            false
        );
    }

    #[test]
    fn test_is_connection_2() {
        let gs_board = Board::try_from("Kh9h2h").unwrap();
        assert_eq!(
            gs_board.is_connection(&BoardConnection::Disconnected),
            false
        );
        assert_eq!(gs_board.is_connection(&BoardConnection::Gutshot), true);
        assert_eq!(gs_board.is_connection(&BoardConnection::OESD), false);
        assert_eq!(gs_board.is_connection(&BoardConnection::Wheel), false);
        assert_eq!(
            gs_board.is_connection(&BoardConnection::NormalStraight),
            false
        );
        assert_eq!(gs_board.is_connection(&BoardConnection::AnyStraight), false);
    }

    #[test]
    fn test_is_connection_3() {
        let oesd_board = Board::try_from("KhTh2c").unwrap();
        assert_eq!(
            oesd_board.is_connection(&BoardConnection::Disconnected),
            false
        );
        assert_eq!(oesd_board.is_connection(&BoardConnection::Gutshot), false);
        assert_eq!(oesd_board.is_connection(&BoardConnection::OESD), true);
        assert_eq!(oesd_board.is_connection(&BoardConnection::Wheel), false);
        assert_eq!(
            oesd_board.is_connection(&BoardConnection::NormalStraight),
            false
        );
        assert_eq!(
            oesd_board.is_connection(&BoardConnection::AnyStraight),
            false
        );
    }

    #[test]
    fn test_is_connection_4() {
        let wheel_board = Board::try_from("KhTh2c").unwrap();
        assert_eq!(
            wheel_board.is_connection(&BoardConnection::Disconnected),
            false
        );
        assert_eq!(wheel_board.is_connection(&BoardConnection::Gutshot), false);
        assert_eq!(wheel_board.is_connection(&BoardConnection::OESD), false);
        assert_eq!(wheel_board.is_connection(&BoardConnection::Wheel), true);
        assert_eq!(
            wheel_board.is_connection(&BoardConnection::NormalStraight),
            false
        );
        assert_eq!(
            wheel_board.is_connection(&BoardConnection::AnyStraight),
            true
        );
    }

    #[test]
    fn test_is_connection_5() {
        let ns_board = Board::try_from("KhTs9c").unwrap();
        assert_eq!(
            ns_board.is_connection(&BoardConnection::Disconnected),
            false
        );
        assert_eq!(ns_board.is_connection(&BoardConnection::Gutshot), false);
        assert_eq!(ns_board.is_connection(&BoardConnection::OESD), false);
        assert_eq!(ns_board.is_connection(&BoardConnection::Wheel), false);
        assert_eq!(
            ns_board.is_connection(&BoardConnection::NormalStraight),
            true
        );
        assert_eq!(ns_board.is_connection(&BoardConnection::AnyStraight), true);
    }

    #[test]
    fn test_get_lowest_card() {
        assert_eq!(
            Board::try_from("Kh8h2h").unwrap().get_lowest_card(),
            &Card::try_from("2h").unwrap()
        );
        assert_eq!(
            Board::try_from("AsKhTs").unwrap().get_lowest_card(),
            &Card::try_from("Ts").unwrap()
        );

        let board = Board::try_from("Kh8h8c").unwrap();
        assert!(board.get_lowest_card().rank == Rank::try_from('8').unwrap());
    }

    #[test]
    fn test_get_highest_card() {
        assert_eq!(
            Board::try_from("Kh8h2h").unwrap().get_highest_card(),
            &Card::try_from("Kh").unwrap()
        );
        assert_eq!(
            Board::try_from("AsKhTs").unwrap().get_highest_card(),
            &Card::try_from("As").unwrap()
        );

        let board = Board::try_from("KhKs8c").unwrap();
        assert!(board.get_highest_card().rank == Rank::try_from('K').unwrap());
    }
}
