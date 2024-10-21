pub struct ServerResponse {
    online: bool,
    players: u32,
    max: u32,
}

impl ServerResponse {
    pub fn new(online: bool, players: u32, max: u32) -> Self {
        ServerResponse {
            online,
            players,
            max,
        }
    }

    pub fn online(&self) -> bool {
        self.online
    }

    pub fn players(&self) -> u32 {
        self.players
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn is_full(&self) -> bool {
        self.players == self.max
    }

    pub fn to_string(&self) -> String {
        format!("{}/{} ({})", self.players, self.max, self.online)
    }
}
