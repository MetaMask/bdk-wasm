import {
  Address,
  Amount,
  BdkError,
  BdkErrorCode,
  FeeRate,
  Recipient,
  Wallet,
} from "../../../pkg/bitcoindevkit";
import type { Network } from "../../../pkg/bitcoindevkit";

describe("Wallet", () => {
  const network: Network = "testnet";
  const externalDesc =
    "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p";
  const internalDesc =
    "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e";
  let wallet: Wallet;
  const recipientAddress = Address.from_string(
    "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v",
    network
  );

  it("creates a new wallet from descriptors", () => {
    wallet = Wallet.create(network, externalDesc, internalDesc);

    const address = wallet.peek_address("external", 0);

    expect(wallet.network).toBe(network);
    expect(address.address.toString()).toBe(
      "tb1qjtgffm20l9vu6a7gacxvpu2ej4kdcsgc26xfdz"
    );
    expect(address.address_type).toBe("p2wpkh");
    expect(wallet.reveal_next_address("external").address.toString()).toBe(
      address.address.toString()
    );
    expect(wallet.public_descriptor("external")).toBe(
      "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );
    expect(wallet.public_descriptor("internal")).toBe(
      "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
  });

  it("loads a previously existing wallet", () => {
    const loadedWallet = Wallet.load(
      wallet.take_staged(),
      externalDesc,
      internalDesc
    );

    expect(loadedWallet.network).toBe(network);
    expect(
      loadedWallet.next_unused_address("external").address.toString()
    ).toBe("tb1qjtgffm20l9vu6a7gacxvpu2ej4kdcsgc26xfdz");
  });

  it("catches fine-grained errors and deserializes its data", () => {
    // Amount should be too big so we fail with InsufficientFunds
    const sendAmount = Amount.from_sat(BigInt(2000000000));

    try {
      wallet
        .build_tx()
        .fee_rate(new FeeRate(BigInt(1)))
        .add_recipient(
          new Recipient(recipientAddress.script_pubkey, sendAmount)
        )
        .finish();
    } catch (error) {
      expect(error).toBeInstanceOf(BdkError);

      const { code, message, data } = error;
      expect(code).toBe(BdkErrorCode.InsufficientFunds);
      expect(message.startsWith("Insufficient funds:")).toBe(true);
      expect(data.needed).toBe(2000000000 + 42);
      expect(data.available).toBeDefined();
    }
  });
});
