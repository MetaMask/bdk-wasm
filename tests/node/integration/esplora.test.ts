import {
  Address,
  Amount,
  EsploraClient,
  FeeRate,
  Network,
  Recipient,
  Wallet,
} from "../../../pkg/bitcoindevkit";

// Tests are expected to run in order
describe("Esplora client", () => {
  const stopGap = 5;
  const parallelRequests = 1;
  const externalDescriptor =
    "wpkh(tprv8ZgxMBicQKsPe2qpAuh1K1Hig72LCoP4JgNxZM2ZRWHZYnpuw5oHoGBsQm7Qb8mLgPpRJVn3hceWgGQRNbPD6x1pp2Qme2YFRAPeYh7vmvE/84'/1'/0'/0/*)#a6kgzlgq";
  const internalDescriptor =
    "wpkh(tprv8ZgxMBicQKsPe2qpAuh1K1Hig72LCoP4JgNxZM2ZRWHZYnpuw5oHoGBsQm7Qb8mLgPpRJVn3hceWgGQRNbPD6x1pp2Qme2YFRAPeYh7vmvE/84'/1'/0'/1/*)#vwnfl2cc";
  const network: Network = "signet";
  const esploraUrl = "https://mutinynet.com/api";
  const recipientAddress = Address.from_string(
    "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v",
    network
  );

  let feeRate: FeeRate;
  let wallet: Wallet;
  const esploraClient = new EsploraClient(esploraUrl);

  it("creates a new wallet", () => {
    wallet = Wallet.create(network, externalDescriptor, internalDescriptor);
    expect(wallet.peek_address("external", 0).address.toString()).toBe(
      "tb1qq2a8ypxglm07luzq8rl29vxkrwxt3j04ac84ze"
    );
  });

  it("synchronizes a wallet", async () => {
    const request = wallet.start_sync_with_revealed_spks();
    const update = await esploraClient.sync(request, parallelRequests);
    wallet.apply_update(update);

    expect(wallet.latest_checkpoint.height).toBeGreaterThan(0);
  });

  it("performs full scan on a wallet", async () => {
    const request = wallet.start_full_scan();
    const update = await esploraClient.full_scan(
      request,
      stopGap,
      parallelRequests
    );
    wallet.apply_update(update);

    expect(wallet.balance.trusted_spendable.to_sat()).toBeGreaterThan(0);
    expect(wallet.latest_checkpoint.height).toBeGreaterThan(0);
  });

  it("fetches fee estimates", async () => {
    const confirmationTarget = 2;
    const feeEstimates = await esploraClient.get_fee_estimates();

    const fee = feeEstimates.get(confirmationTarget);
    expect(fee).toBeDefined();
    feeRate = new FeeRate(BigInt(Math.floor(fee)));
  });

  it("sends a transaction", async () => {
    const sendAmount = Amount.from_sat(BigInt(1000));
    expect(wallet.balance.trusted_spendable.to_sat()).toBeGreaterThan(
      sendAmount.to_sat()
    );

    // Important to test that we can load the wallet from a changeset with the signing descriptors and be able to sign a transaction
    // as the changeset does not contain the private signing information.
    const loadedWallet = Wallet.load(
      wallet.take_staged(),
      externalDescriptor,
      internalDescriptor
    );
    expect(loadedWallet.balance.total.to_sat()).toEqual(
      wallet.balance.total.to_sat()
    );

    const initialDerivationIndex = loadedWallet.derivation_index("internal");
    const psbt = loadedWallet
      .build_tx()
      .fee_rate(feeRate)
      .add_recipient(new Recipient(recipientAddress, sendAmount))
      .finish();

    expect(psbt.fee().to_sat()).toBeGreaterThan(100); // We cannot know the exact fees

    const finalized = loadedWallet.sign(psbt);
    expect(finalized).toBeTruthy();

    const tx = psbt.extract_tx();
    await esploraClient.broadcast(tx);

    // Assert that we are aware of newly created addresses that were revealed during PSBT creation
    const currentDerivationIndex = loadedWallet.derivation_index("internal");
    expect(initialDerivationIndex).toBeLessThan(currentDerivationIndex);

    const fetchedTx = await esploraClient.get_tx(tx.compute_txid());
    expect(fetchedTx).toBeDefined();
  });

  it("excludes utxos from a transaction", () => {
    const utxos = wallet.list_unspent();
    expect(utxos.length).toBeGreaterThan(0);

    // Exclude all UTXOs and expect an insufficient funds error
    expect(() => {
      wallet
        .build_tx()
        .drain_wallet()
        .unspendable(utxos.map((utxo) => utxo.outpoint))
        .finish();
    }).toThrow();
  });
});
