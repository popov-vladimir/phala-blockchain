use serde::{Serialize, Deserialize};

use crate::contracts;
use crate::types::TxRef;
use crate::TransactionStatus;
use crate::std::collections::BTreeMap;
use crate::contracts::AccountIdWrapper;
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Auction {
    pub bids: BTreeMap<AccountIdWrapper, u32>,
    pub winner: Option<AccountIdWrapper>,
    pub winning_bid: u32,
}

impl Auction {
    pub fn new() -> Self {
        Auction { bids: BTreeMap::new(), winner:None, winning_bid:0 }
    }
}

/// The commands that the contract accepts from the blockchain. Also called transactions.
/// Commands are supposed to update the states of the contract.
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// Increments the counter in the contract by some number
    PlaceBid {
        value: u32,
    },
}

/// The errors that the contract could throw for some queries
#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    NotAuthorized,
    SomeOtherError,
}

/// Query requests. The end users can only query the contract states by sending requests.
/// Queries are not supposed to write to the contract states.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Ask for the value of the counter
    GetWinner ,
}

/// Query responses.
#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    GetWinner {
        winner: Option<AccountIdWrapper>,
    },
    Error(Error),
}



impl contracts::Contract<Command, Request, Response> for Auction {
    fn id(&self) -> contracts::ContractId { contracts::AUCTION }

    fn handle_command(&mut self, _origin: &chain::AccountId, _txref: &TxRef, cmd: Command) -> TransactionStatus {
        match cmd {
            Command::PlaceBid { value } => {
                let current_user = AccountIdWrapper(_origin.clone()); 
                println!("got bid {} from {:?}", value, current_user);
                self.bids.insert(current_user.clone(), value);
                if value > self.winning_bid {
                    self.winning_bid = value.clone();
                    self.winner = Some(current_user);
                }
                TransactionStatus::Ok
            }
        }
    }

    fn handle_query(&mut self, _origin: Option<&chain::AccountId>, req: Request) -> Response {
        let inner = || -> Result<Response, Error> {
            match req {
                Request::GetWinner => {
                    Ok(Response::GetWinner { winner: self.winner.clone() })
                }
            }
        };
        match inner() {
            Err(error) => Response::Error(error),
            Ok(resp) => resp
        }
    }
}

