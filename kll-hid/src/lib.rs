// Copyright 2021-2022 Jacob Alexander
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]

/// HID Locales
/// Locales defined by the USB HID Spec v1.11
/// <http://www.usb.org/developers/hidpage/HID1_11.pdf> (6.2.1) HID Descriptor
/// 36-255 are reserved
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, defmt::Format)]
#[repr(u8)]
pub enum Locale {
    Undefined = 0,
    Arabic = 1,
    Belgian = 2,
    CanadianBilingual = 3,
    CanadianFrench = 4,
    CzechRepublic = 5,
    Danish = 6,
    Finnish = 7,
    French = 8,
    German = 9,
    Greek = 10,
    Hebrew = 11,
    Hungary = 12,
    InternationalISO = 13,
    Italian = 14,
    JapanKatakana = 15,
    Korean = 16,
    LatinAmerica = 17,
    NetherlandsDutch = 18,
    Norwegian = 19,
    PersianFarsi = 20,
    Poland = 21,
    Portuguese = 22,
    Russia = 23,
    Slovakia = 24,
    Spanish = 25,
    Swedish = 26,
    SwissFrench = 27,
    SwissGerman = 28,
    Switzerland = 29,
    Taiwan = 30,
    TurkishQ = 31,
    UK = 32,
    US = 33,
    Yugoslavia = 34,
    TurkishF = 35,
}

/// HID Keyboard Codes
/// List of Keycodes - USB HID 1.12v2 pg 53
/// 0xA5 to 0xAF are reserved
/// 0xDE to 0xDF are reserved
/// 0xE8 to 0xFF are reserved
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, defmt::Format)]
#[repr(u8)]
pub enum Keyboard {
    NoEvent = 0x00,
    ErrorRollOver = 0x01,
    PostFail = 0x02,
    ErrorUndefined = 0x03,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    _1 = 0x1E,
    _2 = 0x1F,
    _3 = 0x20,
    _4 = 0x21,
    _5 = 0x22,
    _6 = 0x23,
    _7 = 0x24,
    _8 = 0x25,
    _9 = 0x26,
    _0 = 0x27,
    Enter = 0x28,
    Esc = 0x29,
    Backspace = 0x2A,
    Tab = 0x2B,
    Space = 0x2C,
    Minus = 0x2D,
    Equal = 0x2E,
    LeftBracket = 0x2F,
    RightBracket = 0x30,
    Backslash = 0x31,
    Number = 0x32,
    Semicolon = 0x33,
    Quote = 0x34,
    Backtick = 0x35,
    Comma = 0x36,
    Period = 0x37,
    Slash = 0x38,
    CapsLock = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,
    PrintScreen = 0x46,
    ScrollLock = 0x47,
    Pause = 0x48,
    Insert = 0x49,
    Home = 0x4A,
    PageUp = 0x4B,
    Delete = 0x4C,
    End = 0x4D,
    PageDown = 0x4E,
    Right = 0x4F,
    Left = 0x50,
    Down = 0x51,
    Up = 0x52,
    NumLock = 0x53,
    KeypadSlash = 0x54,
    KeypadAsterisk = 0x55,
    KeypadMinus = 0x56,
    KeypadPlus = 0x57,
    KeypadEnter = 0x58,
    Keypad1 = 0x59,
    Keypad2 = 0x5A,
    Keypad3 = 0x5B,
    Keypad4 = 0x5C,
    Keypad5 = 0x5D,
    Keypad6 = 0x5E,
    Keypad7 = 0x5F,
    Keypad8 = 0x60,
    Keypad9 = 0x61,
    Keypad0 = 0x62,
    KeypadPeriod = 0x63,
    ISOSlash = 0x64,
    App = 0x65,
    KeyboardStatus = 0x66,
    KeypadEqual = 0x67,
    F13 = 0x68,
    F14 = 0x69,
    F15 = 0x6A,
    F16 = 0x6B,
    F17 = 0x6C,
    F18 = 0x6D,
    F19 = 0x6E,
    F20 = 0x6F,
    F21 = 0x70,
    F22 = 0x71,
    F23 = 0x72,
    F24 = 0x73,
    Exec = 0x74,
    Help = 0x75,
    Menu = 0x76,
    Select = 0x77,
    Stop = 0x78,
    Again = 0x79,
    Undo = 0x7A,
    Cut = 0x7B,
    Copy = 0x7C,
    Paste = 0x7D,
    Find = 0x7E,
    Mute = 0x7F,
    VolumeUp = 0x80,
    VolumeDown = 0x81,
    LockingCapsLock = 0x82,
    LockingNumLock = 0x83,
    LockingScrollLock = 0x84,
    KeypadComma = 0x85,
    KeypadEqualAS400 = 0x86,
    International1 = 0x87,
    International2 = 0x88,
    International3 = 0x89,
    International4 = 0x8A,
    International5 = 0x8B,
    International6 = 0x8C,
    International7 = 0x8D,
    International8 = 0x8E,
    International9 = 0x8F,
    LANG1 = 0x90,
    LANG2 = 0x91,
    LANG3 = 0x92,
    LANG4 = 0x93,
    LANG5 = 0x94,
    LANG6 = 0x95,
    LANG7 = 0x96,
    LANG8 = 0x97,
    LANG9 = 0x98,
    AlternateErase = 0x99,
    SysReq = 0x9A,
    Cancel = 0x9B,
    Clear = 0x9C,
    Prior = 0x9D,
    Return = 0x9E,
    Separator = 0x9F,
    Out = 0xA0,
    Oper = 0xA1,
    ClearAgain = 0xA2,
    CrSelProps = 0xA3,
    ExSel = 0xA4,

