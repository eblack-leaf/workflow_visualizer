use std::collections::{HashMap, HashSet};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{
    Bundle, Changed, Component, Or, Query, RemovedComponents, Res, Resource, Without,
};
use bevy_ecs::query::With;
use bevy_ecs::system::ResMut;

use crate::bundling::ResourceHandle;
use crate::images::renderer::{ImageBackend, ImageData, ImageFade, ImageOrientations};
use crate::{
    Animate, Animation, Area, Color, Disabled, EnableVisibility, IconScale, InterfaceContext,
    Interpolation, Layer, Orientation, Position, Section, Tag, Visibility,
};

pub type ImageTag = Tag<Image>;
#[derive(Bundle)]
pub struct Image {
    section: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    handle: ResourceHandle,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    color: Color,
    tag: ImageTag,
}
impl Image {
    pub fn new<IN: Into<ResourceHandle>, L: Into<Layer>, IF: Into<ImageFade>>(
        name: IN,
        layer: L,
        fade: IF,
    ) -> Self {
        Self {
            section: Section::default(),
            layer: layer.into(),
            visibility: EnableVisibility::default(),
            handle: name.into(),
            fade: fade.into(),
            cache: Cache::default(),
            difference: Difference::default(),
            color: ImageIcon::INVALID_COLOR,
            tag: ImageTag::new(),
        }
    }
}
#[derive(Component, Copy, Clone)]
pub struct AspectRatioAlignedDimension {
    pub dimension: Area<InterfaceContext>,
}
impl AspectRatioAlignedDimension {
    pub fn new<A: Into<Area<InterfaceContext>>>(dimension: A) -> Self {
        Self {
            dimension: dimension.into(),
        }
    }
}
impl Animate for AspectRatioAlignedDimension {
    fn interpolations(&self, end: &Self) -> Vec<Interpolation> {
        vec![
            Interpolation::new(end.dimension.width - self.dimension.width),
            Interpolation::new(end.dimension.height - self.dimension.height),
        ]
    }
}
pub(crate) fn apply_aspect_animations(
    mut anims: Query<(
        &mut AspectRatioAlignedDimension,
        &mut Animation<AspectRatioAlignedDimension>,
    )>,
) {
    for (mut dim, mut anim) in anims.iter_mut() {
        let extractions = anim.extractions();
        if let Some(extract) = extractions.get(0).unwrap() {
            dim.dimension.width += extract.0;
        }
        if let Some(extract) = extractions.get(1).unwrap() {
            dim.dimension.height += extract.0;
        }
    }
}
pub(crate) fn aspect_ratio_aligned_dimension(
    mut bound: Query<
        (
            &ResourceHandle,
            &AspectRatioAlignedDimension,
            &mut Area<InterfaceContext>,
        ),
        Or<(
            Changed<AspectRatioAlignedDimension>,
            Changed<Area<InterfaceContext>>,
            Changed<ResourceHandle>,
        )>,
    >,
    orientations: Res<ImageOrientations>,
) {
    for (name, max_dim, mut area) in bound.iter_mut() {
        let orientation = orientations.get(*name);
        let _dimensions_orientation = Orientation::new(max_dim.dimension.as_numerical());
        let mut attempted_width = max_dim.dimension.width;
        let mut attempted_height = attempted_width * orientation.value().reciprocal();
        while attempted_height > max_dim.dimension.height {
            attempted_width -= 1f32;
            attempted_height = attempted_width * orientation.value().reciprocal();
        }
        *area = Area::new(attempted_width, attempted_height);
    }
}
pub type ImageIconTag = Tag<ImageIcon>;
#[derive(Bundle)]
pub struct ImageIcon {
    section: Section<InterfaceContext>,
    layer: Layer,
    visibility: EnableVisibility,
    handle: ResourceHandle,
    fade: ImageFade,
    cache: Cache,
    difference: Difference,
    tag: ImageTag,
    image_icon_tag: ImageIconTag,
    scale: IconScale,
    color: Color,
}
pub enum BundledImageIcon {
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
impl BundledImageIcon {
    pub fn data(&self) -> ImageData {
        match &self {
            BundledImageIcon::Activity => {
                include_bytes!("bundled_image_icons/activity.png").to_vec()
            }
            BundledImageIcon::Airplay => include_bytes!("bundled_image_icons/airplay.png").to_vec(),
            BundledImageIcon::AlertCircle => {
                include_bytes!("bundled_image_icons/alert-circle.png").to_vec()
            }
            BundledImageIcon::AlignCenter => {
                include_bytes!("bundled_image_icons/align-center.png").to_vec()
            }
            BundledImageIcon::AlignJustify => {
                include_bytes!("bundled_image_icons/align-justify.png").to_vec()
            }
            BundledImageIcon::AlignLeft => {
                include_bytes!("bundled_image_icons/align-left.png").to_vec()
            }
            BundledImageIcon::AlertTriangle => {
                include_bytes!("bundled_image_icons/alert-triangle.png").to_vec()
            }
            BundledImageIcon::AlertOctagon => {
                include_bytes!("bundled_image_icons/alert-octagon.png").to_vec()
            }
            BundledImageIcon::AlignRight => {
                include_bytes!("bundled_image_icons/align-right.png").to_vec()
            }
            BundledImageIcon::ArrowRight => {
                include_bytes!("bundled_image_icons/arrow-right.png").to_vec()
            }
            BundledImageIcon::ArrowLeft => {
                include_bytes!("bundled_image_icons/arrow-left.png").to_vec()
            }
            BundledImageIcon::Anchor => include_bytes!("bundled_image_icons/anchor.png").to_vec(),
            BundledImageIcon::Aperture => {
                include_bytes!("bundled_image_icons/aperture.png").to_vec()
            }
            BundledImageIcon::Archive => include_bytes!("bundled_image_icons/archive.png").to_vec(),
            BundledImageIcon::ArrowDown => {
                include_bytes!("bundled_image_icons/arrow-down.png").to_vec()
            }
            BundledImageIcon::ArrowDownCircle => {
                include_bytes!("bundled_image_icons/arrow-down-circle.png").to_vec()
            }
            BundledImageIcon::ArrowDownLeft => {
                include_bytes!("bundled_image_icons/arrow-down-left.png").to_vec()
            }
            BundledImageIcon::ArrowDownRight => {
                include_bytes!("bundled_image_icons/arrow-down-right.png").to_vec()
            }
            BundledImageIcon::ArrowLeftCircle => {
                include_bytes!("bundled_image_icons/arrow-left-circle.png").to_vec()
            }
            BundledImageIcon::ArrowRightCircle => {
                include_bytes!("bundled_image_icons/arrow-right-circle.png").to_vec()
            }
            BundledImageIcon::ArrowUp => {
                include_bytes!("bundled_image_icons/arrow-up.png").to_vec()
            }
            BundledImageIcon::ArrowUpCircle => {
                include_bytes!("bundled_image_icons/arrow-up-circle.png").to_vec()
            }
            BundledImageIcon::ArrowUpLeft => {
                include_bytes!("bundled_image_icons/arrow-up-left.png").to_vec()
            }
            BundledImageIcon::ArrowUpRight => {
                include_bytes!("bundled_image_icons/arrow-up-right.png").to_vec()
            }
            BundledImageIcon::AtSign => include_bytes!("bundled_image_icons/at-sign.png").to_vec(),
            BundledImageIcon::Award => include_bytes!("bundled_image_icons/award.png").to_vec(),
            BundledImageIcon::BarChart => {
                include_bytes!("bundled_image_icons/bar-chart.png").to_vec()
            }
            BundledImageIcon::BarChart2 => {
                include_bytes!("bundled_image_icons/bar-chart-2.png").to_vec()
            }
            BundledImageIcon::Battery => include_bytes!("bundled_image_icons/battery.png").to_vec(),
            BundledImageIcon::BatteryCharging => {
                include_bytes!("bundled_image_icons/battery-charging.png").to_vec()
            }
            BundledImageIcon::Bell => include_bytes!("bundled_image_icons/bell.png").to_vec(),
            BundledImageIcon::BellOff => {
                include_bytes!("bundled_image_icons/bell-off.png").to_vec()
            }
            BundledImageIcon::Bluetooth => {
                include_bytes!("bundled_image_icons/bluetooth.png").to_vec()
            }
            BundledImageIcon::Bold => include_bytes!("bundled_image_icons/bold.png").to_vec(),
            BundledImageIcon::Book => include_bytes!("bundled_image_icons/book.png").to_vec(),
            BundledImageIcon::BookOpen => {
                include_bytes!("bundled_image_icons/book-open.png").to_vec()
            }
            BundledImageIcon::Bookmark => {
                include_bytes!("bundled_image_icons/bookmark.png").to_vec()
            }
            BundledImageIcon::Box => include_bytes!("bundled_image_icons/box.png").to_vec(),
            BundledImageIcon::Briefcase => {
                include_bytes!("bundled_image_icons/briefcase.png").to_vec()
            }
            BundledImageIcon::Calendar => {
                include_bytes!("bundled_image_icons/calendar.png").to_vec()
            }
            BundledImageIcon::Camera => include_bytes!("bundled_image_icons/camera.png").to_vec(),
            BundledImageIcon::CameraOff => {
                include_bytes!("bundled_image_icons/camera-off.png").to_vec()
            }
            BundledImageIcon::Cast => include_bytes!("bundled_image_icons/cast.png").to_vec(),
            BundledImageIcon::Check => include_bytes!("bundled_image_icons/check.png").to_vec(),
            BundledImageIcon::CheckCircle => {
                include_bytes!("bundled_image_icons/check-circle.png").to_vec()
            }
            BundledImageIcon::CheckSquare => {
                include_bytes!("bundled_image_icons/check-square.png").to_vec()
            }
            BundledImageIcon::ChevronDown => {
                include_bytes!("bundled_image_icons/chevron-down.png").to_vec()
            }
            BundledImageIcon::ChevronLeft => {
                include_bytes!("bundled_image_icons/chevron-left.png").to_vec()
            }
            BundledImageIcon::ChevronRight => {
                include_bytes!("bundled_image_icons/chevron-right.png").to_vec()
            }
            BundledImageIcon::ChevronUp => {
                include_bytes!("bundled_image_icons/chevron-up.png").to_vec()
            }
            BundledImageIcon::ChevronsDown => {
                include_bytes!("bundled_image_icons/chevrons-down.png").to_vec()
            }
            BundledImageIcon::ChevronsLeft => {
                include_bytes!("bundled_image_icons/chevrons-left.png").to_vec()
            }
            BundledImageIcon::ChevronsRight => {
                include_bytes!("bundled_image_icons/chevrons-right.png").to_vec()
            }
            BundledImageIcon::ChevronsUp => {
                include_bytes!("bundled_image_icons/chevrons-up.png").to_vec()
            }
            BundledImageIcon::Chrome => include_bytes!("bundled_image_icons/chrome.png").to_vec(),
            BundledImageIcon::Circle => include_bytes!("bundled_image_icons/circle.png").to_vec(),
            BundledImageIcon::Clipboard => {
                include_bytes!("bundled_image_icons/clipboard.png").to_vec()
            }
            BundledImageIcon::Clock => include_bytes!("bundled_image_icons/clock.png").to_vec(),
            BundledImageIcon::Cloud => include_bytes!("bundled_image_icons/cloud.png").to_vec(),
            BundledImageIcon::CloudDrizzle => {
                include_bytes!("bundled_image_icons/cloud-drizzle.png").to_vec()
            }
            BundledImageIcon::CloudLightning => {
                include_bytes!("bundled_image_icons/cloud-lightning.png").to_vec()
            }
            BundledImageIcon::CloudOff => {
                include_bytes!("bundled_image_icons/cloud-off.png").to_vec()
            }
            BundledImageIcon::CloudRain => {
                include_bytes!("bundled_image_icons/cloud-rain.png").to_vec()
            }
            BundledImageIcon::CloudSnow => {
                include_bytes!("bundled_image_icons/cloud-snow.png").to_vec()
            }
            BundledImageIcon::Code => include_bytes!("bundled_image_icons/code.png").to_vec(),
            BundledImageIcon::Codepen => include_bytes!("bundled_image_icons/codepen.png").to_vec(),
            BundledImageIcon::CodeSandbox => {
                include_bytes!("bundled_image_icons/codesandbox.png").to_vec()
            }
            BundledImageIcon::Coffee => include_bytes!("bundled_image_icons/coffee.png").to_vec(),
            BundledImageIcon::Columns => include_bytes!("bundled_image_icons/columns.png").to_vec(),
            BundledImageIcon::Command => include_bytes!("bundled_image_icons/command.png").to_vec(),
            BundledImageIcon::Compass => include_bytes!("bundled_image_icons/compass.png").to_vec(),
            BundledImageIcon::Copy => include_bytes!("bundled_image_icons/copy.png").to_vec(),
            BundledImageIcon::CornerDownLeft => {
                include_bytes!("bundled_image_icons/corner-down-left.png").to_vec()
            }
            BundledImageIcon::CornerDownRight => {
                include_bytes!("bundled_image_icons/corner-down-right.png").to_vec()
            }
            BundledImageIcon::CornerLeftDown => {
                include_bytes!("bundled_image_icons/corner-left-down.png").to_vec()
            }
            BundledImageIcon::CornerLeftUp => {
                include_bytes!("bundled_image_icons/corner-left-up.png").to_vec()
            }
            BundledImageIcon::CornerRightDown => {
                include_bytes!("bundled_image_icons/corner-right-down.png").to_vec()
            }
            BundledImageIcon::CornerRightUp => {
                include_bytes!("bundled_image_icons/corner-right-up.png").to_vec()
            }
            BundledImageIcon::CornerUpLeft => {
                include_bytes!("bundled_image_icons/corner-up-left.png").to_vec()
            }
            BundledImageIcon::CornerUpRight => {
                include_bytes!("bundled_image_icons/corner-up-right.png").to_vec()
            }
            BundledImageIcon::Cpu => include_bytes!("bundled_image_icons/cpu.png").to_vec(),
            BundledImageIcon::CreditCard => {
                include_bytes!("bundled_image_icons/credit-card.png").to_vec()
            }
            BundledImageIcon::Crop => include_bytes!("bundled_image_icons/crop.png").to_vec(),
            BundledImageIcon::Crosshair => {
                include_bytes!("bundled_image_icons/crosshair.png").to_vec()
            }
            BundledImageIcon::Database => {
                include_bytes!("bundled_image_icons/database.png").to_vec()
            }
            BundledImageIcon::Delete => include_bytes!("bundled_image_icons/delete.png").to_vec(),
            BundledImageIcon::Disc => include_bytes!("bundled_image_icons/disc.png").to_vec(),
            BundledImageIcon::Divide => include_bytes!("bundled_image_icons/divide.png").to_vec(),
            BundledImageIcon::DivideCircle => {
                include_bytes!("bundled_image_icons/divide-circle.png").to_vec()
            }
            BundledImageIcon::DivideSquare => {
                include_bytes!("bundled_image_icons/divide-square.png").to_vec()
            }
            BundledImageIcon::DollarSign => {
                include_bytes!("bundled_image_icons/dollar-sign.png").to_vec()
            }
            BundledImageIcon::Download => {
                include_bytes!("bundled_image_icons/download.png").to_vec()
            }
            BundledImageIcon::DownloadCloud => {
                include_bytes!("bundled_image_icons/download-cloud.png").to_vec()
            }
            BundledImageIcon::Dribble => {
                include_bytes!("bundled_image_icons/dribbble.png").to_vec()
            }
            BundledImageIcon::Droplet => include_bytes!("bundled_image_icons/droplet.png").to_vec(),
            BundledImageIcon::Edit => include_bytes!("bundled_image_icons/edit.png").to_vec(),
            BundledImageIcon::EditTwo => include_bytes!("bundled_image_icons/edit-2.png").to_vec(),
            BundledImageIcon::EditThree => {
                include_bytes!("bundled_image_icons/edit-3.png").to_vec()
            }
            BundledImageIcon::ExternalLink => {
                include_bytes!("bundled_image_icons/external-link.png").to_vec()
            }
            BundledImageIcon::Eye => include_bytes!("bundled_image_icons/eye.png").to_vec(),
            BundledImageIcon::EyeOff => include_bytes!("bundled_image_icons/eye-off.png").to_vec(),
            BundledImageIcon::Facebook => {
                include_bytes!("bundled_image_icons/facebook.png").to_vec()
            }
            BundledImageIcon::FastForward => {
                include_bytes!("bundled_image_icons/fast-forward.png").to_vec()
            }
            BundledImageIcon::Feather => include_bytes!("bundled_image_icons/feather.png").to_vec(),
            BundledImageIcon::Figma => include_bytes!("bundled_image_icons/figma.png").to_vec(),
            BundledImageIcon::File => include_bytes!("bundled_image_icons/file.png").to_vec(),
            BundledImageIcon::FileMinus => {
                include_bytes!("bundled_image_icons/file-minus.png").to_vec()
            }
            BundledImageIcon::FilePlus => {
                include_bytes!("bundled_image_icons/file-plus.png").to_vec()
            }
            BundledImageIcon::FileText => {
                include_bytes!("bundled_image_icons/file-text.png").to_vec()
            }
            BundledImageIcon::Film => include_bytes!("bundled_image_icons/film.png").to_vec(),
            BundledImageIcon::Filter => include_bytes!("bundled_image_icons/filter.png").to_vec(),
            BundledImageIcon::Flag => include_bytes!("bundled_image_icons/flag.png").to_vec(),
            BundledImageIcon::Folder => include_bytes!("bundled_image_icons/folder.png").to_vec(),
            BundledImageIcon::FolderMinus => {
                include_bytes!("bundled_image_icons/folder-minus.png").to_vec()
            }
            BundledImageIcon::FolderPlus => {
                include_bytes!("bundled_image_icons/folder-plus.png").to_vec()
            }
            BundledImageIcon::Framer => include_bytes!("bundled_image_icons/framer.png").to_vec(),
            BundledImageIcon::Frown => include_bytes!("bundled_image_icons/frown.png").to_vec(),
            BundledImageIcon::Gift => include_bytes!("bundled_image_icons/gift.png").to_vec(),
            BundledImageIcon::GitBranch => {
                include_bytes!("bundled_image_icons/git-branch.png").to_vec()
            }
            BundledImageIcon::GitCommit => {
                include_bytes!("bundled_image_icons/git-commit.png").to_vec()
            }
            BundledImageIcon::GitMerge => {
                include_bytes!("bundled_image_icons/git-merge.png").to_vec()
            }
            BundledImageIcon::GitPullRequest => {
                include_bytes!("bundled_image_icons/git-pull-request.png").to_vec()
            }
            BundledImageIcon::Github => include_bytes!("bundled_image_icons/github.png").to_vec(),
            BundledImageIcon::Gitlab => include_bytes!("bundled_image_icons/gitlab.png").to_vec(),
            BundledImageIcon::Globe => include_bytes!("bundled_image_icons/globe.png").to_vec(),
            BundledImageIcon::Grid => include_bytes!("bundled_image_icons/grid.png").to_vec(),
            BundledImageIcon::HardDrive => {
                include_bytes!("bundled_image_icons/hard-drive.png").to_vec()
            }
            BundledImageIcon::Hash => include_bytes!("bundled_image_icons/hash.png").to_vec(),
            BundledImageIcon::Headphones => {
                include_bytes!("bundled_image_icons/headphones.png").to_vec()
            }
            BundledImageIcon::Heart => include_bytes!("bundled_image_icons/heart.png").to_vec(),
            BundledImageIcon::HelpCircle => {
                include_bytes!("bundled_image_icons/help-circle.png").to_vec()
            }
            BundledImageIcon::Hexagon => include_bytes!("bundled_image_icons/hexagon.png").to_vec(),
            BundledImageIcon::Home => include_bytes!("bundled_image_icons/home.png").to_vec(),
            BundledImageIcon::Image => include_bytes!("bundled_image_icons/image.png").to_vec(),
            BundledImageIcon::Inbox => include_bytes!("bundled_image_icons/inbox.png").to_vec(),
            BundledImageIcon::Info => include_bytes!("bundled_image_icons/info.png").to_vec(),
            BundledImageIcon::Instagram => {
                include_bytes!("bundled_image_icons/instagram.png").to_vec()
            }
            BundledImageIcon::Italic => include_bytes!("bundled_image_icons/italic.png").to_vec(),
            BundledImageIcon::Key => include_bytes!("bundled_image_icons/key.png").to_vec(),
            BundledImageIcon::Layers => include_bytes!("bundled_image_icons/layers.png").to_vec(),
            BundledImageIcon::Layout => include_bytes!("bundled_image_icons/layout.png").to_vec(),
            BundledImageIcon::LifeBuoy => {
                include_bytes!("bundled_image_icons/life-buoy.png").to_vec()
            }
            BundledImageIcon::Link => include_bytes!("bundled_image_icons/link.png").to_vec(),
            BundledImageIcon::LinkTwo => include_bytes!("bundled_image_icons/link-2.png").to_vec(),
            BundledImageIcon::LinkedIn => {
                include_bytes!("bundled_image_icons/linkedin.png").to_vec()
            }
            BundledImageIcon::List => include_bytes!("bundled_image_icons/list.png").to_vec(),
            BundledImageIcon::Loader => include_bytes!("bundled_image_icons/loader.png").to_vec(),
            BundledImageIcon::Lock => include_bytes!("bundled_image_icons/lock.png").to_vec(),
            BundledImageIcon::LogIn => include_bytes!("bundled_image_icons/log-in.png").to_vec(),
            BundledImageIcon::LogOut => include_bytes!("bundled_image_icons/log-out.png").to_vec(),
            BundledImageIcon::Mail => include_bytes!("bundled_image_icons/mail.png").to_vec(),
            BundledImageIcon::Map => include_bytes!("bundled_image_icons/map.png").to_vec(),
            BundledImageIcon::MapPin => include_bytes!("bundled_image_icons/map-pin.png").to_vec(),
            BundledImageIcon::Maximize => {
                include_bytes!("bundled_image_icons/maximize.png").to_vec()
            }
            BundledImageIcon::MaximizeTwo => {
                include_bytes!("bundled_image_icons/maximize-2.png").to_vec()
            }
            BundledImageIcon::Meh => include_bytes!("bundled_image_icons/meh.png").to_vec(),
            BundledImageIcon::Menu => include_bytes!("bundled_image_icons/menu.png").to_vec(),
            BundledImageIcon::MessageCircle => {
                include_bytes!("bundled_image_icons/message-circle.png").to_vec()
            }
            BundledImageIcon::MessageSquare => {
                include_bytes!("bundled_image_icons/message-square.png").to_vec()
            }
            BundledImageIcon::Mic => include_bytes!("bundled_image_icons/mic.png").to_vec(),
            BundledImageIcon::MicOff => include_bytes!("bundled_image_icons/mic-off.png").to_vec(),
            BundledImageIcon::Minimize => {
                include_bytes!("bundled_image_icons/minimize.png").to_vec()
            }
            BundledImageIcon::MinimizeTwo => {
                include_bytes!("bundled_image_icons/minimize-2.png").to_vec()
            }
            BundledImageIcon::Minus => include_bytes!("bundled_image_icons/minus.png").to_vec(),
            BundledImageIcon::MinusCircle => {
                include_bytes!("bundled_image_icons/minus-circle.png").to_vec()
            }
            BundledImageIcon::MinusSquare => {
                include_bytes!("bundled_image_icons/minus-square.png").to_vec()
            }
            BundledImageIcon::Monitor => include_bytes!("bundled_image_icons/monitor.png").to_vec(),
            BundledImageIcon::Moon => include_bytes!("bundled_image_icons/moon.png").to_vec(),
            BundledImageIcon::MoreHorizontal => {
                include_bytes!("bundled_image_icons/more-horizontal.png").to_vec()
            }
            BundledImageIcon::MoreVertical => {
                include_bytes!("bundled_image_icons/more-vertical.png").to_vec()
            }
            BundledImageIcon::MousePointer => {
                include_bytes!("bundled_image_icons/mouse-pointer.png").to_vec()
            }
            BundledImageIcon::Move => include_bytes!("bundled_image_icons/move.png").to_vec(),
            BundledImageIcon::Music => include_bytes!("bundled_image_icons/music.png").to_vec(),
            BundledImageIcon::Navigation => {
                include_bytes!("bundled_image_icons/navigation.png").to_vec()
            }
            BundledImageIcon::NavigationTwo => {
                include_bytes!("bundled_image_icons/navigation-2.png").to_vec()
            }
            BundledImageIcon::Octagon => include_bytes!("bundled_image_icons/octagon.png").to_vec(),
            BundledImageIcon::Package => include_bytes!("bundled_image_icons/package.png").to_vec(),
            BundledImageIcon::Paperclip => {
                include_bytes!("bundled_image_icons/paperclip.png").to_vec()
            }
            BundledImageIcon::Pause => include_bytes!("bundled_image_icons/pause.png").to_vec(),
            BundledImageIcon::PauseCircle => {
                include_bytes!("bundled_image_icons/pause-circle.png").to_vec()
            }
            BundledImageIcon::PenTool => {
                include_bytes!("bundled_image_icons/pen-tool.png").to_vec()
            }
            BundledImageIcon::Percent => include_bytes!("bundled_image_icons/percent.png").to_vec(),
            BundledImageIcon::Phone => include_bytes!("bundled_image_icons/phone.png").to_vec(),
            BundledImageIcon::PhoneCall => {
                include_bytes!("bundled_image_icons/phone-call.png").to_vec()
            }
            BundledImageIcon::PhoneForwarded => {
                include_bytes!("bundled_image_icons/phone-forwarded.png").to_vec()
            }
            BundledImageIcon::PhoneIncoming => {
                include_bytes!("bundled_image_icons/phone-incoming.png").to_vec()
            }
            BundledImageIcon::PhoneOff => {
                include_bytes!("bundled_image_icons/phone-off.png").to_vec()
            }
            BundledImageIcon::PhoneOutgoing => {
                include_bytes!("bundled_image_icons/phone-outgoing.png").to_vec()
            }
            BundledImageIcon::PieChart => {
                include_bytes!("bundled_image_icons/pie-chart.png").to_vec()
            }
            BundledImageIcon::Play => include_bytes!("bundled_image_icons/play.png").to_vec(),
            BundledImageIcon::PlayCircle => {
                include_bytes!("bundled_image_icons/play-circle.png").to_vec()
            }
            BundledImageIcon::Plus => include_bytes!("bundled_image_icons/plus.png").to_vec(),
            BundledImageIcon::PlusCircle => {
                include_bytes!("bundled_image_icons/plus-circle.png").to_vec()
            }
            BundledImageIcon::PlusSquare => {
                include_bytes!("bundled_image_icons/plus-square.png").to_vec()
            }
            BundledImageIcon::Pocket => include_bytes!("bundled_image_icons/pocket.png").to_vec(),
            BundledImageIcon::Power => include_bytes!("bundled_image_icons/power.png").to_vec(),
            BundledImageIcon::Printer => include_bytes!("bundled_image_icons/printer.png").to_vec(),
            BundledImageIcon::Radio => include_bytes!("bundled_image_icons/radio.png").to_vec(),
            BundledImageIcon::RefreshCCW => {
                include_bytes!("bundled_image_icons/refresh-ccw.png").to_vec()
            }
            BundledImageIcon::RefreshCW => {
                include_bytes!("bundled_image_icons/refresh-cw.png").to_vec()
            }
            BundledImageIcon::Repeat => include_bytes!("bundled_image_icons/repeat.png").to_vec(),
            BundledImageIcon::Rewind => include_bytes!("bundled_image_icons/rewind.png").to_vec(),
            BundledImageIcon::RotateCCW => {
                include_bytes!("bundled_image_icons/rotate-ccw.png").to_vec()
            }
            BundledImageIcon::RotateCW => {
                include_bytes!("bundled_image_icons/rotate-cw.png").to_vec()
            }
            BundledImageIcon::RSS => include_bytes!("bundled_image_icons/rss.png").to_vec(),
            BundledImageIcon::Save => include_bytes!("bundled_image_icons/save.png").to_vec(),
            BundledImageIcon::Scissors => {
                include_bytes!("bundled_image_icons/scissors.png").to_vec()
            }
            BundledImageIcon::Search => include_bytes!("bundled_image_icons/search.png").to_vec(),
            BundledImageIcon::Send => include_bytes!("bundled_image_icons/send.png").to_vec(),
            BundledImageIcon::Server => include_bytes!("bundled_image_icons/server.png").to_vec(),
            BundledImageIcon::Settings => {
                include_bytes!("bundled_image_icons/settings.png").to_vec()
            }
            BundledImageIcon::Share => include_bytes!("bundled_image_icons/share.png").to_vec(),
            BundledImageIcon::ShareTwo => {
                include_bytes!("bundled_image_icons/share-2.png").to_vec()
            }
            BundledImageIcon::Shield => include_bytes!("bundled_image_icons/shield.png").to_vec(),
            BundledImageIcon::ShieldOff => {
                include_bytes!("bundled_image_icons/shield-off.png").to_vec()
            }
            BundledImageIcon::ShoppingBag => {
                include_bytes!("bundled_image_icons/shopping-bag.png").to_vec()
            }
            BundledImageIcon::ShoppingCart => {
                include_bytes!("bundled_image_icons/shopping-cart.png").to_vec()
            }
            BundledImageIcon::Shuffle => include_bytes!("bundled_image_icons/shuffle.png").to_vec(),
            BundledImageIcon::Sidebar => include_bytes!("bundled_image_icons/sidebar.png").to_vec(),
            BundledImageIcon::SkipBack => {
                include_bytes!("bundled_image_icons/skip-back.png").to_vec()
            }
            BundledImageIcon::SkipForward => {
                include_bytes!("bundled_image_icons/skip-forward.png").to_vec()
            }
            BundledImageIcon::Slack => include_bytes!("bundled_image_icons/slack.png").to_vec(),
            BundledImageIcon::Slash => include_bytes!("bundled_image_icons/slash.png").to_vec(),
            BundledImageIcon::Sliders => include_bytes!("bundled_image_icons/sliders.png").to_vec(),
            BundledImageIcon::Smartphone => {
                include_bytes!("bundled_image_icons/smartphone.png").to_vec()
            }
            BundledImageIcon::Smile => include_bytes!("bundled_image_icons/smile.png").to_vec(),
            BundledImageIcon::Speaker => include_bytes!("bundled_image_icons/speaker.png").to_vec(),
            BundledImageIcon::Square => include_bytes!("bundled_image_icons/square.png").to_vec(),
            BundledImageIcon::Star => include_bytes!("bundled_image_icons/star.png").to_vec(),
            BundledImageIcon::StopCircle => {
                include_bytes!("bundled_image_icons/stop-circle.png").to_vec()
            }
            BundledImageIcon::Sun => include_bytes!("bundled_image_icons/sun.png").to_vec(),
            BundledImageIcon::Sunrise => include_bytes!("bundled_image_icons/sunrise.png").to_vec(),
            BundledImageIcon::Sunset => include_bytes!("bundled_image_icons/sunset.png").to_vec(),
            BundledImageIcon::Table => include_bytes!("bundled_image_icons/table.png").to_vec(),
            BundledImageIcon::Tablet => include_bytes!("bundled_image_icons/tablet.png").to_vec(),
            BundledImageIcon::Tag => include_bytes!("bundled_image_icons/tag.png").to_vec(),
            BundledImageIcon::Target => include_bytes!("bundled_image_icons/target.png").to_vec(),
            BundledImageIcon::Terminal => {
                include_bytes!("bundled_image_icons/terminal.png").to_vec()
            }
            BundledImageIcon::Thermometer => {
                include_bytes!("bundled_image_icons/thermometer.png").to_vec()
            }
            BundledImageIcon::ThumbsDown => {
                include_bytes!("bundled_image_icons/thumbs-down.png").to_vec()
            }
            BundledImageIcon::ThumbsUp => {
                include_bytes!("bundled_image_icons/thumbs-up.png").to_vec()
            }
            BundledImageIcon::ToggleLeft => {
                include_bytes!("bundled_image_icons/toggle-left.png").to_vec()
            }
            BundledImageIcon::ToggleRight => {
                include_bytes!("bundled_image_icons/toggle-right.png").to_vec()
            }
            BundledImageIcon::Tool => include_bytes!("bundled_image_icons/tool.png").to_vec(),
            BundledImageIcon::Trash => include_bytes!("bundled_image_icons/trash.png").to_vec(),
            BundledImageIcon::TrashTwo => {
                include_bytes!("bundled_image_icons/trash-2.png").to_vec()
            }
            BundledImageIcon::Trello => include_bytes!("bundled_image_icons/trello.png").to_vec(),
            BundledImageIcon::TrendingDown => {
                include_bytes!("bundled_image_icons/trending-down.png").to_vec()
            }
            BundledImageIcon::TrendingUp => {
                include_bytes!("bundled_image_icons/trending-up.png").to_vec()
            }
            BundledImageIcon::Triangle => {
                include_bytes!("bundled_image_icons/triangle.png").to_vec()
            }
            BundledImageIcon::Truck => include_bytes!("bundled_image_icons/truck.png").to_vec(),
            BundledImageIcon::TV => include_bytes!("bundled_image_icons/tv.png").to_vec(),
            BundledImageIcon::Twitch => include_bytes!("bundled_image_icons/twitch.png").to_vec(),
            BundledImageIcon::Twitter => include_bytes!("bundled_image_icons/twitter.png").to_vec(),
            BundledImageIcon::Type => include_bytes!("bundled_image_icons/type.png").to_vec(),
            BundledImageIcon::Umbrella => {
                include_bytes!("bundled_image_icons/umbrella.png").to_vec()
            }
            BundledImageIcon::Underline => {
                include_bytes!("bundled_image_icons/underline.png").to_vec()
            }
            BundledImageIcon::Unlock => include_bytes!("bundled_image_icons/unlock.png").to_vec(),
            BundledImageIcon::Upload => include_bytes!("bundled_image_icons/upload.png").to_vec(),
            BundledImageIcon::UploadCloud => {
                include_bytes!("bundled_image_icons/upload-cloud.png").to_vec()
            }
            BundledImageIcon::User => include_bytes!("bundled_image_icons/user.png").to_vec(),
            BundledImageIcon::UserCheck => {
                include_bytes!("bundled_image_icons/user-check.png").to_vec()
            }
            BundledImageIcon::UserMinus => {
                include_bytes!("bundled_image_icons/user-minus.png").to_vec()
            }
            BundledImageIcon::UserPlus => {
                include_bytes!("bundled_image_icons/user-plus.png").to_vec()
            }
            BundledImageIcon::UserX => include_bytes!("bundled_image_icons/user-x.png").to_vec(),
            BundledImageIcon::Users => include_bytes!("bundled_image_icons/users.png").to_vec(),
            BundledImageIcon::Video => include_bytes!("bundled_image_icons/video.png").to_vec(),
            BundledImageIcon::VideoOff => {
                include_bytes!("bundled_image_icons/video-off.png").to_vec()
            }
            BundledImageIcon::Voicemail => {
                include_bytes!("bundled_image_icons/voicemail.png").to_vec()
            }
            BundledImageIcon::Volume => include_bytes!("bundled_image_icons/volume.png").to_vec(),
            BundledImageIcon::VolumeOne => {
                include_bytes!("bundled_image_icons/volume-1.png").to_vec()
            }
            BundledImageIcon::VolumeTwo => {
                include_bytes!("bundled_image_icons/volume-2.png").to_vec()
            }
            BundledImageIcon::VolumeX => {
                include_bytes!("bundled_image_icons/volume-x.png").to_vec()
            }
            BundledImageIcon::Watch => include_bytes!("bundled_image_icons/watch.png").to_vec(),
            BundledImageIcon::Wifi => include_bytes!("bundled_image_icons/wifi.png").to_vec(),
            BundledImageIcon::WifiOff => {
                include_bytes!("bundled_image_icons/wifi-off.png").to_vec()
            }
            BundledImageIcon::Wind => include_bytes!("bundled_image_icons/wind.png").to_vec(),
            BundledImageIcon::X => include_bytes!("bundled_image_icons/x.png").to_vec(),
            BundledImageIcon::XCircle => {
                include_bytes!("bundled_image_icons/x-circle.png").to_vec()
            }
            BundledImageIcon::XOctagon => {
                include_bytes!("bundled_image_icons/x-octagon.png").to_vec()
            }
            BundledImageIcon::XSquare => {
                include_bytes!("bundled_image_icons/x-square.png").to_vec()
            }
            BundledImageIcon::Youtube => include_bytes!("bundled_image_icons/youtube.png").to_vec(),
            BundledImageIcon::Zap => include_bytes!("bundled_image_icons/zap.png").to_vec(),
            BundledImageIcon::ZapOff => include_bytes!("bundled_image_icons/zap-off.png").to_vec(),
            BundledImageIcon::ZoomIn => include_bytes!("bundled_image_icons/zoom-in.png").to_vec(),
            BundledImageIcon::ZoomOut => {
                include_bytes!("bundled_image_icons/zoom-out.png").to_vec()
            }
        }
    }
}
impl ImageIcon {
    pub fn new<RH: Into<ResourceHandle>, IS: Into<IconScale>, L: Into<Layer>, C: Into<Color>>(
        handle: RH,
        scale: IS,
        layer: L,
        color: C,
    ) -> Self {
        Self {
            handle: handle.into(),
            scale: scale.into(),
            layer: layer.into(),
            color: color.into(),
            fade: ImageFade::OPAQUE,
            cache: Cache::default(),
            difference: Difference::default(),
            tag: ImageTag::new(),
            image_icon_tag: ImageIconTag::new(),
            visibility: EnableVisibility::new(),
            section: Section::default(),
        }
    }
    pub(crate) const INVALID_COLOR: Color = Color {
        red: -1.0,
        green: -1.0,
        blue: -1.0,
        alpha: -1.0,
    };
}
#[derive(Component, Default)]
pub(crate) struct Cache {
    pub(crate) name: Option<ResourceHandle>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) icon_color: Option<Color>,
}

