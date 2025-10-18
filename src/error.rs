use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, Default)]
pub struct Error {
    message: String,
    notes: Vec<String>,
    helps: Vec<String>,
    raw: Option<String>,
    source: Option<Box<dyn StdError + Send + Sync + 'static>>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl StdError for Error {
    // Add the explicit return type here to match the trait
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(|s| s.as_ref() as _)
    }
}

pub fn red(text: impl Into<String>) -> String {
    format!("\x1b[1;31m{}\x1b[0m", text.into())
}

pub fn green(text: impl Into<String>) -> String {
    format!("\x1b[1;32m{}\x1b[0m", text.into())
}

impl Error {
    pub fn new<M: fmt::Display>(message: M) -> Self {
        Self {
            message: message.to_string(),
            ..Default::default()
        }
    }

    pub fn raw(message: impl Into<String>) -> Self {
        Self {
            raw: Some(message.into()),
            ..Default::default()
        }
    }

    pub fn with_source(
        mut self,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        self.source = Some(Box::new(source));
        self
    }

    pub fn note<M: fmt::Display>(mut self, note: M) -> Self {
        self.notes.push(note.to_string());
        self
    }

    pub fn help<M: fmt::Display>(mut self, help: M) -> Self {
        self.helps.push(help.to_string());
        self
    }

    pub fn message(&self) -> String {
        if let Some(raw_message) = &self.raw {
            return raw_message.clone();
        }

        let mut lines: Vec<String> = Vec::new();

        // First `error:`
        lines.push(format!("{}: {}", red("error"), self.message));

        // Next `error:` if any
        let mut current_source = self.source();
        while let Some(source) = current_source {
            lines.push(format!("{}: {}", red("error"), source));
            current_source = source.source();
        }

        // Next `note:` if any
        if !self.notes.is_empty() {
            for help in &self.notes {
                lines.push(format!("{}: {}", green("note"), help));
            }
        }

        // Next `help:` if any
        if !self.helps.is_empty() {
            for help in &self.helps {
                lines.push(format!("{}: {}", green("help"), help));
            }
        }

        lines.join("\n")
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait IntoHelp {
    fn into_help(self, error: Error) -> Error;
}

impl IntoHelp for &str {
    fn into_help(self, error: Error) -> Error {
        error.help(self)
    }
}

impl IntoHelp for String {
    fn into_help(self, error: Error) -> Error {
        error.help(self)
    }
}

impl<M: fmt::Display> IntoHelp for Option<M> {
    fn into_help(self, error: Error) -> Error {
        if let Some(message) = self {
            return error.help(message);
        }
        error
    }
}

pub trait IntoNote {
    fn into_note(self, error: Error) -> Error;
}

impl IntoNote for &str {
    fn into_note(self, error: Error) -> Error {
        error.note(self)
    }
}

impl IntoNote for String {
    fn into_note(self, error: Error) -> Error {
        error.note(self)
    }
}

impl<M: fmt::Display> IntoNote for Option<M> {
    fn into_note(self, error: Error) -> Error {
        if let Some(message) = self {
            return error.note(message);
        }
        error
    }
}

// Internal helper to apply the key-value options to the Error struct.
#[macro_export]
#[doc(hidden)]
macro_rules! __error_apply_options {
    // Base case: no options left to apply.
    ($err:ident $(,)?) => {};

    // Recursive case: apply the 'source' option.
    ($err:ident, source: $source:expr, $($rest:tt)*) => {
        $err = $err.with_source($source);
        $crate::__error_apply_options!($err, $($rest)*);
    };

    // Recursive case: apply a 'note' option.
    ($err:ident, note: $note:expr, $($rest:tt)*) => {
        $err = $crate::error::IntoNote::into_note($note, $err);
        $crate::__error_apply_options!($err, $($rest)*);
    };

    // Recursive case: apply a 'help' option.
    ($err:ident, help: $help:expr, $($rest:tt)*) => {
        $err = $crate::error::IntoHelp::into_help($help, $err);
        $crate::__error_apply_options!($err, $($rest)*);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __error_internal {
    ( @fmt_args($($fmt_args:tt)*) @options($($options:tt)*) ) => {
        {
            let mut err = $crate::error::Error::new(format!($($fmt_args)*));
            $crate::__error_apply_options!(err, $($options)*);
            err
        }
    };

    ( @fmt_args($($fmt_args:tt)*) @options($($options:tt)*) $key:ident : $value:expr, $($rest:tt)* ) => {
        $crate::__error_internal! {
            @fmt_args($($fmt_args)*)
            @options($($options)* $key: $value,)
            $($rest)*
        }
    };

    ( @fmt_args($($fmt_args:tt)*) @options($($options:tt)*) $arg:expr, $($rest:tt)* ) => {
        $crate::__error_internal! {
            @fmt_args($($fmt_args)* , $arg)
            @options($($options)*)
            $($rest)*
        }
    };
}

#[macro_export]
macro_rules! error {
    // error!(raw: some_string)
    (raw: $err:expr) => {
        $crate::error::Error::raw(format!("{}", $err))
    };

    // error!("ready go! {} {}", vroom, vroom)
    ($fmt:expr, $($args:expr),+) => {
        $crate::error::Error::new(format!($fmt, $($args),+))
    };

    // error!("message {}", path, source: err, help: "...")
    ($fmt:expr, $($rest:tt)+) => {
        $crate::__error_internal! {
            @fmt_args($fmt)
            @options()
            $($rest)*,
        }
    };

    // error!("message") or error!("message {}", var)
    ($($msg:tt)+) => {
        $crate::error::Error::new(format!($($msg)+))
    };
}

#[macro_export]
macro_rules! bail {
    ($($arg:tt)+) => {
        return Err($crate::error!($($arg)+))
    };
}

// This is for direct `?` return
impl From<clap::error::Error> for Error {
    fn from(err: clap::error::Error) -> Error {
        use clap::error::{
            ContextKind,
            ErrorKind,
        };

        match err.kind() {
            ErrorKind::InvalidSubcommand => {
                let command = err
                    .get(ContextKind::InvalidSubcommand)
                    .expect("clap: `InvalidSubcommand` should exists")
                    .to_string();

                error!(
                    "no such command: `{}`", command,
                    help: err.get(ContextKind::SuggestedSubcommand).map(|s| {
                        format!("a command with a similar name exists: `{}`", s)
                    }),
                    help: "view all available commands with `cargo eth --help`"
                )
            }
            ErrorKind::UnknownArgument => {
                let arg = err
                    .get(ContextKind::InvalidArg)
                    .expect("clap: `InvalidArg` should exists")
                    .to_string();

                error!(
                    "unexpected argument '{}' found", arg,
                    note: "arguments for the underlying command must be passed after `--`",
                    help: err.get(ContextKind::SuggestedArg).map(|s| {
                        format!("an argument with a similar name exists: `{}`", s)
                    }),
                    help: format!("to pass '{}' as a value, use '-- {}'", arg, arg),
                    help: "for more information, try '--help'"
                )
            }
            ErrorKind::MissingRequiredArgument => {
                let arg = err
                    .get(ContextKind::InvalidArg)
                    .expect("clap: `InvalidArg` should exists")
                    .to_string();

                error!(
                    "the following required arguments were not provided: {}", arg,
                    help: "for more information, try '--help'"
                )
            }
            // Fallback
            _ => error!(raw: err.render().ansi()),
        }
    }
}
