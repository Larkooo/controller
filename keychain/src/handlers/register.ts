import { ec, number } from "starknet";
import {
  Message,
  RegisterResponse,
  RegisterRequest,
} from "@cartridge/controller";
import Controller from "utils/account";
import { BigNumberish, toBN } from "starknet/utils/number";
import { calculateContractAddressFromHash, getSelectorFromName } from "starknet/utils/hash";
import { encodeShortString } from "starknet/utils/shortString";

const PROXY_CLASS = "0x793a374a266432184f68b29546d14fedfdcbe6346bc51bd34ad730e6ff914f3";
const ACCOUNT_CLASS = "0x21a58754bd7658d29f70e1e5dbebf84ae393a5ef704c4f5a763cc8a61cb3414";
const CONTROLLER_CLASS = "0x10baeb4233aae14d72f1c2f60d8c46be61436fb06631c835df93b3a9f566351";
const ACCOUNT_ADDRESS = "0x07d7bbf672edd77578b8864c3e2900ac9194698220adb1b1ecdc45f9222ca291";

export async function register(
  message: Message<RegisterRequest>,
) {
  const keypair = ec.genKeyPair();
  const deviceKey = ec.getStarkKey(keypair);

  try {
    const {
      params: { username, credential },
    } = message.payload as RegisterRequest;

    const { x: x0, y: x1, z: x2 } = split(toBN(credential.x))
    const { x: y0, y: y1, z: y2 } = split(toBN(credential.y))

    const address = calculateContractAddressFromHash(
      encodeShortString(username),
      toBN(PROXY_CLASS),
      [
        toBN(ACCOUNT_CLASS),
        getSelectorFromName("initialize"),
        "9",
        toBN(CONTROLLER_CLASS),
        "7",
        x0,
        x1,
        x2,
        y0,
        y1,
        y2,
        toBN(deviceKey),
        "12",
      ],
      toBN(ACCOUNT_ADDRESS),
    )

    const controller = new Controller(
      keypair,
      address,
    );
    controller.cache();
    controller.approve(
      "https://cartridge.gg/",
      [],
      "0",
    );

    return {
      method: "register",
      result: { address },
    } as RegisterResponse;
  } catch (error) {
    return {
      method: "register",
      error,
    } as RegisterResponse;
  }
}

const BASE = number.toBN(2).pow(toBN(86));

function split(n: BigNumberish): { x: BigNumberish; y: BigNumberish; z: BigNumberish } {
  const x = n.mod(BASE);
  const y = n.div(BASE).mod(BASE);
  const z = n.div(BASE).div(BASE);
  return { x, y, z };
}
