use crate::{
    api::{player::Player, world_size::WorldSize},
    engine::{
        game::Game,
        outcome::{ChampionshipOutcome, GameOutcome},
    },
    players::{
        alvarez::Luis, armholt::Swede, arola::Arola, fox::TwentyCenturyFox, karjalainen::Miklas,
        laurikainen::PlayerOne, moykkynen::Joonas, niemisto::Niemisto, pop::Aurelian, rahtu::Rahtu,
        rantala::PlayerTeemu, reponen::Samuli, salonen::Es, siimesjarvi::Siimesjarvi,
        terava::PlAgiAntti,
    },
};

pub enum League {
    Academy,
    Open,
}

pub struct Championship {
    league: League,
    world_size: WorldSize,
}

impl Championship {
    pub fn new(league: League, world_size: WorldSize) -> Self {
        Championship { league, world_size }
    }

    pub fn run(&mut self, rounds: u32) -> ChampionshipOutcome {
        let mut championship_outcome = ChampionshipOutcome::new();
        let players = self.get_players();

        let mut player_id = 0;
        for player in players {
            if player.is_ready() {
                player_id += 1;
                championship_outcome.register_player(player_id, player.name());
            }
        }

        for i in 0..rounds {
            let game_id = i + 1;
            let players = self.get_players();
            let (quit, game_outcome) =
                self.run_single_game(game_id, players, self.world_size.clone());

            championship_outcome.add_game_result(game_outcome);
            println!("Game {game_id} finished");

            if quit {
                break;
            }
        }
        championship_outcome.compute_ranks();

        championship_outcome
    }
}

impl Championship {
    fn run_single_game(
        &self,
        game_id: u32,
        players: Vec<Box<dyn Player>>,
        world_size: WorldSize,
    ) -> (bool, GameOutcome) {
        let mut game = Game::new(world_size);
        game.spawn_players(players);

        game.start(game_id)
    }

    fn get_players(&self) -> Vec<Box<dyn Player>> {
        let mut result: Vec<Box<dyn Player>> = vec![
            Box::new(Luis::new()),
            Box::new(Swede::new()),
            Box::new(Arola::new()),
            Box::new(PlayerOne::new()),
            Box::new(Joonas::new()),
            Box::new(Niemisto::new()),
            Box::new(Rahtu::new()),
            Box::new(PlayerTeemu::new()),
            Box::new(Samuli::new()),
            Box::new(Es::new()),
            Box::new(Siimesjarvi::new()),
            Box::new(PlAgiAntti::new()),
            Box::new(TwentyCenturyFox::new()),
        ];

        let mut open_players: Vec<Box<dyn Player>> =
            vec![Box::new(Aurelian::new()), Box::new(Miklas::new())];

        match self.league {
            League::Academy => {}
            League::Open => result.append(&mut open_players),
        }

        result
    }
}