    Keypad00 = 0xB0,
    Keypad000 = 0xB1,
    ThousandSeparator = 0xB2,
    DecimalSeparator = 0xB3,
    CurrencyUnit = 0xB4,
    CurrencySubUnit = 0xB5,
    KeypadLeftParenthesis = 0xB6,
    KeypadRightParenthesis = 0xB7,
    KeypadLeftBrace = 0xB8,
    KeypadRightBrace = 0xB9,
    KeypadTab = 0xBA,
    KeypadBackspace = 0xBB,
    KeypadA = 0xBC,
    KeypadB = 0xBD,
    KeypadC = 0xBE,
    KeypadD = 0xBF,
    KeypadE = 0xC0,
    KeypadF = 0xC1,
    KeypadXOR = 0xC2,
    KeypadChevron = 0xC3,
    KeypadPercent = 0xC4,
    KeypadLessThan = 0xC5,
    KeypadGreaterThan = 0xC6,
    KeypadBITAND = 0xC7,
    KeypadAND = 0xC8,
    KeypadBITOR = 0xC9,
    KeypadOR = 0xCA,
    KeypadColon = 0xCB,
    KeypadNumber = 0xCC,
    KeypadSpace = 0xCD,
    KeypadAt = 0xCE,
    KeypadExclamation = 0xCF,
    KeypadMemoryStore = 0xD0,
    KeypadMemoryRecall = 0xD1,
    KeypadMemoryClear = 0xD2,
    KeypadMemoryAdd = 0xD3,
    KeypadMemorySubtract = 0xD4,
    KeypadMemoryMultiply = 0xD5,
    KeypadMemoryDivide = 0xD6,
    KeypadPlusMinus = 0xD7,
    KeypadClear = 0xD8,
    KeypadClearEntry = 0xD9,
    KeypadBinary = 0xDA,
    KeypadOctal = 0xDB,
    KeypadDecimal = 0xDC,
    KeypadHexidecimal = 0xDD,

    LeftControl = 0xE0,
    LeftShift = 0xE1,
    LeftAlt = 0xE2,
    LeftGUI = 0xE3,
    RightControl = 0xE4,
    RightShift = 0xE5,
    RightAlt = 0xE6,
    RightGUI = 0xE7,
}

/// Conversion from u16 indexes to Keyboard enum
/// # Safety
impl From<u16> for Keyboard {
    fn from(index: u16) -> Keyboard {
        unsafe { core::mem::transmute(index as u8) }
    }
}

/// Conversion from Keyboard enum to u16
/// # Safety
impl From<Keyboard> for u16 {
    fn from(index: Keyboard) -> u16 {
        unsafe { core::mem::transmute(index as u16) }
    }
}

/// Conversion from u8 indexes to LedIndicator enum
/// # Safety
impl From<u8> for LedIndicator {
    fn from(index: u8) -> LedIndicator {
        unsafe { core::mem::transmute(index as u8) }
    }
}

