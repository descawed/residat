use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const SAT_TRIGGER_BY_PLAYER: u8 = 0x01;
pub const SAT_TRIGGER_BY_NPC: u8 = 0x02;
pub const SAT_TRIGGER_BY_OBJECT: u8 = 0x04;
pub const SAT_TRIGGER_BY_ALLY: u8 = 0x08;
pub const SAT_TRIGGER_ON_ACTION: u8 = 0x10;
pub const SAT_TRIGGER_FRONT: u8 = 0x20;
pub const SAT_TRIGGER_CENTER: u8 = 0x40;
pub const SAT_4P: u8 = 0x80;

/// ID of an item in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoPrimitive, TryFromPrimitive)]
#[repr(u16)]
pub enum Item {
    Empty = 0,
    Knife = 1,
    HandgunLeon = 2,
    HandgunClaire = 3,
    CustomHandgun = 4,
    Magnum = 5,
    CustomMagnum = 6,
    Shotgun = 7,
    CustomShotgun = 8,
    GrenadeLauncherExplosive = 9,
    GrenadeLauncherFlame = 10,
    GrenadeLauncherAcid = 11,
    Bowgun = 12,
    ColtSaa = 13,
    Sparkshot = 14,
    SubMachinegun = 15,
    Flamethrower = 16,
    RocketLauncher = 17,
    GatlingGun = 18,
    Beretta = 19,
    HandgunAmmo = 20,
    ShotgunShells = 21,
    MagnumRounds = 22,
    FuelTank = 23,
    ExplosiveRounds = 24,
    FlameRounds = 25,
    AcidRounds = 26,
    SmgAmmo = 27,
    SsBattery = 28,
    BowgunDarts = 29,
    InkRibbon = 30,
    SmallKey = 31,
    HandgunParts = 32,
    MagnumParts = 33,
    ShotgunParts = 34,
    FirstAidSpray = 35,
    AntiVirusBomb = 36,
    ChemicalAcW24 = 37,
    GreenHerb = 38,
    RedHerb = 39,
    BlueHerb = 40,
    GGHerb = 41,
    RGHerb = 42,
    BGHerb = 43,
    GGGHerb = 44,
    GGBHerb = 45,
    RGBHerb = 46,
    Lighter = 47,
    Lockpick = 48,
    PhotoSherry = 49,
    ValveHandle = 50,
    RedJewel = 51,
    RedKeycard = 52,
    BlueKeycard = 53,
    SerpentStone = 54,
    JaguarStone = 55,
    JaguarStoneL = 56,
    JaguarStoneR = 57,
    EagleStone = 58,
    RookPlug = 59,
    QueenPlug = 60,
    KnightPlug = 61,
    KingPlug = 62,
    WeaponBoxKey = 63,
    Detonator = 64,
    Explosive = 65,
    DetonatorAndExplosive = 66,
    SquareCrank = 67,
    FilmA = 68,
    FilmB = 69,
    FilmC = 70,
    UnicornMedal = 71,
    EagleMedal = 72,
    WolfMedal = 73,
    Cogwheel = 74,
    ManholeOpener = 75,
    MainFuse = 76,
    FuseCase = 77,
    Vaccine = 78,
    VaccineBase = 79,
    FilmD = 80,
    VaccineCart = 81,
    GVirus = 82,
    SpecialKey = 83,
    JointPlugBlue = 84,
    JointPlugRed = 85,
    Cord = 86,
    PhotoAda = 87,
    CabinKey = 88,
    SpadeKey = 89,
    DiamondKey = 90,
    HeartKey = 91,
    ClubKey = 92,
    DownKey = 93,
    UpKey = 94,
    PowerRoomKey = 95,
    MoDisk = 96,
    UmbrellaKeycard = 97,
    MasterKey = 98,
    PlatformKey = 99,
}

