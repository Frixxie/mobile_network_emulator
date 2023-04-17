pub struct MobileNetworkCoreEvent {
    kind: EventType,
    description: String,
}

pub struct MobileNetworkCore {
    rans: Vec<Ran>,
    orphans: Vec<User>,
    events: Vec<MobileNetworkCoreEvent>,
    event_subscribers: Vec<Subscriptions>,
}

impl MobileNetworkCore {
    pub fn new(rans: Vec<Ran>, orphans: Vec<User>) -> Self;
    pub fn update_user_positions(&mut self);
    pub fn connect_some_users(&mut self);
    pub fn get_events(&self) -> MobileNetworkCoreEvent;
}

pub struct Ran {
    cell: Rect,
    connected_users: Vec<PDUSession>,
}

impl Ran {
    pub fn new(cell: Rect) -> Self;
    pub fn connect_users(&mut self, users: Vec<PDUSession>);
    pub fn disconnect_users(&mut self) -> Vec<PDUSession>;
}

pub struct PDUSession {
    user: User,
    ip_address: IpAddr,
}

impl PDUSession {
    pub fn new(user: User, ip_address: IpAddr) -> Self;
    pub fn release(&mut self) -> (User, IpAddr);
    pub fn use_application(&self, url: Url);
}

pub struct User {
    id: u32,
    posititon: usize,
    path: Option<MultiPoint>,
}

impl User {
    pub fn new(id: u32) -> Self;
    pub fn add_path(&mut self, path: MultiPoint);
    pub fn current_pos(&self);
    pub fn next_pos(&mut self);
}

pub struct Application {
    url: Url,
}

impl Application {
    pub fn new(url: Url) -> Self;
}

pub struct ApplicationRuntime {
    applications: Vec<(Application, u32)>,
}

impl ApplicationRuntime {
    pub fn new() -> Self;
    pub fn add_application(application: Application);
    pub fn delete_application(application: &Application) -> Application;
    pub fn use_application(application: &Application) -> u32;
}

pub struct EdgeDataCenter {
    application_runtime: ApplicationRuntime,
    name: String,
    posititon: Point,
}

impl EdgeDataCenter {
    pub fn new(name: String, position: Point) -> Self;
    pub fn add_application(application: Application) -> Url;
    pub fn delete_application(application: &Application) -> Application;
    pub fn use_application(application: &Application) -> u32;
}

pub struct Network {
    edge_data_centers: Vec<EdgeDataCenters>,
}