/// Conversion from LedIndicator enum to u8
/// # Safety
impl From<LedIndicator> for u8 {
    fn from(index: LedIndicator) -> u8 {
        unsafe { core::mem::transmute(index as u8) }
    }
}

/// HID LED Indicators
/// List of LED codes - USB HID 1.12v2 pg 61
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, defmt::Format)]
#[repr(u8)]
pub enum LedIndicator {
    Undefined = 0x00,
    NumLock = 0x01,
    CapsLock = 0x02,
    ScrollLock = 0x03,
    Compose = 0x04,
    Kana = 0x05,
    Power = 0x06,
    Shift = 0x07,
    DoNotDisturb = 0x08,
    Mute = 0x09,
    ToneEnable = 0x0A,
    HighCutFilter = 0x0B,
    LowCutFilter = 0x0C,
    EqualizerEnable = 0x0D,
    SoundFieldOn = 0x0E,
    SurroundOn = 0x0F,
    Repeat = 0x10,
    Stereo = 0x11,
    SampleRateDetect = 0x12,
    Spinning = 0x13,
    CAC = 0x14,
    CLV = 0x15,
    RecordingFormatDetect = 0x16,
    OffHook = 0x17,
    Ring = 0x18,
    MessageWaiting = 0x19,
    DataMode = 0x1A,
    BatteryOperation = 0x1B,
    BatteryOK = 0x1C,
    BatteryLow = 0x1D,
    Speaker = 0x1E,
    HeadSet = 0x1F,
    Hold = 0x20,
    Microphone = 0x21,
    Coverage = 0x22,
    NightMode = 0x23,
    SendCalls = 0x24,
    CallPickup = 0x25,
    Conference = 0x26,
    StandBy = 0x27,
    CameraOn = 0x28,
    CameraOff = 0x29,
    OnLine = 0x2A,
    OffLine = 0x2B,
    Busy = 0x2C,
    Ready = 0x2D,
    PaperOut = 0x2E,
    PaperJam = 0x2F,
    Remote = 0x30,
    Forward = 0x31,
    Reverse = 0x32,
    Stop = 0x33,
    Rewind = 0x34,
    FastForward = 0x35,
    Play = 0x36,
    Pause = 0x37,
    Record = 0x38,
    Error = 0x39,

    GenericInd = 0x4B,
    SysSuspend = 0x4C,
    ExtPwrConn = 0x4D,
}

/// HID System Controls
/// List of System Controls - USB HID 1.12v2 pg 32
/// NKRO HID Supports 0x81 - 0xB7
/// 0x94 - 0x9F Reserved
/// 0xA9 - 0xAF Reserved
/// 0xB8 - 0xFFFF Reserved
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, defmt::Format)]
#[repr(u8)]
pub enum SystemControl {
    PowerDown = 0x81,
    Sleep = 0x82,
    WakeUp = 0x83,
    ContextMenu = 0x84,
    MainMenu = 0x85,
    AppMenu = 0x86,
    MenuHelp = 0x87,
    MenuExit = 0x88,
    MenuSelect = 0x89,
    MenuRight = 0x8A,
    MenuLeft = 0x8B,
    MenuUp = 0x8C,
    MenuDown = 0x8D,
    ColdRestart = 0x8E,
    WarmRestart = 0x8F,
    DpadUp = 0x90,
    DpadDown = 0x91,
    DpadRight = 0x92,
    DpadLeft = 0x93,

    SystemFunctionShift = 0x97,
    SystemFunctionShiftLock = 0x98,

    SystemDismissNotification = 0x9A,
    SystemDoNotDisturb = 0x9B,

    Dock = 0xA0,
    Undock = 0xA1,
    Setup = 0xA2,
    Break = 0xA3,
    DebuggerBreak = 0xA4,
    ApplicationBreak = 0xA5,
    ApplicationDebuggerBreak = 0xA6,
    SpeakerMute = 0xA7,
    Hibernate = 0xA8,

    DisplayInvert = 0xB0,
    DisplayInternal = 0xB1,
    DisplayExternal = 0xB2,
    DisplayBoth = 0xB3,
    DisplayDual = 0xB4,
    DisplayToggleInternalExternal = 0xB5,
    DisplaySwapPrimarySecondary = 0xB6,
    DisplayLCDAutoscale = 0xB7,
}

