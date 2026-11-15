use mc_protocol_macros::packet_enum;

use crate::packets::server::*;

/// Every packet the client sends, by protocol state.
#[packet_enum(Serverbound)]
pub enum Serverbound<'p> {
    // Handshake
    ServerListPing(ServerListPing<'p>),

    // Status
    StatusRequest(StatusRequest),
    PingRequest(PingRequest),

    // Login
    LoginStart(LoginStart<'p>),
    EncryptionResponse(EncryptionResponse<'p>),
    LoginPluginResponse(LoginPluginResponse<'p>),
    LoginAcknowledge(LoginAcknowledge),
    LoginCookieResponse(LoginCookieResponse<'p>),

    // Config
    ClientInformation(ClientInformation<'p>),
    ConfigCookieResponse(ConfigCookieResponse<'p>),
    ConfigPluginMessage(ConfigPluginMessage<'p>),
    AcknowledgeFinish(AcknowledgeFinish),
    ConfigKeepAliveResponse(ConfigKeepAliveResponse),
    ConfigResourcePackResponse(ConfigResourcePackResponse),
    ConfigKnownPacks(ConfigKnownPacks<'p>),
    AcceptCodeOfConduct(AcceptCodeOfConduct),
    ConfigPong(ConfigPong),
    ConfigCustomClickAction(ConfigCustomClickAction<'p>),

    // Play
    ConfirmTeleportation(ConfirmTeleportation),
    Attack(Attack),
    BundleItemSelected(BundleItemSelected),
    ChatAcknowledgement(ChatAcknowledgement),
    ChatCommand(ChatCommand<'p>),
    SignedChatCommand(SignedChatCommand<'p>),
    ChatMessage(ChatMessage<'p>),
    ChunkBatchReceived(ChunkBatchReceived),
    ClientCommand(ClientCommand),
    ClientTickEnd(ClientTickEnd),
    PlayClientInformation(PlayClientInformation<'p>),
    CommandSuggestion(CommandSuggestion<'p>),
    AcknowledgeConfiguration(AcknowledgeConfiguration),
    ClickContainer(ClickContainer),
    CloseContainer(CloseContainer),
    PlayKeepAliveResponse(PlayKeepAliveResponse),
    SetPlayerPosition(SetPlayerPosition),
    SetPlayerPositionRotation(SetPlayerPositionRotation),
    SetPlayerRotation(SetPlayerRotation),
    SetPlayerMovementFlags(SetPlayerMovementFlags),
    PlayerAbilitiesRequest(PlayerAbilitiesRequest),
    PlayerAction(PlayerAction),
    PlayerCommand(PlayerCommand),
    PlayerInput(PlayerInput),
    PlayerLoaded(PlayerLoaded),
    Punch(Punch),
    SetHeldItem(SetHeldItem),
    TeleportToEntity(TeleportToEntity),
    UseItemOn(UseItemOn),
    UseItem(UseItem),
    PlayPong(PlayPong),
    PlayPingRequest(PlayPingRequest),
    PlayCookieResponse(PlayCookieResponse<'p>),
    ClientPluginMessage(ClientPluginMessage<'p>),
    PlayResourcePackResponse(PlayResourcePackResponse),
    PlayCustomClickAction(PlayCustomClickAction<'p>),
    ClickContainerButton(ClickContainerButton),
    ChangeContainerSlotState(ChangeContainerSlotState),
    SetCreativeModeSlot(SetCreativeModeSlot<'p>),
    PickItemFromBlock(PickItemFromBlock),
    PickItemFromEntity(PickItemFromEntity),
    PlaceRecipe(PlaceRecipe),
    ChangeRecipeBookSettings(ChangeRecipeBookSettings),
    SetSeenRecipe(SetSeenRecipe),
    RenameItem(RenameItem<'p>),
    SelectTrade(SelectTrade),
    SetBeaconEffect(SetBeaconEffect),
    EditBook(EditBook<'p>),
    UpdateSign(UpdateSign<'p>),
}
