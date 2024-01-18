#![no_std]

use gmeta::Metadata;
use gmeta::{In, InOut, Out};
use gstd::prelude::*;
use gstd::ActorId;

type TransactionId = u64;
type ReservationId = u64;
type AttributeId = u64;

#[derive(Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgAction {
    // TODO: 0️⃣ Copy actions from previous lesson and push changes to the master branch
    // TODO: 3️⃣ Add new actions
    Name,
    Age,
    Feed,
    Play,
    Sleep,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
    ApproveTokens {
        account: ActorId,
        amount: u128,
    },
    SetFTokenContract(ActorId),
    BuyAttribute {
        store_id: ActorId,
        attribute_id: AttributeId,
    },
    CheckState,
    ReserveGas {
        reservation_amount: u64,
        duration: u32,
    },
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum TmgEvent {
    // TODO: 0️⃣ Copy events from previous lesson and push changes to the master branch
    // TODO: 4️⃣ Add new events
    FeedMe,
    PlayWithMe,
    WantToSleep,
    MakeReservation,
    GasReserved,
}

pub struct Tamagotchi {
    // TODO: 0️⃣ Copy fields from previous lesson and push changes to the master branch
    // TODO: 2️⃣ Add new fields
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: u64,
    pub fed_block: u64,
    pub entertained: u64,
    pub entertained_block: u64,
    pub rested: u64,
    pub rested_block: u64,
    pub allowed_account: Option<ActorId>,
    pub ft_contract_id: ActorId,
    pub ft_transaction_id: TransactionId,
    pub approve_transaction: Option<(TransactionId, ActorId, u128)>,
    pub reservations: Vec<ReservationId>,
}

pub struct ProgramMetadata;

// TODO: 0️⃣ Copy `Metadata` from the first lesson and push changes to the master branch
impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Out<Tamagotchi>;
}