/// HID Consumer Controls
/// List of Consumer Codes - USB HID 1.12v2
/// NKRO HID Supports 0x020 - 0x29C
/// 0x023 - 0x02F Reserved
/// 0x037 - 0x03F Reserved
/// 0x049 - 0x05F Reserved
/// 0x067 - 0x06E Reserved?
/// 0x076 - 0x07F Reserved
/// 0x09F Reserved
/// 0x0A5 - 0x0AF Reserved
/// 0x0CF - 0x0DF Reserved
/// 0x0EB - 0x0EF Reserved
/// 0x0F6 - 0x0FF
/// 0x10E - 0x14F Reserved
/// 0x156 - 0x15F Reserved
/// Application Launch Buttons pg 79
/// Generic GUI Application Controls pg 82
/// TODO: Where does 0x29D come from?
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, defmt::Format)]
#[repr(u16)]
pub enum ConsumerControl {
    _10 = 0x020,
    _100 = 0x021,
    AMPM = 0x022,

    Power = 0x030,
    Reset = 0x031,
    Sleep = 0x032,
    SleepAfter = 0x033,
    SleepMode = 0x034,
    Illumination = 0x035,

    Menu = 0x040,
    MenuPick = 0x041,
    MenuUp = 0x042,
    MenuDown = 0x043,
    MenuLeft = 0x044,
    MenuRight = 0x045,
    MenuEscape = 0x046,
    MenuValueIncrease = 0x047,
    MenuValueDecrease = 0x048,

    DataOnScreen = 0x060,
    ClosedCaption = 0x061,
    ClosedCaptionSelect = 0x062,
    VCRTV = 0x063,
    BroadcastMode = 0x064,
    Snapshot = 0x065,
    Still = 0x066,

    BrightnessIncrement = 0x06F,
    BrightnessDecrement = 0x070,

    BacklightToggle = 0x072,
    BrightnessMin = 0x073,
    BrightnessMax = 0x074,
    BrightnessAuto = 0x075,

    AssignSelection = 0x081,
    ModeStep = 0x082,
    RecallLast = 0x083,
    EnterChannel = 0x084,
    OrderMovie = 0x085,

    MediaComputer = 0x088,
    MediaTV = 0x089,
    MediaWWW = 0x08A,
    MediaDVD = 0x08B,
    MediaTelephone = 0x08C,
    MediaProgramGuide = 0x08D,
    MediaVideoPhone = 0x08E,
    MediaSelectGames = 0x08F,
    MediaSelectMessages = 0x090,
    MediaSelectCD = 0x091,
    MediaSelectVCR = 0x092,
    MediaSelectTuner = 0x093,
    Quit = 0x094,
    Help = 0x095,
    MediaSelectTape = 0x096,
    MediaSelectCable = 0x097,
    MediaSelectSatellite = 0x098,
    MediaSelectSecurity = 0x099,
    MediaSelectHome = 0x09A,
    MediaSelectCall = 0x09B,
    ChannelIncrement = 0x09C,
    CahnnelDecrement = 0x09D,
    MediaSelectSAP = 0x09E,

    VCRPlus = 0x0A0,
    Once = 0x0A1,
    Daily = 0x0A2,
    Weekly = 0x0A3,
    Monthly = 0x0A4,

    Play = 0x0B0,
    Pause = 0x0B1,
    Record = 0x0B2,
    FastForward = 0x0B3,
    Rewind = 0x0B4,
    ScanNextTrack = 0x0B5,
    ScanPreviousTrack = 0x0B6,
    Stop = 0x0B7,
    Eject = 0x0B8,
    RandomPlay = 0x0B9,

    Repeat = 0x0BC,

    TrackNormal = 0x0BE,

    FrameForward = 0x0C0,
    FrameBack = 0x0C1,
    Mark = 0x0C2,
    ClearMark = 0x0C3,
    RepeatFromMark = 0x0C4,
    ReturnToMark = 0x0C5,
    SearchMarkForwards = 0x0C6,
    SearchMarkBackwards = 0x0C7,
    CounterReset = 0x0C8,
    ShowCounter = 0x0C9,
    TrackingIncrement = 0x0CA,
    TrackingDecrement = 0x0CB,
    StopEject = 0x0CC,
    PausePlay = 0x0CD,
    PlaySkip = 0x0CE,

    Mute = 0x0E2,

    BassBoost = 0x0E5,
    SurroundMode = 0x0E6,
    Loudness = 0x0E7,
    Mpx = 0x0E8,
    VolumeUp = 0x0E9,
    VolumeDown = 0x0EA,

