import { Wallet } from "../../../pkg/bitcoindevkit";
import type { Network } from "../../../pkg/bitcoindevkit";

describe("Wallet", () => {
  const network: Network = "testnet";
  let externalDesc =
    "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p";
  let internalDesc =
    "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e";

  it("creates a new wallet from descriptors", () => {
    const wallet = Wallet.create(network, externalDesc, internalDesc);

    let address = wallet.peek_address("external", 0);

    expect(wallet.network).toBe(network);
    expect(address.address).toBe("tb1qjtgffm20l9vu6a7gacxvpu2ej4kdcsgc26xfdz");
    expect(address.address_type).toBe("p2wpkh");
    expect(wallet.reveal_next_address("external").address).toBe(
      address.address
    );
    expect(wallet.public_descriptor("external")).toBe(
      "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );
    expect(wallet.public_descriptor("internal")).toBe(
      "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
  });
});
