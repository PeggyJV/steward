//! Error types

use abscissa_core::error::{BoxError, Context};
use deep_space::error::{AddressError, CosmosGrpcError, PrivateKeyError};
use ethers::prelude::{
    errors::EtherscanError, gas_oracle::GasOracleError, AbiError, ContractError, Middleware,
    ProviderError,
};
use std::{
    fmt::{self, Display},
    io,
    net::AddrParseError,
    ops::Deref,
};
use thiserror::Error;
use tonic::{transport::Error as TonicError, Status as TonicStatus};
use x509_parser::prelude::{PEMError, X509Error};

/// Kinds of errors
#[derive(Copy, Clone, Debug, Eq, Error, PartialEq)]
pub enum ErrorKind {
    /// Abi error
    #[error("abi error")]
    AbiError,
    /// Error in configuration file
    #[error("config error")]
    Config,
    /// Contract error
    #[error("contract error")]
    ContractError,
    /// Input/output error
    #[error("I/O error")]
    Io,
    /// Gas Oracle error
    #[error("gas error")]
    GasOracle,
    /// Provider error
    #[error("grpc error")]
    GrpcError,
    /// Input/output error
    #[error("http error")]
    Http,
    /// Cryptographic Keys error
    #[error("key related error")]
    KeysError,
    /// Provider error
    #[error("provider error")]
    ProviderError,
    /// Strategy Provider call error
    #[error("SP call error")]
    SPCallError,
    /// Governance call error
    #[error("SP call error")]
    GovernanceCall,
    /// Client error
    #[error("client error")]
    ClientError,
    /// Cache error
    #[error("cache error")]
    CacheError,
    /// Unapproved cellar error
    #[error("unapproved cellar error")]
    UnapprovedCellar,
    /// Invalid ethereum address
    #[error("invalid ethereum address")]
    InvalidEthereumAddress,
    /// Proposal processing error
    #[error("proposal processing error")]
    ProposalProcessingError,
    #[error("subscriber not found")]
    SubscriberNotFound,
    #[error("publisher not found")]
    PublisherNotFound,
    #[error("invalid argument")]
    InvalidArgument,
    #[error("no subscriptions")]
    NoSubscriptions,
    #[error("parsing error")]
    ParsingError,
    #[error("invalid certificate")]
    InvalidCertificate,
    #[error("invalid domain name")]
    InvalidDomainName,
}

impl ErrorKind {
    /// Create an error context from this error
    pub fn context(self, source: impl Into<BoxError>) -> Context<ErrorKind> {
        Context::new(self, Some(source.into()))
    }
}

/// Error type
#[derive(Debug)]
pub struct Error(Box<Context<ErrorKind>>);

impl Deref for Error {
    type Target = Context<ErrorKind>;

    fn deref(&self) -> &Context<ErrorKind> {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<AbiError> for Error {
    fn from(err: AbiError) -> Error {
        ErrorKind::AbiError.context(err).into()
    }
}

impl From<AddressError> for Error {
    fn from(err: AddressError) -> Error {
        ErrorKind::KeysError.context(err).into()
    }
}

impl From<AddrParseError> for Error {
    fn from(err: AddrParseError) -> Error {
        ErrorKind::Config.context(err).into()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Context::new(kind, None).into()
    }
}

impl From<EtherscanError> for Error {
    fn from(err: EtherscanError) -> Self {
        ErrorKind::GasOracle.context(err).into()
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(context: Context<ErrorKind>) -> Self {
        Error(Box::new(context))
    }
}

impl From<CosmosGrpcError> for Error {
    fn from(err: CosmosGrpcError) -> Self {
        ErrorKind::GrpcError.context(err).into()
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        ErrorKind::Io.context(err).into()
    }
}

impl From<iqhttp::Error> for Error {
    fn from(err: iqhttp::Error) -> Self {
        ErrorKind::Http.context(err).into()
    }
}

impl From<GasOracleError> for Error {
    fn from(err: GasOracleError) -> Self {
        ErrorKind::GasOracle.context(err).into()
    }
}

impl<T: 'static + Middleware> From<ContractError<T>> for Error {
    fn from(err: ContractError<T>) -> Self {
        let err: BoxError = err.into();
        ErrorKind::ContractError.context(err).into()
    }
}

impl From<PrivateKeyError> for Error {
    fn from(err: PrivateKeyError) -> Self {
        ErrorKind::KeysError.context(err).into()
    }
}

impl From<ProviderError> for Error {
    fn from(err: ProviderError) -> Self {
        let err: BoxError = err.into();
        ErrorKind::ContractError.context(err).into()
    }
}

impl From<TonicError> for Error {
    fn from(err: TonicError) -> Self {
        let err: BoxError = err.into();
        ErrorKind::GrpcError.context(err).into()
    }
}

impl From<TonicStatus> for Error {
    fn from(err: TonicStatus) -> Self {
        let err: BoxError = err.into();
        ErrorKind::GrpcError.context(err).into()
    }
}

impl From<X509Error> for Error {
    fn from(err: X509Error) -> Self {
        let err: BoxError = err.into();
        ErrorKind::ParsingError.context(err).into()
    }
}

impl From<x509_parser::nom::Err<X509Error>> for Error {
    fn from(err: x509_parser::nom::Err<X509Error>) -> Self {
        let err: BoxError = err.into();
        ErrorKind::ParsingError.context(err).into()
    }
}

impl From<PEMError> for Error {
    fn from(err: PEMError) -> Self {
        let err: BoxError = err.into();
        ErrorKind::ParsingError.context(err).into()
    }
}

impl From<x509_parser::nom::Err<PEMError>> for Error {
    fn from(err: x509_parser::nom::Err<PEMError>) -> Self {
        let err: BoxError = err.into();
        ErrorKind::ParsingError.context(err).into()
    }
}
