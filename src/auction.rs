//! A collection of NFTs sold one per round in an open auction.
//!
//! Every round lasts `round_duration` milliseconds (24 hours in production). Anyone bids by
//! attaching CSPR; a bidder who is outbid gets the CSPR back in the same transaction. When a round
//! is over its NFT is minted to the highest bidder and the winning bid is paid to the owner. That
//! happens with the first transaction of a later round, or when anyone calls `settle`.
use odra::{
    casper_types::{bytesrepr::Bytes, U256, U512},
    prelude::*,
};
use odra_modules::cep95::{CEP95Interface, Cep95};

/// Errors of the auction. The codes of `Cep95` start at 40000.
#[odra::odra_error]
pub enum Error {
    /// A bid must be higher than the current highest bid.
    BidTooLow = 1,
    /// A bid must not be lower than the minimum bid.
    BidBelowMinimum = 2,
    /// A round must last longer than zero milliseconds.
    InvalidRoundDuration = 3,
    /// The contract has not been initialised.
    NotInitialized = 4,
}

/// A bid became the highest bid of a round.
#[odra::event]
pub struct BidPlaced {
    pub round: u64,
    pub bidder: Address,
    pub amount: U512,
}

/// An outbid bidder got the CSPR back.
#[odra::event]
pub struct BidRefunded {
    pub round: u64,
    pub bidder: Address,
    pub amount: U512,
}

/// A round ended with a winner: the NFT `token_id` was minted to `winner`.
#[odra::event]
pub struct AuctionSettled {
    pub round: u64,
    pub token_id: U256,
    pub winner: Address,
    pub amount: U512,
}

#[odra::module(events = [BidPlaced, BidRefunded, AuctionSettled], errors = Error)]
pub struct DailyAuction {
    /// The NFT collection (CEP-95).
    token: SubModule<Cep95>,
    /// The account that receives the winning bids.
    owner: Var<Address>,
    /// Block time of the start of round 0, in milliseconds.
    started_at: Var<u64>,
    /// Length of a round in milliseconds.
    round_duration: Var<u64>,
    /// The lowest acceptable bid, in motes (1 CSPR = 1_000_000_000 motes).
    min_bid: Var<U512>,
    /// The round the highest bid below belongs to.
    bid_round: Var<u64>,
    /// The highest bidder of `bid_round`, if anyone has bid.
    highest_bidder: Var<Option<Address>>,
    /// The highest bid of `bid_round`, held by the contract.
    highest_bid: Var<U512>,
}

#[odra::module]
impl DailyAuction {
    /// Starts round 0 at the time of the deployment. The deployer becomes the owner.
    pub fn init(&mut self, name: String, symbol: String, round_duration: u64, min_bid: U512) {
        if round_duration == 0 {
            self.env().revert(Error::InvalidRoundDuration);
        }
        self.token.name.set(name);
        self.token.symbol.set(symbol);
        self.owner.set(self.env().caller());
        self.started_at.set(self.env().get_block_time());
        self.round_duration.set(round_duration);
        self.min_bid.set(min_bid);
    }

    /// Bids the attached CSPR in the current round. The bid must beat the highest bid; the
    /// previous highest bidder is refunded.
    #[odra(payable)]
    pub fn bid(&mut self) {
        self.settle();
        let round = self.current_round();
        let bidder = self.env().caller();
        let amount = self.env().attached_value();
        if amount < self.min_bid.get_or_default() {
            self.env().revert(Error::BidBelowMinimum);
        }
        let highest_bid = self.highest_bid.get_or_default();
        if amount <= highest_bid {
            self.env().revert(Error::BidTooLow);
        }
        if let Some(previous) = self.highest_bidder.get_or_default() {
            self.env().transfer_tokens(&previous, &highest_bid);
            self.env().emit_event(BidRefunded {
                round,
                bidder: previous,
                amount: highest_bid,
            });
        }
        self.bid_round.set(round);
        self.highest_bidder.set(Some(bidder));
        self.highest_bid.set(amount);
        self.env().emit_event(BidPlaced {
            round,
            bidder,
            amount,
        });
    }

    /// Closes a finished round: mints its NFT to the winner and pays the winning bid to the
    /// owner. Does nothing while the round is still open or if nobody bid. Anyone may call it.
    pub fn settle(&mut self) {
        let round = self.bid_round.get_or_default();
        if round >= self.current_round() {
            return;
        }
        if let Some(winner) = self.highest_bidder.get_or_default() {
            let amount = self.highest_bid.get_or_default();
            let token_id = U256::from(round);
            self.highest_bidder.set(None);
            self.highest_bid.set(U512::zero());
            let metadata = vec![
                ("name".to_string(), format!("{} #{}", self.token.name(), round)),
                ("round".to_string(), round.to_string()),
                ("winning_bid_motes".to_string(), amount.to_string()),
            ];
            self.token.raw_mint(winner, token_id, metadata);
            self.env().transfer_tokens(&self.owner(), &amount);
            self.env().emit_event(AuctionSettled {
                round,
                token_id,
                winner,
                amount,
            });
        }
        self.bid_round.set(self.current_round());
    }

    /// The number of the round that is open now, starting from 0.
    pub fn current_round(&self) -> u64 {
        let elapsed = self.env().get_block_time() - self.started_at.get_or_default();
        elapsed / self.round_duration()
    }

