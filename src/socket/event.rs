pub enum CSGOEmpireSocketEvent {
    Init,
    Timesync,
    NewItem,
    UpdateItem,
    AuctionUpdate,
    DeletedItem,
    TradeStatus,
    Disconnect,
    Error,
}

impl From<&CSGOEmpireSocketEvent> for &str {
    fn from(event: &CSGOEmpireSocketEvent) -> Self {
        match event {
            CSGOEmpireSocketEvent::Init => "init",
            CSGOEmpireSocketEvent::Timesync => "timesync",
            CSGOEmpireSocketEvent::NewItem => "new_item",
            CSGOEmpireSocketEvent::UpdateItem => "update_item",
            CSGOEmpireSocketEvent::AuctionUpdate => "auction_update",
            CSGOEmpireSocketEvent::DeletedItem => "deleted_item",
            CSGOEmpireSocketEvent::TradeStatus => "trade_status",
            CSGOEmpireSocketEvent::Disconnect => "close",
            CSGOEmpireSocketEvent::Error => "err",
        }
    }
}

impl From<CSGOEmpireSocketEvent> for &str {
    fn from(event: CSGOEmpireSocketEvent) -> Self {
        (&event).into()
    }
}

impl From<&CSGOEmpireSocketEvent> for String {
    fn from(event: &CSGOEmpireSocketEvent) -> Self {
        event.into()
    }
}

impl ToString for CSGOEmpireSocketEvent {
    fn to_string(&self) -> String {
        self.into()
    }
}
