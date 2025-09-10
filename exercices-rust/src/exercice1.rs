

/// Représente un morceau de musique avec une note et un titre.
pub struct Song {
    rank: u32,
    title: String,
}

/// Retourne la note moyenne d'un ensemble de morceaux.
pub fn average_rank(songs: &Vec<Song>) -> f64 {
    
    if songs.is_empty() {
        panic!("Not implemented!");
    }

    let mut somme = 0;
    for s in songs {
        somme += s.rank;
    }

    somme as f64 / songs.len() as f64

}

/// Filtre les morceaux dans `songs` et ne garde que ceux dont la note est
/// strictement supérieure à `rank_min`. -> Vec<Song>
pub fn filter_songs(songs: Vec<Song>, rank_min: u32) -> Vec<Song> {
    
    if songs.is_empty() {
        panic!("Not implemented!");
    }

    let mut filtered_songs: Vec<Song> = Vec::new();

    for s in songs{

        if s.rank > rank_min {
            println!("Rank: {} est supérieur à rank min", s.rank);
            filtered_songs.push(s);
        } else {
            println!("Le morceau n'est pas supérieur a rank min");
        }
        
    }
    return filtered_songs;
}

/// Filtre les morceaux dans `songs` pour ne conserver que ceux dont la note
/// est strictement supérieure à la moyenne.
pub fn good_songs(songs: Vec<Song>) -> Vec<Song> {
    
    if songs.is_empty() {
        panic!("Not implemented!");
    }

    let mut filtered_songs: Vec<Song> = Vec::new();
    let moyen_song: f64 = average_rank(&songs);

    for s in songs{

        if s.rank as f64 > moyen_song {
            println!("Rank: {} est supérieur à average_rank", s.rank);
            filtered_songs.push(s);
        } else {
            println!("Le morceau n'est pas supérieur a average_rank");
        }
        
    }
    return filtered_songs;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<Song> {
        vec![
            Song {
                rank: 4,
                title: String::from("Stairway to Heaven"),
            },
            Song {
                rank: 2,
                title: String::from("Never Gonna Give You Up"),
            },
            Song {
                rank: 5,
                title: String::from("Nigerian Marketplace"),
            },
        ]
    }

    #[test]
    #[should_panic]
    fn empty() {
        let sgs = Vec::new();
        let _m = average_rank(&sgs);
    }

    #[test]
    fn moyenne_un_seul_morceau() {
        let sgs = vec![Song {
            rank: 4,
            title: String::from("Stairway to Heaven"),
        }];
        let m = average_rank(&sgs);
        assert_eq!(m, 4.0);
        println!("La moyenne est {}", m);
    }

    #[test]
    fn moyenne_plusieurs_morceau() {
        let sgs = example();
        let m = average_rank(&sgs);
        assert!((m - 3.66666).abs() < 0.001);
    }

    #[test]
    fn filtre() {
        let sgs = example();
        let f = filter_songs(sgs, 4);
        assert!(f.len() == 1);
        assert_eq!(f[0].rank, 5);
        assert_eq!(f[0].title, "Nigerian Marketplace");
    }

    #[test]
    fn meilleurs() {
        let sgs = example();
        let f = good_songs(sgs);
        assert!(f.len() == 2);
        for m in &f {
            assert!(m.rank >= 4);
        }
    }
}