#[derive(Component, Clone, Default)]
pub(crate) struct Difference {
    pub(crate) name: Option<ResourceHandle>,
    pub(crate) fade: Option<ImageFade>,
    pub(crate) pos: Option<Position<InterfaceContext>>,
    pub(crate) area: Option<Area<InterfaceContext>>,
    pub(crate) layer: Option<Layer>,
    pub(crate) icon_color: Option<Color>,
}
pub(crate) fn set_from_scale(
    mut image_icons: Query<(&IconScale, &mut Area<InterfaceContext>), Changed<IconScale>>,
) {
    for (scale, mut area) in image_icons.iter_mut() {
        area.width = scale.width();
        area.height = scale.height();
    }
}
pub(crate) fn icon_color_diff(
    mut image_icons: Query<
        (&Color, &mut Cache, &mut Difference),
        (Changed<Color>, With<ImageIconTag>),
    >,
) {
    for (color, mut cache, mut difference) in image_icons.iter_mut() {
        if let Some(cached) = cache.icon_color.as_ref() {
            if *cached != *color {
                difference.icon_color.replace(*color);
            }
        }
        cache.icon_color.replace(*color);
    }
}
pub(crate) fn name_diff(
    mut images: Query<(&ResourceHandle, &mut Cache, &mut Difference), Changed<ResourceHandle>>,
) {
    for (name, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.name.as_ref() {
            if cached.0 != name.0 {
                difference.name.replace(*name);
            }
        }
        cache.name.replace(*name);
    }
}
pub(crate) fn fade_diff(
    mut images: Query<(&ImageFade, &mut Cache, &mut Difference), Changed<ImageFade>>,
) {
    for (fade, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.fade.as_ref() {
            if *cached != *fade {
                difference.fade.replace(*fade);
            }
        }
        cache.fade.replace(*fade);
    }
}
pub(crate) fn pos_diff(
    mut images: Query<
        (&Position<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Position<InterfaceContext>>,
    >,
) {
    for (pos, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.pos.as_ref() {
            if *cached != *pos {
                difference.pos.replace(*pos);
            }
        }
        cache.pos.replace(*pos);
    }
}
pub(crate) fn area_diff(
    mut images: Query<
        (&Area<InterfaceContext>, &mut Cache, &mut Difference),
        Changed<Area<InterfaceContext>>,
    >,
) {
    for (area, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.area.as_ref() {
            if *cached != *area {
                difference.area.replace(*area);
            }
        }
        cache.area.replace(*area);
    }
}
pub(crate) fn layer_diff(mut images: Query<(&Layer, &mut Cache, &mut Difference), Changed<Layer>>) {
    for (layer, mut cache, mut difference) in images.iter_mut() {
        if let Some(cached) = cache.layer.as_ref() {
            if *cached != *layer {
                difference.layer.replace(*layer);
            }
        }
        cache.layer.replace(*layer);
    }
}
#[derive(Resource, Default)]
pub(crate) struct Extraction {
    pub(crate) differences: HashMap<Entity, Difference>,
    pub(crate) queued_remove: HashSet<Entity>,
}
impl Extraction {
    pub(crate) fn remove(&mut self, entity: Entity) {
        self.queued_remove.insert(entity);
        self.differences.remove(&entity);
    }
}
pub(crate) fn management(
    mut images: Query<
        (
            Entity,
            &Position<InterfaceContext>,
            &Area<InterfaceContext>,
            &Layer,
            &ResourceHandle,
            &ImageFade,
            &Visibility,
            &mut Cache,
            &mut Difference,
            &Color,
            Option<&ImageIconTag>,
        ),
        Changed<Visibility>,
    >,
    mut removed: RemovedComponents<ImageTag>,
    mut extraction: ResMut<Extraction>,
) {
    for (
        entity,
        pos,
        area,
        layer,
        name,
        fade,
        visibility,
        mut cache,
        mut difference,
        icon_color,
        image_icon,
    ) in images.iter_mut()
    {
        if visibility.visible() {
            cache.pos.replace(*pos);
            cache.area.replace(*area);
            cache.layer.replace(*layer);
            cache.name.replace(*name);
            cache.fade.replace(*fade);
            difference.pos.replace(cache.pos.unwrap());
            difference.area.replace(cache.area.unwrap());
            difference.layer.replace(cache.layer.unwrap());
            difference.fade.replace(cache.fade.unwrap());
            difference.name.replace(cache.name.unwrap());
            if image_icon.is_some() {
                cache.icon_color.replace(*icon_color);
                difference.icon_color.replace(*icon_color);
            }
        } else {
            extraction.remove(entity);
        }
    }
    for entity in removed.iter() {
        extraction.remove(entity);
    }
}
pub(crate) fn extract(
    mut extraction: ResMut<Extraction>,
    mut images: Query<
        (Entity, &mut Difference, &Visibility),
        (Changed<Difference>, Without<Disabled>),
    >,
) {
    for (entity, mut diff, visibility) in images.iter_mut() {
        if visibility.visible() {
            extraction.differences.insert(entity, diff.clone());
        }
        *diff = Difference::default();
    }
}
