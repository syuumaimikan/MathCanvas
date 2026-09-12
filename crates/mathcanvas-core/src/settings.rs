use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppSettings {
    pub math: MathSettings,
    pub pen: PenSettings,
    pub graph: GraphSettings,
    pub locale: LocaleSettings,
    pub notebook: NotebookSettings,
    pub platform: PlatformSettings,
}

// ---------------------------------------------------------
// Math Engine Settings
// ---------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MathSettings {
    pub angle_unit: AngleUnit,
    pub calculation_mode: CalculationMode,
    pub complex_mode: ComplexMode,
    pub precision: PrecisionSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum AngleUnit {
    #[default]
    Radian,
    Degree,
    Gradian,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum CalculationMode {
    #[default]
    ExactAlgebraic, // Prioritize fractions, CAS
    NumericApproximate, // Fast floating point
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum ComplexMode {
    #[default]
    RealOnly,
    ComplexAuto,
    ImaginarySymbol(ImaginarySymbol),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum ImaginarySymbol {
    #[default]
    I, // Math standard
    J, // Engineering standard
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrecisionSettings {
    pub significant_digits: u32,
    pub bignum_bits: u32,
}

impl Default for PrecisionSettings {
    fn default() -> Self {
        Self {
            significant_digits: 10,
            bignum_bits: 256,
        }
    }
}

// ---------------------------------------------------------
// Pen & Stroke Settings
// ---------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PenSettings {
    pub input_mode: PenInputMode,
    pub palm_rejection: Sensitivity,
    pub recognition_trigger: RecognitionTrigger,
    pub smoothing: SmoothingLevel,
    pub auto_shape_recognition: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum PenInputMode {
    #[default]
    StylusOnly,
    Hybrid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum Sensitivity {
    Off,
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum RecognitionTrigger {
    #[default]
    RealTime,
    ExplicitGesture,
    ManualButton,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum SmoothingLevel {
    None,
    Weak,
    #[default]
    Strong,
}

// ---------------------------------------------------------
// Graph & Rendering Settings
// ---------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GraphSettings {
    pub point_count: u32,
    pub adaptive_sampling: bool,
    pub hardware_acceleration: bool,
    pub coordinate_system: CoordinateSystem,
    pub singularity_detection: bool,
}

impl Default for GraphSettings {
    fn default() -> Self {
        Self {
            point_count: 1000,
            adaptive_sampling: true,
            hardware_acceleration: true,
            coordinate_system: CoordinateSystem::Cartesian,
            singularity_detection: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum CoordinateSystem {
    #[default]
    Cartesian,
    Polar,
    Logarithmic,
}

// ---------------------------------------------------------
// Locale & Display Settings
// ---------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LocaleSettings {
    pub decimal_style: DecimalStyle,
    pub multiply_symbol: MultiplySymbol,
    pub divide_symbol: DivideSymbol,
    pub default_font: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum DecimalStyle {
    #[default]
    UsJapan, // 1,234.56
    EuropeanIso, // 1.234,56
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum MultiplySymbol {
    #[default]
    Times, // x
    Dot, // .
    Implicit,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum DivideSymbol {
    #[default]
    Fraction,
    Div,   // ÷
    Slash, // /
}

// ---------------------------------------------------------
// Notebook Settings
// ---------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NotebookSettings {
    pub reactive_dag: ReactiveMode,
    pub auto_save_interval_sec: u32,
    pub undo_stack_limit: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub enum ReactiveMode {
    #[default]
    Immediate,
    ManualBatch,
}

// ---------------------------------------------------------
// Platform Settings
// ---------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PlatformSettings {
    pub use_windows_ink: bool,
    pub android_refresh_sync: u32,
}
