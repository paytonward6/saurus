use strum_macros::{Display, EnumString, IntoStaticStr};

/// Languages that can be used by the "listings" LaTeX package.
/// List of all languages can also be found at [TeXDoc](https://texdoc.org/serve/listings.pdf/0) on page 13
#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, IntoStaticStr)] // strum macros.
pub enum Languages {
    #[strum(serialize = "python")]
    Python,

    #[strum(serialize = "c")]
    C,

    #[strum(serialize = "matlab")]
    MATLAB,

    #[strum(serialize = "abap")]
    ABAP,

    #[strum(serialize = "acm")]
    ACM,

    #[strum(serialize = "acmscript")]
    ACMscript,

    #[strum(serialize = "acsl")]
    ACSL,

    #[strum(serialize = "ada")]
    Ada,

    #[strum(serialize = "algol")]
    Algol,

    #[strum(serialize = "ant")]
    Ant,

    #[strum(serialize = "assembler")]
    Assembler,

    #[strum(serialize = "awk")]
    Awk,

    #[strum(serialize = "bash")]
    Bash,

    #[strum(serialize = "basic")]
    Basic,

    #[strum(serialize = "c++")]
    CPP,

    #[strum(serialize = "cil")]
    CIL,

    #[strum(serialize = "clean")]
    Clean,

    #[strum(serialize = "cobol")]
    Cobol,

    #[strum(serialize = "comal 80")]
    Comal80,

    #[strum(serialize = "command.com")]
    CommandDotCom,

    #[strum(serialize = "comsol")]
    Comsol,

    #[strum(serialize = "csh")]
    Csh,

    #[strum(serialize = "delphi")]
    Delphi,

    #[strum(serialize = "eiffel")]
    Eiffel,

    #[strum(serialize = "elan")]
    Elan,

    #[strum(serialize = "elisp")]
    Elisp,

    #[strum(serialize = "erlang")]
    Erlang,

    #[strum(serialize = "euphoria")]
    Euphoria,

    #[strum(serialize = "fortran")]
    Fortran,

    #[strum(serialize = "gap")]
    GAP,

    #[strum(serialize = "gcl")]
    GCL,

    #[strum(serialize = "gnuplot")]
    Gnuplot,

    #[strum(serialize = "go")]
    Go,

    #[strum(serialize = "hansl")]
    Hansl,

    #[strum(serialize = "haskell")]
    Haskell,

    #[strum(serialize = "html")]
    HTML,

    #[strum(serialize = "idl")]
    IDL,

    #[strum(serialize = "inform")]
    InForm,

    #[strum(serialize = "java")]
    Java,

    #[strum(serialize = "jvmis")]
    JVMIS,

    #[strum(serialize = "ksh")]
    Ksh,

    #[strum(serialize = "lingo")]
    Lingo,

    #[strum(serialize = "lisp")]
    Lisp,

    #[strum(serialize = "llvm")]
    LLVM,

    #[strum(serialize = "logo")]
    Logo,

    #[strum(serialize = "lua")]
    Lua,

    #[strum(serialize = "make")]
    MAKE,

    #[strum(serialize = "mathematica")]
    Mathematica,

    #[strum(serialize = "mercury")]
    Mercury,

    #[strum(serialize = "metapost")]
    MetaPost,

    #[strum(serialize = "miranda")]
    Miranda,

    #[strum(serialize = "mizar")]
    Mizar,

    #[strum(serialize = "ml")]
    ML,

    #[strum(serialize = "modula-2")]
    Modula2,

    #[strum(serialize = "mupad")]
    MuPAD,

    #[strum(serialize = "nastran")]
    NASTRAN,

    #[strum(serialize = "oberon-2")]
    Oberon2,

    #[strum(serialize = "ocl")]
    OCL,

    #[strum(serialize = "octave")]
    Octave,

    #[strum(serialize = "oorexx")]
    OORexx,

    #[strum(serialize = "oz")]
    Oz,

    #[strum(serialize = "pascal")]
    Pascal,

    #[strum(serialize = "perl")]
    Perl,

    #[strum(serialize = "php")]
    PHP,

    #[strum(serialize = "pl/i")]
    PLI,

    #[strum(serialize = "plasm")]
    Plasm,

    #[strum(serialize = "postscript")]
    PostScript,

    #[strum(serialize = "pov")]
    POV,

    #[strum(serialize = "prolog")]
    Prolog,

    #[strum(serialize = "promela")]
    Promela,

    #[strum(serialize = "pstricks")]
    PSTricks,

    #[strum(serialize = "r")]
    R,

    #[strum(serialize = "reduce")]
    Reduce,

    #[strum(serialize = "rexx")]
    Rexx,

    #[strum(serialize = "sl")]
    SL,

    #[strum(serialize = "ruby")]
    Ruby,

    #[strum(serialize = "s")]
    S,

    #[strum(serialize = "sas")]
    SAS,

    #[strum(serialize = "scala")]
    Scala,

    #[strum(serialize = "scilab")]
    Scilab,

    #[strum(serialize = "sh")]
    SH,

    #[strum(serialize = "shelxl")]
    SHELXL,

    #[strum(serialize = "simula")]
    Simula,

    #[strum(serialize = "sparql")]
    SPARQL,

    #[strum(serialize = "sql")]
    SQL,

    #[strum(serialize = "swift")]
    Swift,

    #[strum(serialize = "tcl")]
    TCL,

    #[strum(serialize = "tex")]
    TeX,

    #[strum(serialize = "vbscript")]
    VBScript,

    #[strum(serialize = "verilog")]
    Verilog,

    #[strum(serialize = "vhdl")]
    VHDL,

    #[strum(serialize = "vrml")]
    VRML,

    #[strum(serialize = "xml")]
    XML,

    #[strum(serialize = "xslt")]
    XSLT,
}
