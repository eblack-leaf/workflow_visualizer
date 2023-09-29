use crate::icon::IconData;

pub enum BundledIcon {
    Activity,
    Airplay,
    AlertCircle,
    AlertOctagon,
    AlertTriangle,
    AlignCenter,
    AlignJustify,
    AlignLeft,
    AlignRight,
    Anchor,
    Aperture,
    Archive,
    ArrowDown,
    ArrowDownCircle,
    ArrowDownLeft,
    ArrowDownRight,
    ArrowLeft,
    ArrowLeftCircle,
    ArrowRight,
    ArrowRightCircle,
    ArrowUp,
    ArrowUpCircle,
    ArrowUpLeft,
    ArrowUpRight,
    AtSign,
    Award,
    BarChart,
    BarChart2,
    Battery,
    BatteryCharging,
    Bell,
    BellOff,
    Bluetooth,
    Bold,
    Book,
    BookOpen,
    Bookmark,
    Box,
    Briefcase,
    Calendar,
    Camera,
    CameraOff,
    Cast,
    Check,
    CheckCircle,
    CheckSquare,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronsDown,
    ChevronsLeft,
    ChevronsRight,
    ChevronsUp,
    Chrome,
    Circle,
    Clipboard,
    Clock,
    Cloud,
    CloudDrizzle,
    CloudLightning,
    CloudOff,
    CloudRain,
    CloudSnow,
    Code,
    Codepen,
    CodeSandbox,
    Coffee,
    Columns,
    Command,
    Compass,
    Copy,
    CornerDownLeft,
    CornerDownRight,
    CornerLeftDown,
    CornerLeftUp,
    CornerRightDown,
    CornerRightUp,
    CornerUpLeft,
    CornerUpRight,
    Cpu,
    CreditCard,
    Crop,
    Crosshair,
    Database,
    Delete,
    Disc,
    Divide,
    DivideCircle,
    DivideSquare,
    DollarSign,
    Download,
    DownloadCloud,
    Dribble,
    Droplet,
    Edit,
    EditTwo,
    EditThree,
    ExternalLink,
    Eye,
    EyeOff,
    Facebook,
    FastForward,
    Feather,
    Figma,
    File,
    FileMinus,
    FilePlus,
    FileText,
    Film,
    Filter,
    Flag,
    Folder,
    FolderMinus,
    FolderPlus,
    Framer,
    Frown,
    Gift,
    GitBranch,
    GitCommit,
    GitMerge,
    GitPullRequest,
    Github,
    Gitlab,
    Globe,
    Grid,
    HardDrive,
    Hash,
    Headphones,
    Heart,
    HelpCircle,
    Hexagon,
    Home,
    Image,
    Inbox,
    Info,
    Instagram,
    Italic,
    Key,
    Layers,
    Layout,
    LifeBuoy,
    Link,
    LinkTwo,
    LinkedIn,
    List,
    Loader,
    Lock,
    LogIn,
    LogOut,
    Mail,
    Map,
    MapPin,
    Maximize,
    MaximizeTwo,
    Meh,
    Menu,
    MessageCircle,
    MessageSquare,
    Mic,
    MicOff,
    Minimize,
    MinimizeTwo,
    Minus,
    MinusCircle,
    MinusSquare,
    Monitor,
    Moon,
    MoreHorizontal,
    MoreVertical,
    MousePointer,
    Move,
    Music,
    Navigation,
    NavigationTwo,
    Octagon,
    Package,
    Paperclip,
    Pause,
    PauseCircle,
    PenTool,
    Percent,
    Phone,
    PhoneCall,
    PhoneForwarded,
    PhoneIncoming,
    PhoneOff,
    PhoneOutgoing,
    PieChart,
    Play,
    PlayCircle,
    Plus,
    PlusCircle,
    PlusSquare,
    Pocket,
    Power,
    Printer,
    Radio,
    RefreshCCW,
    RefreshCW,
    Repeat,
    Rewind,
    RotateCCW,
    RotateCW,
    RSS,
    Save,
    Scissors,
    Search,
    Send,
    Server,
    Settings,
    Share,
    ShareTwo,
    Shield,
    ShieldOff,
    ShoppingBag,
    ShoppingCart,
    Shuffle,
    Sidebar,
    SkipBack,
    SkipForward,
    Slack,
    Slash,
    Sliders,
    Smartphone,
    Smile,
    Speaker,
    Square,
    Star,
    StopCircle,
    Sun,
    Sunrise,
    Sunset,
    Table,
    Tablet,
    Tag,
    Target,
    Terminal,
    Thermometer,
    ThumbsDown,
    ThumbsUp,
    ToggleLeft,
    ToggleRight,
    Tool,
    Trash,
    TrashTwo,
    Trello,
    TrendingDown,
    TrendingUp,
    Triangle,
    Truck,
    TV,
    Twitch,
    Twitter,
    Type,
    Umbrella,
    Underline,
    Unlock,
    Upload,
    UploadCloud,
    User,
    UserCheck,
    UserMinus,
    UserPlus,
    UserX,
    Users,
    Video,
    VideoOff,
    Voicemail,
    Volume,
    VolumeOne,
    VolumeTwo,
    VolumeX,
    Watch,
    Wifi,
    WifiOff,
    Wind,
    X,
    XCircle,
    XOctagon,
    XSquare,
    Youtube,
    Zap,
    ZapOff,
    ZoomIn,
    ZoomOut,
}