impl Item {
    pub const fn is_weapon(&self) -> bool {
        matches!(
            self,
            Self::Knife | Self::HandgunLeon | Self::HandgunClaire | Self::CustomHandgun
            | Self::Magnum | Self::CustomMagnum | Self::Shotgun | Self::CustomShotgun
            | Self::GrenadeLauncherExplosive | Self::GrenadeLauncherFlame | Self::GrenadeLauncherAcid
            | Self::Bowgun | Self::ColtSaa | Self::Sparkshot | Self::SubMachinegun
            | Self::Flamethrower | Self::RocketLauncher | Self::GatlingGun | Self::Beretta
        )
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Empty => "Empty",
            Self::Knife => "Knife",
            Self::HandgunLeon => "Handgun (Leon)",
            Self::HandgunClaire => "Handgun (Claire)",
            Self::CustomHandgun => "Custom Handgun",
            Self::Magnum => "Magnum",
            Self::CustomMagnum => "Custom Magnum",
            Self::Shotgun => "Shotgun",
            Self::CustomShotgun => "Custom Shotgun",
            Self::GrenadeLauncherExplosive => "Grenade Launcher (Explosive)",
            Self::GrenadeLauncherFlame => "Grenade Launcher (Flame)",
            Self::GrenadeLauncherAcid => "Grenade Launcher (Acid)",
            Self::Bowgun => "Bowgun",
            Self::ColtSaa => "Colt SAA",
            Self::Sparkshot => "Sparkshot",
            Self::SubMachinegun => "Sub Machinegun",
            Self::Flamethrower => "Flamethrower",
            Self::RocketLauncher => "Rocket Launcher",
            Self::GatlingGun => "Gatling Gun",
            Self::Beretta => "Beretta",
            Self::HandgunAmmo => "Handgun Ammo",
            Self::ShotgunShells => "Shotgun Shells",
            Self::MagnumRounds => "Magnum Rounds",
            Self::FuelTank => "Fuel Tank",
            Self::ExplosiveRounds => "Explosive Rounds",
            Self::FlameRounds => "Flame Rounds",
            Self::AcidRounds => "Acid Rounds",
            Self::SmgAmmo => "SMG Ammo",
            Self::SsBattery => "SS Battery",
            Self::BowgunDarts => "Bowgun Darts",
            Self::InkRibbon => "Ink Ribbon",
            Self::SmallKey => "Small Key",
            Self::HandgunParts => "Handgun Parts",
            Self::MagnumParts => "Magnum Parts",
            Self::ShotgunParts => "Shotgun Parts",
            Self::FirstAidSpray => "First Aid Spray",
            Self::AntiVirusBomb => "Anti Virus Bomb",
            Self::ChemicalAcW24 => "Chemical AC-W24",
            Self::GreenHerb => "Green Herb",
            Self::RedHerb => "Red Herb",
            Self::BlueHerb => "Blue Herb",
            Self::GGHerb => "Mixed Herbs (G+G)",
            Self::RGHerb => "Mixed Herbs (R+G)",
            Self::BGHerb => "Mixed Herbs (B+G)",
            Self::GGGHerb => "Mixed Herbs (G+G+G)",
            Self::GGBHerb => "Mixed Herbs (G+G+B)",
            Self::RGBHerb => "Mixed Herbs (R+G+B)",
            Self::Lighter => "Lighter",
            Self::Lockpick => "Lockpick",
            Self::PhotoSherry => "Photo (Sherry)",
            Self::ValveHandle => "Valve Handle",
            Self::RedJewel => "Red Jewel",
            Self::RedKeycard => "Red Keycard",
            Self::BlueKeycard => "Blue Keycard",
            Self::SerpentStone => "Serpent Stone",
            Self::JaguarStone => "Jaguar Stone",
            Self::JaguarStoneL => "Jaguar Stone L",
            Self::JaguarStoneR => "Jaguar Stone R",
            Self::EagleStone => "Eagle Stone",
            Self::RookPlug => "Rook Plug",
            Self::QueenPlug => "Queen Plug",
            Self::KnightPlug => "Knight Plug",
            Self::KingPlug => "King Plug",
            Self::WeaponBoxKey => "Weapon Box Key",
            Self::Detonator => "Detonator",
            Self::Explosive => "Explosive",
            Self::DetonatorAndExplosive => "Detonator and Explosive",
            Self::SquareCrank => "Square Crank",
            Self::FilmA => "Film A",
            Self::FilmB => "Film B",
            Self::FilmC => "Film C",
            Self::FilmD => "Film D",
            Self::UnicornMedal => "Unicorn Medal",
            Self::EagleMedal => "Eagle Medal",
            Self::WolfMedal => "Wolf Medal",
            Self::Cogwheel => "Cogwheel",
            Self::ManholeOpener => "Manhole Opener",
            Self::MainFuse => "Main Fuse",
            Self::FuseCase => "Fuse Case",
            Self::Vaccine => "Vaccine",
            Self::VaccineBase => "Vaccine Base",
            Self::VaccineCart => "Vaccine Cart",
            Self::GVirus => "G-Virus",
            Self::SpecialKey => "Special Key",
            Self::JointPlugBlue => "Joint Plug Blue",
            Self::JointPlugRed => "Joint Plug Red",
            Self::Cord => "Cord",
            Self::PhotoAda => "Photo (Ada)",
            Self::CabinKey => "Cabin Key",
            Self::SpadeKey => "Spade Key",
            Self::DiamondKey => "Diamond Key",
            Self::HeartKey => "Heart Key",
            Self::ClubKey => "Club Key",
            Self::DownKey => "Down Key",
            Self::UpKey => "Up Key",
            Self::PowerRoomKey => "Power Room Key",
            Self::MoDisk => "MO Disk",
            Self::UmbrellaKeycard => "Umbrella Keycard",
            Self::MasterKey => "Master Key",
            Self::PlatformKey => "Platform Key",
        }
    }

    pub fn name_from_id(id: u16) -> String {
        let name = Self::try_from(id).map(|item| item.name()).unwrap_or("Unknown");
        format!("{} ({})", name, id)
    }
}

