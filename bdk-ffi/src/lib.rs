mod bitcoin;
mod descriptor;
mod error;
mod esplora;
mod keys;
mod types;
mod wallet;

use crate::bitcoin::Address;
use crate::bitcoin::OutPoint;
use crate::bitcoin::Psbt;
use crate::bitcoin::Script;
use crate::bitcoin::Transaction;
use crate::bitcoin::TxOut;
use crate::descriptor::Descriptor;
use crate::error::AddressError;
use crate::error::Alpha3Error;
use crate::error::CalculateFeeError;
use crate::error::DescriptorError;
use crate::error::EsploraError;
use crate::error::ExtractTxError;
use crate::error::FeeRateError;
use crate::error::PersistenceError;
use crate::error::PsbtParseError;
use crate::error::TransactionError;
use crate::error::TxidParseError;
use crate::error::WalletCreationError;
use crate::esplora::EsploraClient;
use crate::keys::DerivationPath;
use crate::keys::DescriptorPublicKey;
use crate::keys::DescriptorSecretKey;
use crate::keys::Mnemonic;
use crate::types::AddressIndex;
use crate::types::AddressInfo;
use crate::types::Balance;
use crate::types::CanonicalTx;
use crate::types::ChainPosition;
use crate::types::FeeRate;
use crate::types::LocalOutput;
use crate::types::ScriptAmount;
use crate::wallet::BumpFeeTxBuilder;
use crate::wallet::SentAndReceivedValues;
use crate::wallet::TxBuilder;
use crate::wallet::Update;
use crate::wallet::Wallet;

use bdk::bitcoin::Network;
use bdk::keys::bip39::WordCount;
use bdk::wallet::tx_builder::ChangeSpendPolicy;
use bdk::KeychainKind;

uniffi::include_scaffolding!("bdk");

// TODO: TxIn, Payload
