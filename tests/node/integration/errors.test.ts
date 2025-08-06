import {
  Address,
  Amount,
  BdkError,
  BdkErrorCode,
} from "../../../pkg/bitcoindevkit";
import type { Network } from "../../../pkg/bitcoindevkit";

describe("Wallet", () => {
  const network: Network = "testnet";

  it("catches fine-grained address errors", () => {
    try {
      Address.from_string(
        "tb1qd28npep0s8frcm3y7dxqajkcy2m40eysplyr9v",
        "bitcoin"
      );
    } catch (error) {
      expect(error).toBeInstanceOf(BdkError);

      const { code, message, data } = error;
      expect(code).toBe(BdkErrorCode.NetworkValidation);
      expect(message.startsWith("validation error")).toBe(true);
      expect(data).toBeUndefined();
    }

    try {
      Address.from_string("notAnAddress", network);
    } catch (error) {
      expect(error).toBeInstanceOf(BdkError);

      const { code, message, data } = error;
      expect(code).toBe(BdkErrorCode.Base58);
      expect(message.startsWith("base58 error")).toBe(true);
      expect(data).toBeUndefined();
    }
  });

  it("catches fine-grained amount errors", () => {
    try {
      Amount.from_btc(-100000000);
    } catch (error) {
      expect(error).toBeInstanceOf(BdkError);

      const { code, message, data } = error;
      expect(code).toBe(BdkErrorCode.OutOfRange);
      expect(message.startsWith("amount out of range")).toBe(true);
      expect(data).toBeUndefined();
    }
  });
});