    SpeedSelect = 0x0F0,
    StandardPlay = 0x0F2,
    LongPlay = 0x0F3,
    ExtendedPlay = 0x0F4,
    Slow = 0x0F5,

    FanEnable = 0x100,

    LightEnable = 0x102,

    ClimateControlEnable = 0x104,

    SecurityEnable = 0x106,
    FireAlarm = 0x107,

    Motion = 0x10A,
    DuressAlarm = 0x10B,
    HoldupAlarm = 0x10C,
    MedicalAlarm = 0x10D,

    BalanceRight = 0x150,
    BalanceLeft = 0x151,
    BassIncrement = 0x152,
    BassDecrement = 0x153,
    TrebleIncrement = 0x154,
    TrebleDecrement = 0x155,

    SubChannelIncrement = 0x171,
    SubChannelDecrement = 0x172,
    AltAudioIncrement = 0x173,
    AltAudioDecrement = 0x174,

    LaunchButtonConfigTool = 0x181,
    ProgrammableButtonConfig = 0x182,
    ConsumerControlConfig = 0x183,
    WordProcessor = 0x184,
    TextEditor = 0x185,
    Spreadsheet = 0x186,
    GraphicsEditor = 0x187,
    PresentationApp = 0x188,
    DatabaseApp = 0x189,
    EmailReader = 0x18A,
    Newsreader = 0x18B,
    Voicemail = 0x18C,
    ContactsAddressBook = 0x18D,
    CalendarSchedule = 0x18E,
    TaskProjectManager = 0x18F,
    LogJournalTimecard = 0x190,
    CheckbookFinance = 0x191,
    Calculator = 0x192,
    AVCapturePlayback = 0x193,
    LocalMachineBrowser = 0x194,
    LANWANBrowser = 0x195,
    InternetBrowser = 0x196,
    RemoteNetworkingISPConnect = 0x197,
    NetworkConference = 0x198,
    NetworkChat = 0x199,
    TelephonyDialer = 0x19A,
    Logon = 0x19B,
    Logoff = 0x19C,
    LogonLogoff = 0x19D,
    TerminalLockScreensaver = 0x19E,
    ControlPanel = 0x19F,
    CommandLineProcessorRun = 0x1A0,
    ProcessTaskManager = 0x1A1,
    SelectTastApplication = 0x1A2,
    NextTaskApplication = 0x1A3,
    PreviousTaskApplication = 0x1A4,
    PreemptiveHaltTaskApplication = 0x1A5,
    IntegratedHelpCenter = 0x1A6,
    Documents = 0x1A7,
    Thesaurus = 0x1A8,
    Dictionary = 0x1A9,
    Desktop = 0x1AA,
    SpellCheck = 0x1AB,
    GrammarCheck = 0x1AC,
    WirelessStatus = 0x1AD,
    KeyboardLayout = 0x1AE,
    VirusProtection = 0x1AF,
    Encryption = 0x1B0,
    ScreenSaver = 0x1B1,
    Alarms = 0x1B2,
    Clock = 0x1B3,
    FileBrowser = 0x1B4,
    PowerStatus = 0x1B5,
    ImageBrowser = 0x1B6,
    AudioBrowser = 0x1B7,
    MovieBrowser = 0x1B8,
    DigitalRightsManager = 0x1B9,
    DigitalWallet = 0x1BA,

    InstantMessaging = 0x1BC,
    OEMFeaturesTipsTutorial = 0x1BD,
    OEMHelp = 0x1BE,
    OnlineCommunity = 0x1BF,
    EntertainmentContent = 0x1C0,
    OnlineShopping = 0x1C1,
    SmartcardInfoHelp = 0x1C2,
    MarketMonitor = 0x1C3,
    CustomizedCorpNews = 0x1C4,
    OnlineActivity = 0x1C5,
    SearchBrowser = 0x1C6,
    AudioPlayer = 0x1C7,