/// ID of an AOT's type
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, IntoPrimitive)]
pub enum SceType {
    Auto = 0,
    Door = 1,
    Item = 2,
    Normal = 3,
    Message = 4,
    Event = 5,
    FlagChg = 6,
    Water = 7,
    Move = 8,
    Save = 9,
    ItemBox = 10,
    Damage = 11,
    Status = 12,
    Hikidashi = 13,
    Windows = 14,
    Unknown = 0xFF,
}

impl SceType {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Auto => "Auto",
            Self::Door => "Door",
            Self::Item => "Item",
            Self::Normal => "Normal",
            Self::Message => "Message",
            Self::Event => "Event",
            Self::FlagChg => "Flag Change",
            Self::Water => "Water",
            Self::Move => "Move",
            Self::Save => "Save",
            Self::ItemBox => "Item Box",
            Self::Damage => "Damage",
            Self::Status => "Status",
            Self::Hikidashi => "Hikidashi",
            Self::Windows => "Windows",
            Self::Unknown => "Unknown",
        }
    }

    pub const fn is_trigger(&self) -> bool {
        matches!(self, Self::Door | Self::Event | Self::FlagChg | Self::Item | Self::ItemBox | Self::Save | Self::Damage | Self::Message)
    }
}

impl From<u8> for SceType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Auto,
            1 => Self::Door,
            2 => Self::Item,
            3 => Self::Normal,
            4 => Self::Message,
            5 => Self::Event,
            6 => Self::FlagChg,
            7 => Self::Water,
            8 => Self::Move,
            9 => Self::Save,
            10 => Self::ItemBox,
            11 => Self::Damage,
            12 => Self::Status,
            13 => Self::Hikidashi,
            14 => Self::Windows,
            _ => Self::Unknown,
        }
    }
}