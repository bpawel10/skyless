use super::definitions::Client;
use crate::prelude::Player;
use skyless_core::prelude::*;

#[attribute]
pub struct Clients(pub HashMap<Player, Client>);