    /// Block time, in milliseconds, at which the open round ends.
    pub fn round_ends_at(&self) -> u64 {
        self.started_at.get_or_default() + (self.current_round() + 1) * self.round_duration()
    }

    /// Length of a round in milliseconds.
    pub fn round_duration(&self) -> u64 {
        self.round_duration.get_or_revert_with(Error::NotInitialized)
    }

    /// The highest bid of the open round in motes; zero if nobody has bid yet.
    pub fn highest_bid(&self) -> U512 {
        if self.bid_round.get_or_default() == self.current_round() {
            self.highest_bid.get_or_default()
        } else {
            U512::zero()
        }
    }

    /// The highest bidder of the open round.
    pub fn highest_bidder(&self) -> Option<Address> {
        if self.bid_round.get_or_default() == self.current_round() {
            self.highest_bidder.get_or_default()
        } else {
            None
        }
    }

    /// The lowest acceptable bid in motes.
    pub fn min_bid(&self) -> U512 {
        self.min_bid.get_or_default()
    }

    /// The account that receives the winning bids.
    pub fn owner(&self) -> Address {
        self.owner.get_or_revert_with(Error::NotInitialized)
    }

    delegate! {
        to self.token {
            fn name(&self) -> String;
            fn symbol(&self) -> String;
            fn balance_of(&self, owner: Address) -> U256;
            fn owner_of(&self, token_id: U256) -> Option<Address>;
            fn safe_transfer_from(&mut self, from: Address, to: Address, token_id: U256, data: Option<Bytes>);
            fn transfer_from(&mut self, from: Address, to: Address, token_id: U256);
            fn approve(&mut self, spender: Address, token_id: U256);
            fn revoke_approval(&mut self, token_id: U256);
            fn approved_for(&self, token_id: U256) -> Option<Address>;
            fn approve_for_all(&mut self, operator: Address);
            fn revoke_approval_for_all(&mut self, operator: Address);
            fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool;
            fn token_metadata(&self, token_id: U256) -> Vec<(String, String)>;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::time::Duration;
    use odra::host::{Deployer, HostEnv, HostRef};

    const DAY: u64 = 24 * 60 * 60 * 1000;
    const CSPR: u64 = 1_000_000_000;

    fn setup() -> (HostEnv, DailyAuctionHostRef) {
        let env = odra_test::env();
        let auction = DailyAuction::deploy(
            &env,
            DailyAuctionInitArgs {
                name: "Daily".to_string(),
                symbol: "DAY".to_string(),
                round_duration: DAY,
                min_bid: U512::from(CSPR),
            },
        );
        (env, auction)
    }

    #[test]
    fn highest_bidder_wins_and_losers_are_refunded() {
        let (env, mut auction) = setup();
        let (owner, alice, bob) = (env.get_account(0), env.get_account(1), env.get_account(2));
        let alice_before = env.balance_of(&alice);
        let owner_before = env.balance_of(&owner);

        env.set_caller(alice);
        auction.with_tokens(U512::from(10 * CSPR)).bid();
        env.set_caller(bob);
        auction.with_tokens(U512::from(15 * CSPR)).bid();

        assert_eq!(env.balance_of(&alice), alice_before);
        assert!(env.emitted_event(
            &auction,
            BidRefunded {
                round: 0,
                bidder: alice,
                amount: U512::from(10 * CSPR)
            }
        ));
        assert_eq!(auction.highest_bidder(), Some(bob));

        env.advance_block_time(Duration::from_millis(DAY));
        assert_eq!(auction.current_round(), 1);
        assert_eq!(auction.highest_bid(), U512::zero());
        env.set_caller(alice);
        auction.settle();

        assert_eq!(auction.owner_of(U256::zero()), Some(bob));
        assert_eq!(env.balance_of(&owner), owner_before + U512::from(15 * CSPR));
        assert_eq!(env.balance_of(&auction), U512::zero());
    }

    #[test]
    fn a_bid_in_the_next_round_settles_the_previous_one() {
        let (env, mut auction) = setup();
        let (alice, bob) = (env.get_account(1), env.get_account(2));

        env.set_caller(alice);
        auction.with_tokens(U512::from(2 * CSPR)).bid();
        env.advance_block_time(Duration::from_millis(3 * DAY));
        env.set_caller(bob);
        auction.with_tokens(U512::from(CSPR)).bid();

        assert_eq!(auction.owner_of(U256::zero()), Some(alice));
        assert_eq!(auction.current_round(), 3);
        assert_eq!(auction.highest_bidder(), Some(bob));
        assert_eq!(env.balance_of(&auction), U512::from(CSPR));
    }

    #[test]
    fn low_bids_are_rejected() {
        let (env, mut auction) = setup();
        env.set_caller(env.get_account(1));
        assert_eq!(
            auction.with_tokens(U512::from(CSPR / 2)).try_bid(),
            Err(Error::BidBelowMinimum.into())
        );
        auction.with_tokens(U512::from(5 * CSPR)).bid();
        env.set_caller(env.get_account(2));
        assert_eq!(
            auction.with_tokens(U512::from(5 * CSPR)).try_bid(),
            Err(Error::BidTooLow.into())
        );
    }

    #[test]
    fn a_round_without_bids_mints_nothing() {
        let (env, mut auction) = setup();
        env.advance_block_time(Duration::from_millis(DAY));
        auction.settle();
        assert_eq!(auction.owner_of(U256::zero()), None);
        assert_eq!(auction.current_round(), 1);
    }
}
