import {
  AccountInterface,
  DeployContractPayload,
  Abi,
  Call,
  AddTransactionResponse,
  InvocationsDetails,
  Signature,
  typedData,
  number,
  Provider,
  Invocation,
  EstimateFeeResponse as StarknetEstimateFeeResponse,
} from "starknet";
import { Messenger } from "./messenger";
import {
  DeployContractResponse,
  EstimateFeeResponse,
  ExecuteResponse,
  GetNonceResponse,
  HashMessageResponse,
  ProbeRequest,
  ProbeResponse,
  Scope,
  SignMessageResponse,
  VerifyMessageHashResponse,
  VerifyMessageResponse,
} from "./types";

export class Account extends Provider implements AccountInterface {
  address: string;
  messenger: Messenger;

  constructor(address: string, messenger: Messenger) {
    super();
    this.address = address;
    this.messenger = messenger;
  }

  /**
   * Deploys a given compiled contract (json) to starknet
   *
   * @param payload payload to be deployed containing:
   * - compiled contract code
   * - constructor calldata
   * - address salt
   * @param abi the abi of the contract
   * @returns a confirmation of sending a transaction on the starknet contract
   */
  async deployContract(
    payload: DeployContractPayload,
    abi?: Abi
  ): Promise<AddTransactionResponse> {
    const response = await this.messenger.send<DeployContractResponse>({
      method: "deploy-contract",
      params: {
        payload,
        abi,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  /**
   * Estimate Fee for a method on starknet
   *
   * @param invocation the invocation object containing:
   * - contractAddress - the address of the contract
   * - entrypoint - the entrypoint of the contract
   * - calldata - (defaults to []) the calldata
   * - signature - (defaults to []) the signature
   *
   * @returns response from addTransaction
   */
  async estimateFee(
    invocation: Invocation
  ): Promise<StarknetEstimateFeeResponse> {
    const response = await this.messenger.send<EstimateFeeResponse>({
      method: "deploy-contract",
      params: {
        invocation,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  /**
   * Invoke execute function in account contract
   *
   * @param transactions the invocation object or an array of them, containing:
   * - contractAddress - the address of the contract
   * - entrypoint - the entrypoint of the contract
   * - calldata - (defaults to []) the calldata
   * - signature - (defaults to []) the signature
   * @param abi (optional) the abi of the contract for better displaying
   *
   * @returns response from addTransaction
   */
  async execute(
    transactions: Call | Call[],
    abis?: Abi[],
    transactionsDetail?: InvocationsDetails
  ): Promise<AddTransactionResponse> {
    let response = await this.messenger.send<ExecuteResponse>({
      method: "execute",
      params: {
        transactions,
        abis,
        transactionsDetail,
      },
    });

    if (response.result) {
      return response.result;
    }

    if (response.error && response.error !== "missing scopes") {
      throw new Error(response.error as string);
    }

    window.open(
      `process.env.BASE_URL/wallet/approve?origin=${encodeURIComponent(
        window.origin
      )}&scopes=${encodeURIComponent(JSON.stringify(response.scopes))}`,
      "_blank",
      "height=600,width=400"
    );

    response = await this.messenger.send<ExecuteResponse>({
      method: "execute",
      params: {
        transactions,
        abis,
        transactionsDetail,
        wait: true,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  /**
   * Sign an JSON object for off-chain usage with the starknet private key and return the signature
   * This adds a message prefix so it cant be interchanged with transactions
   *
   * @param json - JSON object to be signed
   * @returns the signature of the JSON object
   * @throws {Error} if the JSON object is not a valid JSON
   */
  async signMessage(typedData: typedData.TypedData): Promise<Signature> {
    const response = await this.messenger.send<SignMessageResponse>({
      method: "sign-message",
      params: {
        typedData,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  /**
   * Hash a JSON object with pederson hash and return the hash
   * This adds a message prefix so it cant be interchanged with transactions
   *
   * @param json - JSON object to be hashed
   * @returns the hash of the JSON object
   * @throws {Error} if the JSON object is not a valid JSON
   */
  async hashMessage(typedData: typedData.TypedData): Promise<string> {
    const response = await this.messenger.send<HashMessageResponse>({
      method: "hash-message",
      params: {
        typedData,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  /**
   * Verify a signature of a JSON object
   *
   * @param json - JSON object to be verified
   * @param signature - signature of the JSON object
   * @returns true if the signature is valid, false otherwise
   * @throws {Error} if the JSON object is not a valid JSON or the signature is not a valid signature
   */
  async verifyMessage(
    typedData: typedData.TypedData,
    signature: Signature
  ): Promise<boolean> {
    const response = await this.messenger.send<VerifyMessageResponse>({
      method: "verify-message",
      params: {
        typedData,
        signature,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  /**
   * Verify a signature of a given hash
   * @warning This method is not recommended, use verifyMessage instead
   *
   * @param hash - hash to be verified
   * @param signature - signature of the hash
   * @returns true if the signature is valid, false otherwise
   * @throws {Error} if the signature is not a valid signature
   */
  async verifyMessageHash(
    hash: number.BigNumberish,
    signature: Signature
  ): Promise<boolean> {
    const response = await this.messenger.send<VerifyMessageHashResponse>({
      method: "verify-message-hash",
      params: {
        hash,
        signature,
      },
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }

  async getNonce(): Promise<string> {
    const response = await this.messenger.send<GetNonceResponse>({
      method: "get-nonce",
    });

    if (response.error) {
      throw new Error(response.error as string);
    }

    return response.result;
  }
}
