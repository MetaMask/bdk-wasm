import { EsploraClient, Wallet } from "../../../pkg/bitcoindevkit";

describe("Esplora client", () => {
  const stopGap = 5;
  const parallelRequests = 1;
  const wallet = Wallet.create(
    "signet",
    "wpkh(tprv8ZgxMBicQKsPe2qpAuh1K1Hig72LCoP4JgNxZM2ZRWHZYnpuw5oHoGBsQm7Qb8mLgPpRJVn3hceWgGQRNbPD6x1pp2Qme2YFRAPeYh7vmvE/84'/1'/0'/0/*)#a6kgzlgq",
    "wpkh(tprv8ZgxMBicQKsPe2qpAuh1K1Hig72LCoP4JgNxZM2ZRWHZYnpuw5oHoGBsQm7Qb8mLgPpRJVn3hceWgGQRNbPD6x1pp2Qme2YFRAPeYh7vmvE/84'/1'/0'/1/*)#vwnfl2cc"
  );
  const esploraClient = new EsploraClient("https://mutinynet.com/api");

  it("Performs full scan on a new wallet", async () => {
    let request = wallet.start_full_scan();
    let update = await esploraClient.full_scan(
      request,
      stopGap,
      parallelRequests
    );
    wallet.apply_update(update);

    expect(wallet.balance.trusted_spendable.to_sat()).toBeGreaterThan(0);
    expect(wallet.latest_checkpoint.height).toBeGreaterThan(0);
  }, 60000);
});