impl BundledIcon {
    pub fn data(&self) -> IconData {
        match &self {
            BundledIcon::Activity => include_bytes!("bundled_image_icons/activity.png").to_vec(),
            BundledIcon::Airplay => include_bytes!("bundled_image_icons/airplay.png").to_vec(),
            BundledIcon::AlertCircle => {
                include_bytes!("bundled_image_icons/alert-circle.png").to_vec()
            }
            BundledIcon::AlignCenter => {
                include_bytes!("bundled_image_icons/align-center.png").to_vec()
            }
            BundledIcon::AlignJustify => {
                include_bytes!("bundled_image_icons/align-justify.png").to_vec()
            }
            BundledIcon::AlignLeft => include_bytes!("bundled_image_icons/align-left.png").to_vec(),
            BundledIcon::AlertTriangle => {
                include_bytes!("bundled_image_icons/alert-triangle.png").to_vec()
            }
            BundledIcon::AlertOctagon => {
                include_bytes!("bundled_image_icons/alert-octagon.png").to_vec()
            }
            BundledIcon::AlignRight => {
                include_bytes!("bundled_image_icons/align-right.png").to_vec()
            }
            BundledIcon::ArrowRight => {
                include_bytes!("bundled_image_icons/arrow-right.png").to_vec()
            }
            BundledIcon::ArrowLeft => include_bytes!("bundled_image_icons/arrow-left.png").to_vec(),
            BundledIcon::Anchor => include_bytes!("bundled_image_icons/anchor.png").to_vec(),
            BundledIcon::Aperture => include_bytes!("bundled_image_icons/aperture.png").to_vec(),
            BundledIcon::Archive => include_bytes!("bundled_image_icons/archive.png").to_vec(),
            BundledIcon::ArrowDown => include_bytes!("bundled_image_icons/arrow-down.png").to_vec(),
            BundledIcon::ArrowDownCircle => {
                include_bytes!("bundled_image_icons/arrow-down-circle.png").to_vec()
            }
            BundledIcon::ArrowDownLeft => {
                include_bytes!("bundled_image_icons/arrow-down-left.png").to_vec()
            }
            BundledIcon::ArrowDownRight => {
                include_bytes!("bundled_image_icons/arrow-down-right.png").to_vec()
            }
            BundledIcon::ArrowLeftCircle => {
                include_bytes!("bundled_image_icons/arrow-left-circle.png").to_vec()
            }
            BundledIcon::ArrowRightCircle => {
                include_bytes!("bundled_image_icons/arrow-right-circle.png").to_vec()
            }
            BundledIcon::ArrowUp => include_bytes!("bundled_image_icons/arrow-up.png").to_vec(),
            BundledIcon::ArrowUpCircle => {
                include_bytes!("bundled_image_icons/arrow-up-circle.png").to_vec()
            }
            BundledIcon::ArrowUpLeft => {
                include_bytes!("bundled_image_icons/arrow-up-left.png").to_vec()
            }
            BundledIcon::ArrowUpRight => {
                include_bytes!("bundled_image_icons/arrow-up-right.png").to_vec()
            }
            BundledIcon::AtSign => include_bytes!("bundled_image_icons/at-sign.png").to_vec(),
            BundledIcon::Award => include_bytes!("bundled_image_icons/award.png").to_vec(),
            BundledIcon::BarChart => include_bytes!("bundled_image_icons/bar-chart.png").to_vec(),
            BundledIcon::BarChart2 => {
                include_bytes!("bundled_image_icons/bar-chart-2.png").to_vec()
            }
            BundledIcon::Battery => include_bytes!("bundled_image_icons/battery.png").to_vec(),
            BundledIcon::BatteryCharging => {
                include_bytes!("bundled_image_icons/battery-charging.png").to_vec()
            }
            BundledIcon::Bell => include_bytes!("bundled_image_icons/bell.png").to_vec(),
            BundledIcon::BellOff => include_bytes!("bundled_image_icons/bell-off.png").to_vec(),
            BundledIcon::Bluetooth => include_bytes!("bundled_image_icons/bluetooth.png").to_vec(),
            BundledIcon::Bold => include_bytes!("bundled_image_icons/bold.png").to_vec(),
            BundledIcon::Book => include_bytes!("bundled_image_icons/book.png").to_vec(),
            BundledIcon::BookOpen => include_bytes!("bundled_image_icons/book-open.png").to_vec(),
            BundledIcon::Bookmark => include_bytes!("bundled_image_icons/bookmark.png").to_vec(),
            BundledIcon::Box => include_bytes!("bundled_image_icons/box.png").to_vec(),
            BundledIcon::Briefcase => include_bytes!("bundled_image_icons/briefcase.png").to_vec(),
            BundledIcon::Calendar => include_bytes!("bundled_image_icons/calendar.png").to_vec(),
            BundledIcon::Camera => include_bytes!("bundled_image_icons/camera.png").to_vec(),
            BundledIcon::CameraOff => include_bytes!("bundled_image_icons/camera-off.png").to_vec(),
            BundledIcon::Cast => include_bytes!("bundled_image_icons/cast.png").to_vec(),
            BundledIcon::Check => include_bytes!("bundled_image_icons/check.png").to_vec(),
            BundledIcon::CheckCircle => {
                include_bytes!("bundled_image_icons/check-circle.png").to_vec()
            }
            BundledIcon::CheckSquare => {
                include_bytes!("bundled_image_icons/check-square.png").to_vec()
            }
            BundledIcon::ChevronDown => {
                include_bytes!("bundled_image_icons/chevron-down.png").to_vec()
            }
            BundledIcon::ChevronLeft => {
                include_bytes!("bundled_image_icons/chevron-left.png").to_vec()
            }
            BundledIcon::ChevronRight => {
                include_bytes!("bundled_image_icons/chevron-right.png").to_vec()
            }
            BundledIcon::ChevronUp => include_bytes!("bundled_image_icons/chevron-up.png").to_vec(),
            BundledIcon::ChevronsDown => {
                include_bytes!("bundled_image_icons/chevrons-down.png").to_vec()
            }
            BundledIcon::ChevronsLeft => {
                include_bytes!("bundled_image_icons/chevrons-left.png").to_vec()
            }
            BundledIcon::ChevronsRight => {
                include_bytes!("bundled_image_icons/chevrons-right.png").to_vec()
            }
            BundledIcon::ChevronsUp => {
                include_bytes!("bundled_image_icons/chevrons-up.png").to_vec()
            }
            BundledIcon::Chrome => include_bytes!("bundled_image_icons/chrome.png").to_vec(),
            BundledIcon::Circle => include_bytes!("bundled_image_icons/circle.png").to_vec(),
            BundledIcon::Clipboard => include_bytes!("bundled_image_icons/clipboard.png").to_vec(),
            BundledIcon::Clock => include_bytes!("bundled_image_icons/clock.png").to_vec(),
            BundledIcon::Cloud => include_bytes!("bundled_image_icons/cloud.png").to_vec(),
            BundledIcon::CloudDrizzle => {
                include_bytes!("bundled_image_icons/cloud-drizzle.png").to_vec()
            }
            BundledIcon::CloudLightning => {
                include_bytes!("bundled_image_icons/cloud-lightning.png").to_vec()
            }
            BundledIcon::CloudOff => include_bytes!("bundled_image_icons/cloud-off.png").to_vec(),
            BundledIcon::CloudRain => include_bytes!("bundled_image_icons/cloud-rain.png").to_vec(),
            BundledIcon::CloudSnow => include_bytes!("bundled_image_icons/cloud-snow.png").to_vec(),
            BundledIcon::Code => include_bytes!("bundled_image_icons/code.png").to_vec(),
            BundledIcon::Codepen => include_bytes!("bundled_image_icons/codepen.png").to_vec(),
            BundledIcon::CodeSandbox => {
                include_bytes!("bundled_image_icons/codesandbox.png").to_vec()
            }
            BundledIcon::Coffee => include_bytes!("bundled_image_icons/coffee.png").to_vec(),
            BundledIcon::Columns => include_bytes!("bundled_image_icons/columns.png").to_vec(),
            BundledIcon::Command => include_bytes!("bundled_image_icons/command.png").to_vec(),
            BundledIcon::Compass => include_bytes!("bundled_image_icons/compass.png").to_vec(),
            BundledIcon::Copy => include_bytes!("bundled_image_icons/copy.png").to_vec(),
            BundledIcon::CornerDownLeft => {
                include_bytes!("bundled_image_icons/corner-down-left.png").to_vec()
            }
            BundledIcon::CornerDownRight => {
                include_bytes!("bundled_image_icons/corner-down-right.png").to_vec()
            }
            BundledIcon::CornerLeftDown => {
                include_bytes!("bundled_image_icons/corner-left-down.png").to_vec()
            }
            BundledIcon::CornerLeftUp => {
                include_bytes!("bundled_image_icons/corner-left-up.png").to_vec()
            }
            BundledIcon::CornerRightDown => {
                include_bytes!("bundled_image_icons/corner-right-down.png").to_vec()
            }
            BundledIcon::CornerRightUp => {
                include_bytes!("bundled_image_icons/corner-right-up.png").to_vec()
            }
            BundledIcon::CornerUpLeft => {
                include_bytes!("bundled_image_icons/corner-up-left.png").to_vec()
            }
            BundledIcon::CornerUpRight => {
                include_bytes!("bundled_image_icons/corner-up-right.png").to_vec()
            }
            BundledIcon::Cpu => include_bytes!("bundled_image_icons/cpu.png").to_vec(),
            BundledIcon::CreditCard => {
                include_bytes!("bundled_image_icons/credit-card.png").to_vec()
            }
            BundledIcon::Crop => include_bytes!("bundled_image_icons/crop.png").to_vec(),
            BundledIcon::Crosshair => include_bytes!("bundled_image_icons/crosshair.png").to_vec(),
            BundledIcon::Database => include_bytes!("bundled_image_icons/database.png").to_vec(),
            BundledIcon::Delete => include_bytes!("bundled_image_icons/delete.png").to_vec(),
            BundledIcon::Disc => include_bytes!("bundled_image_icons/disc.png").to_vec(),
            BundledIcon::Divide => include_bytes!("bundled_image_icons/divide.png").to_vec(),
            BundledIcon::DivideCircle => {
                include_bytes!("bundled_image_icons/divide-circle.png").to_vec()
            }
            BundledIcon::DivideSquare => {
                include_bytes!("bundled_image_icons/divide-square.png").to_vec()
            }
            BundledIcon::DollarSign => {
                include_bytes!("bundled_image_icons/dollar-sign.png").to_vec()
            }
            BundledIcon::Download => include_bytes!("bundled_image_icons/download.png").to_vec(),
            BundledIcon::DownloadCloud => {
                include_bytes!("bundled_image_icons/download-cloud.png").to_vec()
            }
            BundledIcon::Dribble => include_bytes!("bundled_image_icons/dribbble.png").to_vec(),
            BundledIcon::Droplet => include_bytes!("bundled_image_icons/droplet.png").to_vec(),
            BundledIcon::Edit => include_bytes!("bundled_image_icons/edit.png").to_vec(),
            BundledIcon::EditTwo => include_bytes!("bundled_image_icons/edit-2.png").to_vec(),
            BundledIcon::EditThree => include_bytes!("bundled_image_icons/edit-3.png").to_vec(),
            BundledIcon::ExternalLink => {
                include_bytes!("bundled_image_icons/external-link.png").to_vec()
            }
            BundledIcon::Eye => include_bytes!("bundled_image_icons/eye.png").to_vec(),
            BundledIcon::EyeOff => include_bytes!("bundled_image_icons/eye-off.png").to_vec(),
            BundledIcon::Facebook => include_bytes!("bundled_image_icons/facebook.png").to_vec(),
            BundledIcon::FastForward => {
                include_bytes!("bundled_image_icons/fast-forward.png").to_vec()
            }
            BundledIcon::Feather => include_bytes!("bundled_image_icons/feather.png").to_vec(),
            BundledIcon::Figma => include_bytes!("bundled_image_icons/figma.png").to_vec(),
            BundledIcon::File => include_bytes!("bundled_image_icons/file.png").to_vec(),
            BundledIcon::FileMinus => include_bytes!("bundled_image_icons/file-minus.png").to_vec(),
            BundledIcon::FilePlus => include_bytes!("bundled_image_icons/file-plus.png").to_vec(),
            BundledIcon::FileText => include_bytes!("bundled_image_icons/file-text.png").to_vec(),
            BundledIcon::Film => include_bytes!("bundled_image_icons/film.png").to_vec(),
            BundledIcon::Filter => include_bytes!("bundled_image_icons/filter.png").to_vec(),
            BundledIcon::Flag => include_bytes!("bundled_image_icons/flag.png").to_vec(),
            BundledIcon::Folder => include_bytes!("bundled_image_icons/folder.png").to_vec(),
            BundledIcon::FolderMinus => {
                include_bytes!("bundled_image_icons/folder-minus.png").to_vec()
            }
            BundledIcon::FolderPlus => {
                include_bytes!("bundled_image_icons/folder-plus.png").to_vec()
            }
            BundledIcon::Framer => include_bytes!("bundled_image_icons/framer.png").to_vec(),
            BundledIcon::Frown => include_bytes!("bundled_image_icons/frown.png").to_vec(),
            BundledIcon::Gift => include_bytes!("bundled_image_icons/gift.png").to_vec(),
            BundledIcon::GitBranch => include_bytes!("bundled_image_icons/git-branch.png").to_vec(),
            BundledIcon::GitCommit => include_bytes!("bundled_image_icons/git-commit.png").to_vec(),
            BundledIcon::GitMerge => include_bytes!("bundled_image_icons/git-merge.png").to_vec(),
            BundledIcon::GitPullRequest => {
                include_bytes!("bundled_image_icons/git-pull-request.png").to_vec()
            }
            BundledIcon::Github => include_bytes!("bundled_image_icons/github.png").to_vec(),
            BundledIcon::Gitlab => include_bytes!("bundled_image_icons/gitlab.png").to_vec(),
            BundledIcon::Globe => include_bytes!("bundled_image_icons/globe.png").to_vec(),
            BundledIcon::Grid => include_bytes!("bundled_image_icons/grid.png").to_vec(),
            BundledIcon::HardDrive => include_bytes!("bundled_image_icons/hard-drive.png").to_vec(),
            BundledIcon::Hash => include_bytes!("bundled_image_icons/hash.png").to_vec(),
            BundledIcon::Headphones => {
                include_bytes!("bundled_image_icons/headphones.png").to_vec()
            }
            BundledIcon::Heart => include_bytes!("bundled_image_icons/heart.png").to_vec(),
            BundledIcon::HelpCircle => {
                include_bytes!("bundled_image_icons/help-circle.png").to_vec()
            }
            BundledIcon::Hexagon => include_bytes!("bundled_image_icons/hexagon.png").to_vec(),
            BundledIcon::Home => include_bytes!("bundled_image_icons/home.png").to_vec(),
            BundledIcon::Image => include_bytes!("bundled_image_icons/image.png").to_vec(),
            BundledIcon::Inbox => include_bytes!("bundled_image_icons/inbox.png").to_vec(),
            BundledIcon::Info => include_bytes!("bundled_image_icons/info.png").to_vec(),
            BundledIcon::Instagram => include_bytes!("bundled_image_icons/instagram.png").to_vec(),
            BundledIcon::Italic => include_bytes!("bundled_image_icons/italic.png").to_vec(),
            BundledIcon::Key => include_bytes!("bundled_image_icons/key.png").to_vec(),
            BundledIcon::Layers => include_bytes!("bundled_image_icons/layers.png").to_vec(),
            BundledIcon::Layout => include_bytes!("bundled_image_icons/layout.png").to_vec(),
            BundledIcon::LifeBuoy => include_bytes!("bundled_image_icons/life-buoy.png").to_vec(),
            BundledIcon::Link => include_bytes!("bundled_image_icons/link.png").to_vec(),
            BundledIcon::LinkTwo => include_bytes!("bundled_image_icons/link-2.png").to_vec(),
            BundledIcon::LinkedIn => include_bytes!("bundled_image_icons/linkedin.png").to_vec(),
            BundledIcon::List => include_bytes!("bundled_image_icons/list.png").to_vec(),
            BundledIcon::Loader => include_bytes!("bundled_image_icons/loader.png").to_vec(),
            BundledIcon::Lock => include_bytes!("bundled_image_icons/lock.png").to_vec(),
            BundledIcon::LogIn => include_bytes!("bundled_image_icons/log-in.png").to_vec(),
            BundledIcon::LogOut => include_bytes!("bundled_image_icons/log-out.png").to_vec(),
            BundledIcon::Mail => include_bytes!("bundled_image_icons/mail.png").to_vec(),
            BundledIcon::Map => include_bytes!("bundled_image_icons/map.png").to_vec(),
            BundledIcon::MapPin => include_bytes!("bundled_image_icons/map-pin.png").to_vec(),
            BundledIcon::Maximize => include_bytes!("bundled_image_icons/maximize.png").to_vec(),
            BundledIcon::MaximizeTwo => {
                include_bytes!("bundled_image_icons/maximize-2.png").to_vec()
            }
            BundledIcon::Meh => include_bytes!("bundled_image_icons/meh.png").to_vec(),
            BundledIcon::Menu => include_bytes!("bundled_image_icons/menu.png").to_vec(),
            BundledIcon::MessageCircle => {
                include_bytes!("bundled_image_icons/message-circle.png").to_vec()
            }
            BundledIcon::MessageSquare => {
                include_bytes!("bundled_image_icons/message-square.png").to_vec()
            }
            BundledIcon::Mic => include_bytes!("bundled_image_icons/mic.png").to_vec(),
            BundledIcon::MicOff => include_bytes!("bundled_image_icons/mic-off.png").to_vec(),
            BundledIcon::Minimize => include_bytes!("bundled_image_icons/minimize.png").to_vec(),
            BundledIcon::MinimizeTwo => {
                include_bytes!("bundled_image_icons/minimize-2.png").to_vec()
            }
            BundledIcon::Minus => include_bytes!("bundled_image_icons/minus.png").to_vec(),
            BundledIcon::MinusCircle => {
                include_bytes!("bundled_image_icons/minus-circle.png").to_vec()
            }
            BundledIcon::MinusSquare => {
                include_bytes!("bundled_image_icons/minus-square.png").to_vec()
            }
            BundledIcon::Monitor => include_bytes!("bundled_image_icons/monitor.png").to_vec(),
            BundledIcon::Moon => include_bytes!("bundled_image_icons/moon.png").to_vec(),
            BundledIcon::MoreHorizontal => {
                include_bytes!("bundled_image_icons/more-horizontal.png").to_vec()
            }
            BundledIcon::MoreVertical => {
                include_bytes!("bundled_image_icons/more-vertical.png").to_vec()
            }
            BundledIcon::MousePointer => {
                include_bytes!("bundled_image_icons/mouse-pointer.png").to_vec()
            }
            BundledIcon::Move => include_bytes!("bundled_image_icons/move.png").to_vec(),
            BundledIcon::Music => include_bytes!("bundled_image_icons/music.png").to_vec(),
            BundledIcon::Navigation => {
                include_bytes!("bundled_image_icons/navigation.png").to_vec()
            }
            BundledIcon::NavigationTwo => {
                include_bytes!("bundled_image_icons/navigation-2.png").to_vec()
            }
            BundledIcon::Octagon => include_bytes!("bundled_image_icons/octagon.png").to_vec(),
            BundledIcon::Package => include_bytes!("bundled_image_icons/package.png").to_vec(),
            BundledIcon::Paperclip => include_bytes!("bundled_image_icons/paperclip.png").to_vec(),
            BundledIcon::Pause => include_bytes!("bundled_image_icons/pause.png").to_vec(),
            BundledIcon::PauseCircle => {
                include_bytes!("bundled_image_icons/pause-circle.png").to_vec()
            }
            BundledIcon::PenTool => include_bytes!("bundled_image_icons/pen-tool.png").to_vec(),
            BundledIcon::Percent => include_bytes!("bundled_image_icons/percent.png").to_vec(),
            BundledIcon::Phone => include_bytes!("bundled_image_icons/phone.png").to_vec(),
            BundledIcon::PhoneCall => include_bytes!("bundled_image_icons/phone-call.png").to_vec(),
            BundledIcon::PhoneForwarded => {
                include_bytes!("bundled_image_icons/phone-forwarded.png").to_vec()
            }
            BundledIcon::PhoneIncoming => {
                include_bytes!("bundled_image_icons/phone-incoming.png").to_vec()
            }
            BundledIcon::PhoneOff => include_bytes!("bundled_image_icons/phone-off.png").to_vec(),
            BundledIcon::PhoneOutgoing => {
                include_bytes!("bundled_image_icons/phone-outgoing.png").to_vec()
            }
            BundledIcon::PieChart => include_bytes!("bundled_image_icons/pie-chart.png").to_vec(),
            BundledIcon::Play => include_bytes!("bundled_image_icons/play.png").to_vec(),
            BundledIcon::PlayCircle => {
                include_bytes!("bundled_image_icons/play-circle.png").to_vec()
            }
            BundledIcon::Plus => include_bytes!("bundled_image_icons/plus.png").to_vec(),
            BundledIcon::PlusCircle => {
                include_bytes!("bundled_image_icons/plus-circle.png").to_vec()
            }
            BundledIcon::PlusSquare => {
                include_bytes!("bundled_image_icons/plus-square.png").to_vec()
            }
            BundledIcon::Pocket => include_bytes!("bundled_image_icons/pocket.png").to_vec(),
            BundledIcon::Power => include_bytes!("bundled_image_icons/power.png").to_vec(),
            BundledIcon::Printer => include_bytes!("bundled_image_icons/printer.png").to_vec(),
            BundledIcon::Radio => include_bytes!("bundled_image_icons/radio.png").to_vec(),
            BundledIcon::RefreshCCW => {
                include_bytes!("bundled_image_icons/refresh-ccw.png").to_vec()
            }
            BundledIcon::RefreshCW => include_bytes!("bundled_image_icons/refresh-cw.png").to_vec(),
            BundledIcon::Repeat => include_bytes!("bundled_image_icons/repeat.png").to_vec(),
            BundledIcon::Rewind => include_bytes!("bundled_image_icons/rewind.png").to_vec(),
            BundledIcon::RotateCCW => include_bytes!("bundled_image_icons/rotate-ccw.png").to_vec(),
            BundledIcon::RotateCW => include_bytes!("bundled_image_icons/rotate-cw.png").to_vec(),
            BundledIcon::RSS => include_bytes!("bundled_image_icons/rss.png").to_vec(),
            BundledIcon::Save => include_bytes!("bundled_image_icons/save.png").to_vec(),
            BundledIcon::Scissors => include_bytes!("bundled_image_icons/scissors.png").to_vec(),
            BundledIcon::Search => include_bytes!("bundled_image_icons/search.png").to_vec(),
            BundledIcon::Send => include_bytes!("bundled_image_icons/send.png").to_vec(),
            BundledIcon::Server => include_bytes!("bundled_image_icons/server.png").to_vec(),
            BundledIcon::Settings => include_bytes!("bundled_image_icons/settings.png").to_vec(),
            BundledIcon::Share => include_bytes!("bundled_image_icons/share.png").to_vec(),
            BundledIcon::ShareTwo => include_bytes!("bundled_image_icons/share-2.png").to_vec(),
            BundledIcon::Shield => include_bytes!("bundled_image_icons/shield.png").to_vec(),
            BundledIcon::ShieldOff => include_bytes!("bundled_image_icons/shield-off.png").to_vec(),
            BundledIcon::ShoppingBag => {
                include_bytes!("bundled_image_icons/shopping-bag.png").to_vec()
            }
            BundledIcon::ShoppingCart => {
                include_bytes!("bundled_image_icons/shopping-cart.png").to_vec()
            }
            BundledIcon::Shuffle => include_bytes!("bundled_image_icons/shuffle.png").to_vec(),
            BundledIcon::Sidebar => include_bytes!("bundled_image_icons/sidebar.png").to_vec(),
            BundledIcon::SkipBack => include_bytes!("bundled_image_icons/skip-back.png").to_vec(),
            BundledIcon::SkipForward => {
                include_bytes!("bundled_image_icons/skip-forward.png").to_vec()
            }
            BundledIcon::Slack => include_bytes!("bundled_image_icons/slack.png").to_vec(),
            BundledIcon::Slash => include_bytes!("bundled_image_icons/slash.png").to_vec(),
            BundledIcon::Sliders => include_bytes!("bundled_image_icons/sliders.png").to_vec(),
            BundledIcon::Smartphone => {
                include_bytes!("bundled_image_icons/smartphone.png").to_vec()
            }
            BundledIcon::Smile => include_bytes!("bundled_image_icons/smile.png").to_vec(),
            BundledIcon::Speaker => include_bytes!("bundled_image_icons/speaker.png").to_vec(),
            BundledIcon::Square => include_bytes!("bundled_image_icons/square.png").to_vec(),
            BundledIcon::Star => include_bytes!("bundled_image_icons/star.png").to_vec(),
            BundledIcon::StopCircle => {
                include_bytes!("bundled_image_icons/stop-circle.png").to_vec()
            }
            BundledIcon::Sun => include_bytes!("bundled_image_icons/sun.png").to_vec(),
            BundledIcon::Sunrise => include_bytes!("bundled_image_icons/sunrise.png").to_vec(),
            BundledIcon::Sunset => include_bytes!("bundled_image_icons/sunset.png").to_vec(),
            BundledIcon::Table => include_bytes!("bundled_image_icons/table.png").to_vec(),
            BundledIcon::Tablet => include_bytes!("bundled_image_icons/tablet.png").to_vec(),
            BundledIcon::Tag => include_bytes!("bundled_image_icons/tag.png").to_vec(),
            BundledIcon::Target => include_bytes!("bundled_image_icons/target.png").to_vec(),
            BundledIcon::Terminal => include_bytes!("bundled_image_icons/terminal.png").to_vec(),
            BundledIcon::Thermometer => {
                include_bytes!("bundled_image_icons/thermometer.png").to_vec()
            }
            BundledIcon::ThumbsDown => {
                include_bytes!("bundled_image_icons/thumbs-down.png").to_vec()
            }
            BundledIcon::ThumbsUp => include_bytes!("bundled_image_icons/thumbs-up.png").to_vec(),
            BundledIcon::ToggleLeft => {
                include_bytes!("bundled_image_icons/toggle-left.png").to_vec()
            }
            BundledIcon::ToggleRight => {
                include_bytes!("bundled_image_icons/toggle-right.png").to_vec()
            }
            BundledIcon::Tool => include_bytes!("bundled_image_icons/tool.png").to_vec(),
            BundledIcon::Trash => include_bytes!("bundled_image_icons/trash.png").to_vec(),
            BundledIcon::TrashTwo => include_bytes!("bundled_image_icons/trash-2.png").to_vec(),
            BundledIcon::Trello => include_bytes!("bundled_image_icons/trello.png").to_vec(),
            BundledIcon::TrendingDown => {
                include_bytes!("bundled_image_icons/trending-down.png").to_vec()
            }
            BundledIcon::TrendingUp => {
                include_bytes!("bundled_image_icons/trending-up.png").to_vec()
            }
            BundledIcon::Triangle => include_bytes!("bundled_image_icons/triangle.png").to_vec(),
            BundledIcon::Truck => include_bytes!("bundled_image_icons/truck.png").to_vec(),
            BundledIcon::TV => include_bytes!("bundled_image_icons/tv.png").to_vec(),
            BundledIcon::Twitch => include_bytes!("bundled_image_icons/twitch.png").to_vec(),
            BundledIcon::Twitter => include_bytes!("bundled_image_icons/twitter.png").to_vec(),
            BundledIcon::Type => include_bytes!("bundled_image_icons/type.png").to_vec(),
            BundledIcon::Umbrella => include_bytes!("bundled_image_icons/umbrella.png").to_vec(),
            BundledIcon::Underline => include_bytes!("bundled_image_icons/underline.png").to_vec(),
            BundledIcon::Unlock => include_bytes!("bundled_image_icons/unlock.png").to_vec(),
            BundledIcon::Upload => include_bytes!("bundled_image_icons/upload.png").to_vec(),
            BundledIcon::UploadCloud => {
                include_bytes!("bundled_image_icons/upload-cloud.png").to_vec()
            }
            BundledIcon::User => include_bytes!("bundled_image_icons/user.png").to_vec(),
            BundledIcon::UserCheck => include_bytes!("bundled_image_icons/user-check.png").to_vec(),
            BundledIcon::UserMinus => include_bytes!("bundled_image_icons/user-minus.png").to_vec(),
            BundledIcon::UserPlus => include_bytes!("bundled_image_icons/user-plus.png").to_vec(),
            BundledIcon::UserX => include_bytes!("bundled_image_icons/user-x.png").to_vec(),
            BundledIcon::Users => include_bytes!("bundled_image_icons/users.png").to_vec(),
            BundledIcon::Video => include_bytes!("bundled_image_icons/video.png").to_vec(),
            BundledIcon::VideoOff => include_bytes!("bundled_image_icons/video-off.png").to_vec(),
            BundledIcon::Voicemail => include_bytes!("bundled_image_icons/voicemail.png").to_vec(),
            BundledIcon::Volume => include_bytes!("bundled_image_icons/volume.png").to_vec(),
            BundledIcon::VolumeOne => include_bytes!("bundled_image_icons/volume-1.png").to_vec(),
            BundledIcon::VolumeTwo => include_bytes!("bundled_image_icons/volume-2.png").to_vec(),
            BundledIcon::VolumeX => include_bytes!("bundled_image_icons/volume-x.png").to_vec(),
            BundledIcon::Watch => include_bytes!("bundled_image_icons/watch.png").to_vec(),
            BundledIcon::Wifi => include_bytes!("bundled_image_icons/wifi.png").to_vec(),
            BundledIcon::WifiOff => include_bytes!("bundled_image_icons/wifi-off.png").to_vec(),
            BundledIcon::Wind => include_bytes!("bundled_image_icons/wind.png").to_vec(),
            BundledIcon::X => include_bytes!("bundled_image_icons/x.png").to_vec(),
            BundledIcon::XCircle => include_bytes!("bundled_image_icons/x-circle.png").to_vec(),
            BundledIcon::XOctagon => include_bytes!("bundled_image_icons/x-octagon.png").to_vec(),
            BundledIcon::XSquare => include_bytes!("bundled_image_icons/x-square.png").to_vec(),
            BundledIcon::Youtube => include_bytes!("bundled_image_icons/youtube.png").to_vec(),
            BundledIcon::Zap => include_bytes!("bundled_image_icons/zap.png").to_vec(),
            BundledIcon::ZapOff => include_bytes!("bundled_image_icons/zap-off.png").to_vec(),
            BundledIcon::ZoomIn => include_bytes!("bundled_image_icons/zoom-in.png").to_vec(),
            BundledIcon::ZoomOut => include_bytes!("bundled_image_icons/zoom-out.png").to_vec(),
        }
    }
}