    New = 0x201,
    Open = 0x202,
    Close = 0x203,
    Exit = 0x204,
    Maximize = 0x205,
    Minimize = 0x206,
    Save = 0x207,
    Print = 0x208,
    Properties = 0x209,
    Undo = 0x21A,
    Copy = 0x21B,
    Cut = 0x21C,
    Paste = 0x21D,
    SelectAll = 0x21E,
    Find = 0x21F,
    FindAndReplace = 0x220,
    Search = 0x221,
    GoTo = 0x222,
    Home = 0x223,
    Back = 0x224,
    Forward = 0x225,
    StopWeb = 0x226,
    Refresh = 0x227,
    PreviousLink = 0x228,
    NextLink = 0x229,
    Bookmarks = 0x22A,
    History = 0x22B,
    Subscriptions = 0x22C,
    ZoomIn = 0x22D,
    ZoomOut = 0x22E,
    Zoom = 0x22F,
    FullScreenView = 0x230,
    NormalView = 0x231,
    ViewToggle = 0x232,
    ScrollUp = 0x233,
    ScrollDown = 0x234,
    Scroll = 0x235,
    PanLeft = 0x236,
    PanRight = 0x237,
    Pan = 0x238,
    NewWindow = 0x239,
    TileHorizontally = 0x23A,
    TileVertically = 0x23B,
    Format = 0x23C,
    Edit = 0x23D,
    Bold = 0x23E,
    Italics = 0x23F,
    Underline = 0x240,
    Strikethrough = 0x241,
    Subscript = 0x242,
    Superscript = 0x243,
    AllCaps = 0x244,
    Rotate = 0x245,
    Resize = 0x246,
    FilpHorizontal = 0x247,
    FilpVertical = 0x248,
    MirrorHorizontal = 0x249,
    MirrorVertical = 0x24A,
    FontSelect = 0x24B,
    FontColor = 0x24C,
    FontSize = 0x24D,
    JustifyLeft = 0x24E,
    JustifyCenterH = 0x24F,
    JustifyRight = 0x250,
    JustifyBlockH = 0x251,
    JustifyTop = 0x252,
    JustifyCenterV = 0x253,
    JustifyBottom = 0x254,
    JustifyBlockV = 0x255,
    IndentDecrease = 0x256,
    IndentIncrease = 0x257,
    NumberedList = 0x258,
    RestartNumbering = 0x259,
    BulletedList = 0x25A,
    Promote = 0x25B,
    Demote = 0x25C,
    Yes = 0x25D,
    No = 0x25E,
    Cancel = 0x25F,
    Catalog = 0x260,
    BuyCheckout = 0x261,
    AddToCart = 0x262,
    Expand = 0x263,
    ExpandAll = 0x264,
    Collapse = 0x265,
    CollapseAll = 0x266,
    PrintPreview = 0x267,
    PasteSpecial = 0x268,
    InsertMode = 0x269,
    Delete = 0x26A,
    Lock = 0x26B,
    Unlock = 0x26C,
    Protect = 0x26D,
    Unprotect = 0x26E,
    AttachComment = 0x26F,
    DeleteComment = 0x270,
    ViewComment = 0x271,
    SelectWord = 0x272,
    SelectSentence = 0x273,
    SelectParagraph = 0x274,
    SelectColumn = 0x275,
    SelectRow = 0x276,
    SelectTable = 0x277,
    SelectObject = 0x278,
    RedoRepeat = 0x279,
    Sort = 0x27A,
    SortAscending = 0x27B,
    SortDescending = 0x27C,
    Filter = 0x27D,
    SetClock = 0x27E,
    ViewClock = 0x27F,
    SelectTimeZone = 0x280,
    EditTimeZone = 0x281,
    SetAlarm = 0x282,
    ClearAlarm = 0x283,
    SnoozeAlarm = 0x284,
    ResetAlarm = 0x285,
    Synchronize = 0x286,
    SendReceive = 0x287,
    SendTo = 0x288,
    Reply = 0x289,
    ReplyAll = 0x28A,
    ForwardMsg = 0x28B,
    Send = 0x28C,
    AttachFile = 0x28D,
    Upload = 0x28E,
    Download = 0x28F,
    SetBorders = 0x290,
    InsertRow = 0x291,
    InsertColumn = 0x292,
    InsertFile = 0x293,
    InsertPicture = 0x294,
    InsertObject = 0x295,
    InsertSymbol = 0x296,
    SaveAndClose = 0x297,
    Rename = 0x298,
    Merge = 0x299,
    Split = 0x29A,
    DistributeHorizontally = 0x29B,
    DistributeVertically = 0x29C,
    NextKeyboardLayoutSel = 0x29D,
}
