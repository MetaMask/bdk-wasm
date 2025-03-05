import {
  AddressType,
  Network,
  seed_to_descriptor,
  seed_to_xpriv,
  xpriv_to_descriptor,
  xpub_to_descriptor,
} from "../../../pkg/bitcoindevkit";
import { mnemonicToSeedSync } from "bip39";

describe("Utilities", () => {
  const addressType: AddressType = "p2wpkh";
  const network: Network = "testnet";
  const seed = mnemonicToSeedSync(
    "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose"
  );

  it("generates xpriv from seed", async () => {
    const xpriv = seed_to_xpriv(seed, network);

    expect(xpriv).toBe(
      "tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU"
    );
  });

  it("generates descriptors from seed", async () => {
    const descriptors = seed_to_descriptor(seed, network, addressType);

    expect(descriptors.external).toBe(
      "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/0/*)#uel0vg9p"
    );
    expect(descriptors.internal).toBe(
      "wpkh(tprv8ZgxMBicQKsPf6vydw7ixvsLKY79hmeXujBkGCNCApyft92yVYng2y28JpFZcneBYTTHycWSRpokhHE25GfHPBxnW5GpSm2dMWzEi9xxEyU/84'/1'/0'/1/*)#dd6w3a4e"
    );
  });

  it("extracts descriptors from xpriv", async () => {
    const xpriv =
      "tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp";
    const fingerprint = "27f9035f";
    const descriptors = xpriv_to_descriptor(
      xpriv,
      fingerprint,
      network,
      addressType
    );

    expect(descriptors.external).toBe(
      "wpkh([27f9035f/84'/1'/0']tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp/0/*)#sx5quhf7"
    );
    expect(descriptors.internal).toBe(
      "wpkh([27f9035f/84'/1'/0']tprv8g4stFEyX1zQoi4oNBdUFy4cDqWcyWu1kacHgK3RRvTdTPDm8HTxhERpV9JLTct69h4479xKJXm85SYkFZ4eMUsru5MdUNkeouuzbivKAJp/1/*)#pj3ppzex"
    );
  });

  it("extracts descriptors from xpub", async () => {
    const xpub =
      "tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq";
    const fingerprint = "27f9035f";
    const descriptors = xpub_to_descriptor(
      xpub,
      fingerprint,
      network,
      addressType
    );

    expect(descriptors.external).toBe(
      "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/0/*)#wle7e0wp"
    );
    expect(descriptors.internal).toBe(
      "wpkh([27f9035f/84'/1'/0']tpubDCkv2fHDfPg5hB6bFqJ4fNiins2Z8r5vKtD4xq5irCG2HsUXkgHYsj3gfGTdvAv41hoJeXjfxu7EBQqZMm6SVkxztKFtaaE7HuLdkuL7KNq/1/*)#ltuly67e"
    );
  });
});
