pub struct Container {
    pub name: String,
    pub id: String,
    pub status: Status,
}

pub enum Status {
    Active {
        display: String
    },
    Restarting {
        display: String
    },
    Looping {
        display: String
    },
    Inactive {
        display: String
    }
}
