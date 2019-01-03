use actix::{Context, Handler, Message};
use protocol::composer::player::{credits_composer, messenger::messenger_config_composer, player_info_composer, points_balance_composer, achievement_points_composer};
use session::ServerSession;


#[derive(Message)]
pub struct InfoRetrieve;

impl Handler<InfoRetrieve> for ServerSession {
    type Result = ();

    fn handle(&mut self, _: InfoRetrieve, _: &mut Context<Self>) {
        let p = self.player().clone();
        let player = p.read().unwrap();

        let _ = self.compose_all(vec![
            credits_composer(player.balance.credits),
            messenger_config_composer(),
            points_balance_composer(&player.balance),
            achievement_points_composer(player.achievement_points),
            player_info_composer(&player.avatar)
        ]);
    }
}
