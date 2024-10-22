pub struct ServerResponse {
    online: bool,
    players: Option<u32>,
    max: Option<u32>,
}

impl ServerResponse {
    pub fn new(online: bool, players: Option<u32>, max: Option<u32>) -> Self {
        ServerResponse {
            online,
            players,
            max,
        }
    }

    pub fn online(&self) -> bool {
        self.online
    }

    pub fn players(&self) -> Option<u32> {
        self.players
    }

    pub fn max(&self) -> Option<u32> {
        self.max
    }

    pub fn is_full(&self) -> bool {
        self.players >= self.max
    }

    pub fn to_string(&self) -> String {
        if let (Some(players), Some(max)) = (self.players, self.max) {
            format!("{}/{} ({})", players, max, self.online)
        } else {
            format!("N/A ({})", self.online)
        }
    }
}
