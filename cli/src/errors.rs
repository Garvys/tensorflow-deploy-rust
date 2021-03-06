#[cfg(feature = "tensorflow")]
use conform;
use ndarray;
use serde_json;
use tfdeploy;

/// Configures error handling for this crate.
error_chain! {
    types {
        CliError, CliErrorKind, CliResultExt, CliResult;
    }
    links {
        Conform(conform::Error, conform::ErrorKind) #[cfg(feature="tensorflow")];
        Tfdeploy(tfdeploy::TfdError, tfdeploy::TfdErrorKind);
    }

    foreign_links {
        Io(::std::io::Error);
        NumParseInt(::std::num::ParseIntError);
        SerdeJson(serde_json::Error);
        NdarrayShape(ndarray::ShapeError);
    }
}
