#![allow(unused_imports)]
use odra_wasm_client::wasm_bindgen;
use odra_wasm_client::wasm_bindgen_futures;
use odra_wasm_client::JsValueSerdeExt;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub struct DailyAuctionWasmClient {
    wasm_client: odra_wasm_client::OdraWasmClient,
    address: odra_wasm_client::types::Address,
}
#[wasm_bindgen]
impl DailyAuctionWasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "wasmClient")] wasm_client: &odra_wasm_client::OdraWasmClient,
        address: odra_wasm_client::types::Address,
    ) -> Self {
        DailyAuctionWasmClient {
            wasm_client: wasm_client.clone(),
            address,
        }
    }
    #[doc = "Bids the attached CSPR in the current round. The bid must beat the highest bid; the previous highest bidder is refunded."]
    #[wasm_bindgen(js_name = "bid")]
    pub async fn bid(
        &self,
        #[wasm_bindgen(js_name = "attachedValue")] attached_value: odra_wasm_client::types::U512,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_payable_entry_point (* self . address , "bid" , odra_wasm_client :: casper_types :: runtime_args ! { "attached_value" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U512 > :: into_odra_value (attached_value) ? } , * attached_value) . await
    }
    #[doc = "Closes a finished round: mints its NFT to the winner and pays the winning bid to the owner. Does nothing while the round is still open or if nobody bid. Anyone may call it."]
    #[wasm_bindgen(js_name = "settle")]
    pub async fn settle(&self) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self.wasm_client
            .call_entry_point(
                *self.address,
                "settle",
                odra_wasm_client::casper_types::runtime_args! {},
            )
            .await
    }
    #[doc = "The number of the round that is open now, starting from 0."]
    #[wasm_bindgen(js_name = "currentRound")]
    pub async fn current_round(&self) -> Result<u64, JsError> {
        self.wasm_client
            .call_entry_point_with_proxy::<u64, u64>(
                *self.address,
                "current_round",
                odra_wasm_client::casper_types::runtime_args! {},
            )
            .await
    }
    #[doc = "Block time, in milliseconds, at which the open round ends."]
    #[wasm_bindgen(js_name = "roundEndsAt")]
    pub async fn round_ends_at(&self) -> Result<u64, JsError> {
        self.wasm_client
            .call_entry_point_with_proxy::<u64, u64>(
                *self.address,
                "round_ends_at",
                odra_wasm_client::casper_types::runtime_args! {},
            )
            .await
    }
    #[doc = "Length of a round in milliseconds."]
    #[wasm_bindgen(js_name = "roundDuration")]
    pub async fn round_duration(&self) -> Result<u64, JsError> {
        self.wasm_client
            .call_entry_point_with_proxy::<u64, u64>(
                *self.address,
                "round_duration",
                odra_wasm_client::casper_types::runtime_args! {},
            )
            .await
    }
    #[doc = "The highest bid of the open round in motes; zero if nobody has bid yet."]
    #[wasm_bindgen(js_name = "highestBid")]
    pub async fn highest_bid(&self) -> Result<odra_wasm_client::types::U512, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < odra_wasm_client :: types :: U512 , odra_wasm_client :: casper_types :: U512 > (* self . address , "highest_bid" , odra_wasm_client :: casper_types :: runtime_args ! { }) . await
    }
    #[doc = "The highest bidder of the open round."]
    #[wasm_bindgen(js_name = "highestBidder")]
    pub async fn highest_bidder(
        &self,
    ) -> Result<Option<odra_wasm_client::types::Address>, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < Option < odra_wasm_client :: types :: Address > , Option < odra_wasm_client :: OdraAddress > > (* self . address , "highest_bidder" , odra_wasm_client :: casper_types :: runtime_args ! { }) . await
    }
    #[doc = "The lowest acceptable bid in motes."]
    #[wasm_bindgen(js_name = "minBid")]
    pub async fn min_bid(&self) -> Result<odra_wasm_client::types::U512, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < odra_wasm_client :: types :: U512 , odra_wasm_client :: casper_types :: U512 > (* self . address , "min_bid" , odra_wasm_client :: casper_types :: runtime_args ! { }) . await
    }
    #[doc = "The account that receives the winning bids."]
    #[wasm_bindgen(js_name = "owner")]
    pub async fn owner(&self) -> Result<odra_wasm_client::types::Address, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < odra_wasm_client :: types :: Address , odra_wasm_client :: OdraAddress > (* self . address , "owner" , odra_wasm_client :: casper_types :: runtime_args ! { }) . await
    }
    #[doc = "Delegated. See `self.token.name()` for details."]
    #[wasm_bindgen(js_name = "name")]
    pub async fn name(&self) -> Result<String, JsError> {
        self.wasm_client
            .call_entry_point_with_proxy::<String, String>(
                *self.address,
                "name",
                odra_wasm_client::casper_types::runtime_args! {},
            )
            .await
    }
    #[doc = "Delegated. See `self.token.symbol()` for details."]
    #[wasm_bindgen(js_name = "symbol")]
    pub async fn symbol(&self) -> Result<String, JsError> {
        self.wasm_client
            .call_entry_point_with_proxy::<String, String>(
                *self.address,
                "symbol",
                odra_wasm_client::casper_types::runtime_args! {},
            )
            .await
    }
    #[doc = "Delegated. See `self.token.balance_of()` for details."]
    #[wasm_bindgen(js_name = "balanceOf")]
    pub async fn balance_of(
        &self,
        #[wasm_bindgen(js_name = "owner")] owner: odra_wasm_client::types::Address,
    ) -> Result<odra_wasm_client::types::U256, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < odra_wasm_client :: types :: U256 , odra_wasm_client :: casper_types :: U256 > (* self . address , "balance_of" , odra_wasm_client :: casper_types :: runtime_args ! { "owner" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (owner) ? }) . await
    }
    #[doc = "Delegated. See `self.token.owner_of()` for details."]
    #[wasm_bindgen(js_name = "ownerOf")]
    pub async fn owner_of(
        &self,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Option<odra_wasm_client::types::Address>, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < Option < odra_wasm_client :: types :: Address > , Option < odra_wasm_client :: OdraAddress > > (* self . address , "owner_of" , odra_wasm_client :: casper_types :: runtime_args ! { "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? }) . await
    }
    #[doc = "Delegated. See `self.token.safe_transfer_from()` for details."]
    #[wasm_bindgen(js_name = "safeTransferFrom")]
    pub async fn safe_transfer_from(
        &self,
        #[wasm_bindgen(js_name = "from")] from: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "to")] to: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
        #[wasm_bindgen(js_name = "data")] data: Option<Vec<u8>>,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_entry_point (* self . address , "safe_transfer_from" , odra_wasm_client :: casper_types :: runtime_args ! { "from" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (from) ? , "to" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (to) ? , "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? , "data" => odra_wasm_client :: types :: IntoOdraValue :: < Option < Vec < u8 > > > :: into_odra_value (data) ? }) . await
    }
    #[doc = "Delegated. See `self.token.transfer_from()` for details."]
    #[wasm_bindgen(js_name = "transferFrom")]
    pub async fn transfer_from(
        &self,
        #[wasm_bindgen(js_name = "from")] from: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "to")] to: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_entry_point (* self . address , "transfer_from" , odra_wasm_client :: casper_types :: runtime_args ! { "from" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (from) ? , "to" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (to) ? , "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? }) . await
    }
    #[doc = "Delegated. See `self.token.approve()` for details."]
    #[wasm_bindgen(js_name = "approve")]
    pub async fn approve(
        &self,
        #[wasm_bindgen(js_name = "spender")] spender: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_entry_point (* self . address , "approve" , odra_wasm_client :: casper_types :: runtime_args ! { "spender" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (spender) ? , "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? }) . await
    }
    #[doc = "Delegated. See `self.token.revoke_approval()` for details."]
    #[wasm_bindgen(js_name = "revokeApproval")]
    pub async fn revoke_approval(
        &self,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_entry_point (* self . address , "revoke_approval" , odra_wasm_client :: casper_types :: runtime_args ! { "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? }) . await
    }
    #[doc = "Delegated. See `self.token.approved_for()` for details."]
    #[wasm_bindgen(js_name = "approvedFor")]
    pub async fn approved_for(
        &self,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Option<odra_wasm_client::types::Address>, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < Option < odra_wasm_client :: types :: Address > , Option < odra_wasm_client :: OdraAddress > > (* self . address , "approved_for" , odra_wasm_client :: casper_types :: runtime_args ! { "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? }) . await
    }
    #[doc = "Delegated. See `self.token.approve_for_all()` for details."]
    #[wasm_bindgen(js_name = "approveForAll")]
    pub async fn approve_for_all(
        &self,
        #[wasm_bindgen(js_name = "operator")] operator: odra_wasm_client::types::Address,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_entry_point (* self . address , "approve_for_all" , odra_wasm_client :: casper_types :: runtime_args ! { "operator" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (operator) ? }) . await
    }
    #[doc = "Delegated. See `self.token.revoke_approval_for_all()` for details."]
    #[wasm_bindgen(js_name = "revokeApprovalForAll")]
    pub async fn revoke_approval_for_all(
        &self,
        #[wasm_bindgen(js_name = "operator")] operator: odra_wasm_client::types::Address,
    ) -> Result<odra_wasm_client::cspr_click::TransactionResult, JsError> {
        self . wasm_client . call_entry_point (* self . address , "revoke_approval_for_all" , odra_wasm_client :: casper_types :: runtime_args ! { "operator" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (operator) ? }) . await
    }
    #[doc = "Delegated. See `self.token.is_approved_for_all()` for details."]
    #[wasm_bindgen(js_name = "isApprovedForAll")]
    pub async fn is_approved_for_all(
        &self,
        #[wasm_bindgen(js_name = "owner")] owner: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "operator")] operator: odra_wasm_client::types::Address,
    ) -> Result<bool, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < bool , bool > (* self . address , "is_approved_for_all" , odra_wasm_client :: casper_types :: runtime_args ! { "owner" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (owner) ? , "operator" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: OdraAddress > :: into_odra_value (operator) ? }) . await
    }
    #[doc = "Delegated. See `self.token.token_metadata()` for details."]
    #[wasm_bindgen(js_name = "tokenMetadata")]
    pub async fn token_metadata(
        &self,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Vec<JsValue>, JsError> {
        self . wasm_client . call_entry_point_with_proxy :: < Vec < JsValue > , Vec < (String , String) > > (* self . address , "token_metadata" , odra_wasm_client :: casper_types :: runtime_args ! { "token_id" => odra_wasm_client :: types :: IntoOdraValue :: < odra_wasm_client :: casper_types :: U256 > :: into_odra_value (token_id) ? }) . await
    }
}
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub enum DailyAuctionErrors {
    #[doc = "The approval is set to the current owner."]
    ApprovalToCurrentOwner = 40003isize,
    #[doc = "The caller is the same as the operator."]
    ApproveToCaller = 40004isize,
    #[doc = "A bid must not be lower than the minimum bid."]
    BidBelowMinimum = 2isize,
    #[doc = "A bid must be higher than the current highest bid."]
    BidTooLow = 1isize,
    #[doc = "A round must last longer than zero milliseconds."]
    InvalidRoundDuration = 3isize,
    #[doc = "The token ID is invalid."]
    InvalidTokenId = 40005isize,
    #[doc = "The token ID is invalid."]
    NotAnOwnerOrApproved = 40002isize,
    #[doc = "The contract has not been initialised."]
    NotInitialized = 4isize,
    #[doc = "The token with the given ID already exists."]
    TokenAlreadyExists = 40006isize,
    #[doc = "The transfer failed."]
    TransferFailed = 40001isize,
    #[doc = "The value is not set."]
    ValueNotSet = 40000isize,
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct AuctionSettled {
    #[wasm_bindgen(js_name = "round")]
    pub round: u64,
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
    #[wasm_bindgen(js_name = "winner")]
    winner: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "amount")]
    amount: odra_wasm_client::casper_types::U512,
}
#[wasm_bindgen]
impl AuctionSettled {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "round")] round: u64,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
        #[wasm_bindgen(js_name = "winner")] winner: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "amount")] amount: odra_wasm_client::types::U512,
    ) -> Result<Self, JsError> {
        Ok(Self {
            round,
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
            winner: odra_wasm_client::types::IntoOdraValue::into_odra_value(winner)?,
            amount: odra_wasm_client::types::IntoOdraValue::into_odra_value(amount)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_winner(&mut self, value: odra_wasm_client::types::Address) {
        self.winner = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn winner(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.winner.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_amount(&mut self, value: odra_wasm_client::types::U512) {
        self.amount = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> odra_wasm_client::types::U512 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.amount.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for AuctionSettled {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (round, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (winner, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (amount, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((
            Self {
                round,
                token_id,
                winner,
                amount,
            },
            bytes,
        ))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for AuctionSettled {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.round)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.winner)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.amount)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.round.serialized_length();
        result += self.token_id.serialized_length();
        result += self.winner.serialized_length();
        result += self.amount.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for AuctionSettled {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<AuctionSettled> for AuctionSettled {
    fn into_odra_value(self) -> Result<AuctionSettled, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<AuctionSettled> for AuctionSettled {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct BidRefunded {
    #[wasm_bindgen(js_name = "round")]
    pub round: u64,
    #[wasm_bindgen(js_name = "bidder")]
    bidder: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "amount")]
    amount: odra_wasm_client::casper_types::U512,
}
#[wasm_bindgen]
impl BidRefunded {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "round")] round: u64,
        #[wasm_bindgen(js_name = "bidder")] bidder: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "amount")] amount: odra_wasm_client::types::U512,
    ) -> Result<Self, JsError> {
        Ok(Self {
            round,
            bidder: odra_wasm_client::types::IntoOdraValue::into_odra_value(bidder)?,
            amount: odra_wasm_client::types::IntoOdraValue::into_odra_value(amount)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_bidder(&mut self, value: odra_wasm_client::types::Address) {
        self.bidder = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn bidder(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.bidder.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_amount(&mut self, value: odra_wasm_client::types::U512) {
        self.amount = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> odra_wasm_client::types::U512 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.amount.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for BidRefunded {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (round, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (bidder, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (amount, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((
            Self {
                round,
                bidder,
                amount,
            },
            bytes,
        ))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for BidRefunded {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.round)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.bidder)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.amount)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.round.serialized_length();
        result += self.bidder.serialized_length();
        result += self.amount.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for BidRefunded {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<BidRefunded> for BidRefunded {
    fn into_odra_value(self) -> Result<BidRefunded, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<BidRefunded> for BidRefunded {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Burn {
    #[wasm_bindgen(js_name = "from")]
    from: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
}
#[wasm_bindgen]
impl Burn {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "from")] from: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Self, JsError> {
        Ok(Self {
            from: odra_wasm_client::types::IntoOdraValue::into_odra_value(from)?,
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_from(&mut self, value: odra_wasm_client::types::Address) {
        self.from = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn from(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.from.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for Burn {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (from, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((Self { from, token_id }, bytes))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for Burn {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.from)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.from.serialized_length();
        result += self.token_id.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for Burn {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<Burn> for Burn {
    fn into_odra_value(self) -> Result<Burn, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<Burn> for Burn {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct MetadataUpdate {
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
}
#[wasm_bindgen]
impl MetadataUpdate {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Self, JsError> {
        Ok(Self {
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for MetadataUpdate {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((Self { token_id }, bytes))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for MetadataUpdate {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.token_id.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for MetadataUpdate {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<MetadataUpdate> for MetadataUpdate {
    fn into_odra_value(self) -> Result<MetadataUpdate, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<MetadataUpdate> for MetadataUpdate {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct RevokeApproval {
    #[wasm_bindgen(js_name = "owner")]
    owner: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "spender")]
    spender: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
}
#[wasm_bindgen]
impl RevokeApproval {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "owner")] owner: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "spender")] spender: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Self, JsError> {
        Ok(Self {
            owner: odra_wasm_client::types::IntoOdraValue::into_odra_value(owner)?,
            spender: odra_wasm_client::types::IntoOdraValue::into_odra_value(spender)?,
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_owner(&mut self, value: odra_wasm_client::types::Address) {
        self.owner = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn owner(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.owner.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_spender(&mut self, value: odra_wasm_client::types::Address) {
        self.spender = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn spender(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.spender.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for RevokeApproval {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (owner, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (spender, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((
            Self {
                owner,
                spender,
                token_id,
            },
            bytes,
        ))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for RevokeApproval {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.owner)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.spender)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.owner.serialized_length();
        result += self.spender.serialized_length();
        result += self.token_id.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for RevokeApproval {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<RevokeApproval> for RevokeApproval {
    fn into_odra_value(self) -> Result<RevokeApproval, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<RevokeApproval> for RevokeApproval {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct ApprovalForAll {
    #[wasm_bindgen(js_name = "owner")]
    owner: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "operator")]
    operator: odra_wasm_client::OdraAddress,
}
#[wasm_bindgen]
impl ApprovalForAll {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "owner")] owner: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "operator")] operator: odra_wasm_client::types::Address,
    ) -> Result<Self, JsError> {
        Ok(Self {
            owner: odra_wasm_client::types::IntoOdraValue::into_odra_value(owner)?,
            operator: odra_wasm_client::types::IntoOdraValue::into_odra_value(operator)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_owner(&mut self, value: odra_wasm_client::types::Address) {
        self.owner = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn owner(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.owner.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_operator(&mut self, value: odra_wasm_client::types::Address) {
        self.operator = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn operator(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.operator.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for ApprovalForAll {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (owner, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (operator, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((Self { owner, operator }, bytes))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for ApprovalForAll {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.owner)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.operator)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.owner.serialized_length();
        result += self.operator.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for ApprovalForAll {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<ApprovalForAll> for ApprovalForAll {
    fn into_odra_value(self) -> Result<ApprovalForAll, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<ApprovalForAll> for ApprovalForAll {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct BidPlaced {
    #[wasm_bindgen(js_name = "round")]
    pub round: u64,
    #[wasm_bindgen(js_name = "bidder")]
    bidder: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "amount")]
    amount: odra_wasm_client::casper_types::U512,
}
#[wasm_bindgen]
impl BidPlaced {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "round")] round: u64,
        #[wasm_bindgen(js_name = "bidder")] bidder: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "amount")] amount: odra_wasm_client::types::U512,
    ) -> Result<Self, JsError> {
        Ok(Self {
            round,
            bidder: odra_wasm_client::types::IntoOdraValue::into_odra_value(bidder)?,
            amount: odra_wasm_client::types::IntoOdraValue::into_odra_value(amount)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_bidder(&mut self, value: odra_wasm_client::types::Address) {
        self.bidder = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn bidder(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.bidder.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_amount(&mut self, value: odra_wasm_client::types::U512) {
        self.amount = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> odra_wasm_client::types::U512 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.amount.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for BidPlaced {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (round, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (bidder, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (amount, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((
            Self {
                round,
                bidder,
                amount,
            },
            bytes,
        ))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for BidPlaced {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.round)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.bidder)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.amount)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.round.serialized_length();
        result += self.bidder.serialized_length();
        result += self.amount.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for BidPlaced {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<BidPlaced> for BidPlaced {
    fn into_odra_value(self) -> Result<BidPlaced, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<BidPlaced> for BidPlaced {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct RevokeApprovalForAll {
    #[wasm_bindgen(js_name = "owner")]
    owner: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "operator")]
    operator: odra_wasm_client::OdraAddress,
}
#[wasm_bindgen]
impl RevokeApprovalForAll {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "owner")] owner: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "operator")] operator: odra_wasm_client::types::Address,
    ) -> Result<Self, JsError> {
        Ok(Self {
            owner: odra_wasm_client::types::IntoOdraValue::into_odra_value(owner)?,
            operator: odra_wasm_client::types::IntoOdraValue::into_odra_value(operator)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_owner(&mut self, value: odra_wasm_client::types::Address) {
        self.owner = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn owner(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.owner.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_operator(&mut self, value: odra_wasm_client::types::Address) {
        self.operator = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn operator(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.operator.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for RevokeApprovalForAll {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (owner, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (operator, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((Self { owner, operator }, bytes))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for RevokeApprovalForAll {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.owner)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.operator)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.owner.serialized_length();
        result += self.operator.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for RevokeApprovalForAll {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<RevokeApprovalForAll> for RevokeApprovalForAll {
    fn into_odra_value(self) -> Result<RevokeApprovalForAll, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<RevokeApprovalForAll> for RevokeApprovalForAll {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Approval {
    #[wasm_bindgen(js_name = "owner")]
    owner: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "spender")]
    spender: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
}
#[wasm_bindgen]
impl Approval {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "owner")] owner: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "spender")] spender: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Self, JsError> {
        Ok(Self {
            owner: odra_wasm_client::types::IntoOdraValue::into_odra_value(owner)?,
            spender: odra_wasm_client::types::IntoOdraValue::into_odra_value(spender)?,
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_owner(&mut self, value: odra_wasm_client::types::Address) {
        self.owner = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn owner(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.owner.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_spender(&mut self, value: odra_wasm_client::types::Address) {
        self.spender = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn spender(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.spender.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for Approval {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (owner, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (spender, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((
            Self {
                owner,
                spender,
                token_id,
            },
            bytes,
        ))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for Approval {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.owner)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.spender)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.owner.serialized_length();
        result += self.spender.serialized_length();
        result += self.token_id.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for Approval {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<Approval> for Approval {
    fn into_odra_value(self) -> Result<Approval, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<Approval> for Approval {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Transfer {
    #[wasm_bindgen(js_name = "from")]
    from: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "to")]
    to: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
}
#[wasm_bindgen]
impl Transfer {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "from")] from: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "to")] to: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Self, JsError> {
        Ok(Self {
            from: odra_wasm_client::types::IntoOdraValue::into_odra_value(from)?,
            to: odra_wasm_client::types::IntoOdraValue::into_odra_value(to)?,
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_from(&mut self, value: odra_wasm_client::types::Address) {
        self.from = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn from(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.from.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_to(&mut self, value: odra_wasm_client::types::Address) {
        self.to = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn to(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.to.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for Transfer {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (from, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (to, bytes) = odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((Self { from, to, token_id }, bytes))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for Transfer {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.from)?);
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.to)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.from.serialized_length();
        result += self.to.serialized_length();
        result += self.token_id.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for Transfer {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<Transfer> for Transfer {
    fn into_odra_value(self) -> Result<Transfer, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<Transfer> for Transfer {
    fn to_wasm_value(self) -> Self {
        self
    }
}
#[doc = ""]
#[derive(Debug, Clone, serde :: Serialize, serde :: Deserialize)]
#[wasm_bindgen(getter_with_clone)]
pub struct Mint {
    #[wasm_bindgen(js_name = "to")]
    to: odra_wasm_client::OdraAddress,
    #[wasm_bindgen(js_name = "tokenId")]
    token_id: odra_wasm_client::casper_types::U256,
}
#[wasm_bindgen]
impl Mint {
    #[wasm_bindgen(constructor)]
    pub fn new(
        #[wasm_bindgen(js_name = "to")] to: odra_wasm_client::types::Address,
        #[wasm_bindgen(js_name = "tokenId")] token_id: odra_wasm_client::types::U256,
    ) -> Result<Self, JsError> {
        Ok(Self {
            to: odra_wasm_client::types::IntoOdraValue::into_odra_value(to)?,
            token_id: odra_wasm_client::types::IntoOdraValue::into_odra_value(token_id)?,
        })
    }
    #[wasm_bindgen(js_name = "toJson")]
    pub fn to_json(&self) -> JsValue {
        JsValue::from_serde(self).unwrap_or(JsValue::null())
    }
    #[wasm_bindgen(setter)]
    pub fn set_to(&mut self, value: odra_wasm_client::types::Address) {
        self.to = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn to(&self) -> odra_wasm_client::types::Address {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.to.clone())
    }
    #[wasm_bindgen(setter)]
    pub fn set_token_id(&mut self, value: odra_wasm_client::types::U256) {
        self.token_id = odra_wasm_client::types::IntoOdraValue::into_odra_value(value).unwrap();
    }
    #[wasm_bindgen(getter)]
    pub fn token_id(&self) -> odra_wasm_client::types::U256 {
        odra_wasm_client::types::IntoWasmValue::to_wasm_value(self.token_id.clone())
    }
}
impl odra_wasm_client::casper_types::bytesrepr::FromBytes for Mint {
    fn from_bytes(
        bytes: &[u8],
    ) -> Result<(Self, &[u8]), odra_wasm_client::casper_types::bytesrepr::Error> {
        let (to, bytes) = odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        let (token_id, bytes) =
            odra_wasm_client::casper_types::bytesrepr::FromBytes::from_bytes(bytes)?;
        Ok((Self { to, token_id }, bytes))
    }
}
impl odra_wasm_client::casper_types::bytesrepr::ToBytes for Mint {
    fn to_bytes(&self) -> Result<Vec<u8>, odra_wasm_client::casper_types::bytesrepr::Error> {
        let mut result = Vec::with_capacity(self.serialized_length());
        result.extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.to)?);
        result
            .extend(odra_wasm_client::casper_types::bytesrepr::ToBytes::to_bytes(&self.token_id)?);
        Ok(result)
    }
    fn serialized_length(&self) -> usize {
        let mut result = 0;
        result += self.to.serialized_length();
        result += self.token_id.serialized_length();
        result
    }
}
impl odra_wasm_client::casper_types::CLTyped for Mint {
    fn cl_type() -> odra_wasm_client::casper_types::CLType {
        odra_wasm_client::casper_types::CLType::Any
    }
}
impl odra_wasm_client::types::IntoOdraValue<Mint> for Mint {
    fn into_odra_value(self) -> Result<Mint, JsError> {
        Ok(self)
    }
}
impl odra_wasm_client::types::IntoWasmValue<Mint> for Mint {
    fn to_wasm_value(self) -> Self {
        self
    }
}
