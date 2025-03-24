import {
  Wallet,
  EsploraClient,
  ChangeSet,
} from "../../../pkg/bitcoindevkit";

describe("Wallet persistence", () => {
  const externalDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m";
  const internalDescriptor = "tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr";
  const network = "signet";
  const esploraUrl = "https://mutinynet.com/api";
  
  it("handles wallet persistence, reloading, and address generation", async () => {
    const client = new EsploraClient(esploraUrl);
    let wallet = Wallet.create(network, externalDescriptor, internalDescriptor);
    
    // Initial scan
    const fullScanRequest = wallet.start_full_scan();
    const update = await client.full_scan(fullScanRequest, 1, 1);
    wallet.apply_update(update);
    expect(wallet.latest_checkpoint.height).toBeGreaterThan(0);

    // Test persistence and reload
    const walletDataString = wallet.take_staged().to_json();
    const initialBalance = wallet.balance.confirmed.to_sat();

    // Simulate wallet destruction and reload
    wallet = null;
    const changeSet = ChangeSet.from_json(walletDataString);
    wallet = Wallet.load(
      changeSet,
      externalDescriptor,
      internalDescriptor
    );
    expect(wallet.balance.confirmed.to_sat()).toBe(initialBalance);

    // Test syncing reloaded wallet
    const syncRequest = wallet.start_sync_with_revealed_spks();
    const syncUpdate = await client.sync(syncRequest, 1);
    wallet.apply_update(syncUpdate);
    expect(wallet.latest_checkpoint.height).toBeGreaterThan(0);

    // Test address generation updates
    const initialAddress = wallet.reveal_next_address("external");
    const updateChangeSet = wallet.take_staged();
    expect(updateChangeSet).toBeDefined();
    
    // Save both the initial address and its index for comparison
    const initialAddressString = initialAddress.address.toString();
    const initialAddressIndex = initialAddress.index;
    
    // Test merging changes
    const baseChangeSet = ChangeSet.from_json(walletDataString);
    baseChangeSet.merge(updateChangeSet);
    
    const mergedWallet = Wallet.load(
      baseChangeSet,
      externalDescriptor,
      internalDescriptor
    );
    
    // Compare with the saved initial address using its actual index
    expect(mergedWallet.peek_address("external", initialAddressIndex).address.toString())
      .toBe(initialAddressString);
  });
}); 