"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Banksy = exports.ProgramTestContext = exports.BanksClient = exports.GenesisConfig = exports.BanksTransactionResultWithMeta = exports.BanksTransactionMeta = exports.TransactionReturnData = exports.FeeRateGovernor = exports.PohConfig = exports.Clock = exports.Rent = exports.TransactionStatus = exports.EpochSchedule = void 0;
const internal_1 = require("./internal");
var internal_2 = require("./internal");
Object.defineProperty(exports, "EpochSchedule", { enumerable: true, get: function () { return internal_2.EpochSchedule; } });
Object.defineProperty(exports, "TransactionStatus", { enumerable: true, get: function () { return internal_2.TransactionStatus; } });
Object.defineProperty(exports, "Rent", { enumerable: true, get: function () { return internal_2.Rent; } });
Object.defineProperty(exports, "Clock", { enumerable: true, get: function () { return internal_2.Clock; } });
Object.defineProperty(exports, "PohConfig", { enumerable: true, get: function () { return internal_2.PohConfig; } });
Object.defineProperty(exports, "FeeRateGovernor", { enumerable: true, get: function () { return internal_2.FeeRateGovernor; } });
const web3_js_1 = require("@solana/web3.js");
const bs58_1 = __importDefault(require("bs58"));
function convertCommitment(c) {
    if (c != null) {
        switch (c) {
            case "processed":
                return 0 /* CommitmentLevel.Processed */;
            case "confirmed":
                return 1 /* CommitmentLevel.Confirmed */;
            case "finalized":
                return 2 /* CommitmentLevel.Finalized */;
            default:
                throw new Error(`Unrecognized commitment level: ${c}`);
        }
    }
    return null;
}
function toAccountInfo(acc) {
    return {
        executable: acc.executable,
        owner: new web3_js_1.PublicKey(acc.owner),
        lamports: Number(acc.lamports),
        data: acc.data,
        rentEpoch: Number(acc.rentEpoch),
    };
}
function fromAccountInfo(acc) {
    const maybeRentEpoch = acc.rentEpoch;
    const rentEpoch = maybeRentEpoch || 0;
    return new internal_1.Account(BigInt(acc.lamports), acc.data, acc.owner.toBytes(), acc.executable, BigInt(rentEpoch));
}
function fromOptionalPubkey(pubkey) {
    if (pubkey) {
        return pubkey.toBytes();
    }
    return undefined;
}
class TransactionReturnData {
    constructor(inner) {
        this.inner = inner;
    }
    get programId() {
        return new web3_js_1.PublicKey(this.inner.programId);
    }
    get data() {
        return this.inner.data;
    }
}
exports.TransactionReturnData = TransactionReturnData;
/**
 * Transaction metadata.
 */
class BanksTransactionMeta {
    constructor(inner) {
        this.inner = inner;
    }
    /** The log messages written during transaction execution. */
    get logMessages() {
        return this.inner.logMessages;
    }
    /** The transaction return data, if present. */
    get returnData() {
        const inner = this.inner.returnData;
        if (!inner)
            return null;
        return new TransactionReturnData(inner);
    }
    /** The number of compute units consumed by the transaction. */
    get computeUnitsConsumed() {
        return this.inner.computeUnitsConsumed;
    }
}
exports.BanksTransactionMeta = BanksTransactionMeta;
/**
 * A transaction result. Contains transaction metadata, and the transaction error, if there is one.
 */
class BanksTransactionResultWithMeta {
    constructor(inner) {
        this.inner = inner;
    }
    /** The transaction error info, if the transaction failed. */
    get result() {
        return this.inner.result;
    }
    /** The transaction metadata. */
    get meta() {
        const inner = this.inner.meta;
        if (!inner)
            return null;
        return new BanksTransactionMeta(inner);
    }
}
exports.BanksTransactionResultWithMeta = BanksTransactionResultWithMeta;
class GenesisConfig {
    constructor(inner) {
        this.inner = inner;
    }
    get creationTime() {
        return this.inner.creationTime;
    }
    get accounts() {
        return new Map(this.inner.accounts.map((obj) => {
            return [new web3_js_1.PublicKey(obj.address), toAccountInfo(obj.account)];
        }));
    }
    get nativeInstructionProcessors() {
        return this.inner.nativeInstructionProcessors.map((obj) => [
            obj.stringVal,
            new web3_js_1.PublicKey(obj.pubkeyVal),
        ]);
    }
    get rewardsPools() {
        return new Map(this.inner.rewardsPools.map((obj) => {
            return [new web3_js_1.PublicKey(obj.address), toAccountInfo(obj.account)];
        }));
    }
    get ticksPerSlot() {
        return this.inner.ticksPerSlot;
    }
    get pohConfig() {
        return this.inner.pohConfig;
    }
    get feeRateGovernor() {
        return this.inner.feeRateGovernor;
    }
    get rent() {
        return this.inner.rent;
    }
    get inflation() {
        return this.inner.inflation;
    }
    get epochSchedule() {
        return this.inner.epochSchedule;
    }
    get clusterType() {
        return this.inner.clusterType;
    }
}
exports.GenesisConfig = GenesisConfig;
/**
 * A client for the ledger state, from the perspective of an arbitrary validator.
 *
 * The client is used to send transactions and query account data, among other things.
 * Use `start()` to initialize a BanksClient.
 */
class BanksClient {
    constructor(inner) {
        this.inner = inner;
    }
    /**
     * Return the account at the given address at the slot corresponding to the given
     * commitment level. If the account is not found, None is returned.
     * @param address - The account address to look up.
     * @param commitment - The commitment to use.
     * @returns The account object, if the account exists.
     */
    async getAccount(address, commitment) {
        const inner = await this.inner.getAccount(address.toBytes(), convertCommitment(commitment));
        return inner === null ? null : toAccountInfo(inner);
    }
    /**
     * Send a transaction and return immediately.
     * @param tx - The transaction to send.
     */
    async sendTransaction(tx) {
        const serialized = tx.serialize();
        const internal = this.inner;
        if (tx instanceof web3_js_1.Transaction) {
            await internal.sendLegacyTransaction(serialized);
        }
        else {
            await internal.sendVersionedTransaction(serialized);
        }
    }
    /**
     * Process a transaction and return the result with metadata.
     * @param tx - The transaction to send.
     * @returns The transaction result and metadata.
     */
    async processTransaction(tx) {
        const serialized = tx.serialize();
        const internal = this.inner;
        const inner = tx instanceof web3_js_1.Transaction
            ? await internal.processLegacyTransaction(serialized)
            : await internal.processVersionedTransaction(serialized);
        return new BanksTransactionMeta(inner);
    }
    /**
     * Try to process a transaction and return the result with metadata.
     *
     * If the transaction errors, a JS error is not raised.
     * Instead the returned object's `result` field will contain an error message.
     *
     * This makes it easier to process transactions that you expect to fail
     * and make assertions about things like log messages.
     *
     * @param tx - The transaction to send.
     * @returns The transaction result and metadata.
     */
    async tryProcessTransaction(tx) {
        const serialized = tx.serialize();
        const internal = this.inner;
        const inner = tx instanceof web3_js_1.Transaction
            ? await internal.tryProcessLegacyTransaction(serialized)
            : await internal.tryProcessVersionedTransaction(serialized);
        return new BanksTransactionResultWithMeta(inner);
    }
    /**
     * Simulate a transaction at the given commitment level.
     * @param tx - The transaction to simulate.
     * @param commitment - The commitment to use.
     * @returns The transaction simulation result.
     */
    async simulateTransaction(tx, commitment) {
        const internal = this.inner;
        const serialized = tx.serialize();
        const commitmentConverted = convertCommitment(commitment);
        const inner = tx instanceof web3_js_1.Transaction
            ? await internal.simulateLegacyTransaction(serialized, commitmentConverted)
            : await internal.simulateVersionedTransaction(serialized, commitmentConverted);
        return new BanksTransactionResultWithMeta(inner);
    }
    /**
     * Return the status of a transaction with a signature matching the transaction's first signature.
     *
     * Return None if the transaction is not found, which may be because the
     * blockhash was expired or the fee-paying account had insufficient funds to pay the
     * transaction fee. Note that servers rarely store the full transaction history. This
     * method may return None if the transaction status has been discarded.
     *
     * @param signature - The transaction signature (the first signature of the transaction).
     * @returns The transaction status, if found.
     */
    async getTransactionStatus(signature) {
        const decodedSig = bs58_1.default.decode(signature);
        return await this.inner.getTransactionStatus(decodedSig);
    }
    /**
     * Same as `getTransactionStatus`, but for multiple transactions.
     * @param signatures - The transaction signatures.
     * @returns The transaction statuses, if found.
     */
    async getTransactionStatuses(signatures) {
        const decoded = signatures.map(bs58_1.default.decode);
        return await this.inner.getTransactionStatuses(decoded);
    }
    /**
     * Get the slot that has reached the given commitment level (or the default commitment).
     * @param commitment - The commitment to use.
     * @returns The current slot.
     */
    async getSlot(commitment) {
        return await this.inner.getSlot(convertCommitment(commitment));
    }
    /**
     * Get the current block height.
     * @param commitment - The commitment to use.
     * @returns The current block height.
     */
    async getBlockHeight(commitment) {
        return await this.inner.getBlockHeight(convertCommitment(commitment));
    }
    /**
     * Get the cluster rent.
     * @returns The rent object.
     */
    async getRent() {
        return await this.inner.getRent();
    }
    /**
     * Get the cluster clock.
     * @returns the clock object.
     */
    async getClock() {
        return await this.inner.getClock();
    }
    /**
     * Return the balance in lamports of an account at the given address at the slot.
     * @param address - The account to look up.
     * @param commitment - The commitment to use.
     * @returns The account balance in lamports.
     */
    async getBalance(address, commitment) {
        return await this.inner.getBalance(address.toBytes(), convertCommitment(commitment));
    }
    /**
     * Returns latest blockhash and last valid block height for given commitment level.
     * @param commitment - The commitment to use.
     * @returns The blockhash and last valid block height.
     */
    async getLatestBlockhash(commitment) {
        const inner = await this.inner.getLatestBlockhash(convertCommitment(commitment));
        if (!inner)
            return null;
        return [inner.blockhash, inner.lastValidBlockHeight];
    }
    /**
     * Get the fee in lamports for a given message.
     * @param msg - The message to check.
     * @param commitment - The commitment to use.
     * @returns The fee for the given message.
     */
    async getFeeForMessage(msg, commitment) {
        return await this.inner.getFeeForMessage(msg.serialize(), convertCommitment(commitment));
    }
}
exports.BanksClient = BanksClient;
/**
 * The result of calling `start()`.
 *
 * Contains a BanksClient, a recent blockhash and a funded payer keypair.
 */
class ProgramTestContext {
    constructor(inner) {
        this.inner = inner;
    }
    /** The client for this test. */
    get banksClient() {
        return new BanksClient(this.inner.banksClient);
    }
    /** A funded keypair for sending transactions. */
    get payer() {
        return web3_js_1.Keypair.fromSecretKey(this.inner.payer);
    }
    /** The last blockhash registered when the client was initialized. */
    get lastBlockhash() {
        return this.inner.lastBlockhash;
    }
    /** The chain's genesis config. */
    get genesisConfig() {
        return new GenesisConfig(this.inner.genesisConfig);
    }
    /**
     * Create or overwrite an account, subverting normal runtime checks.
     *
     * This method exists to make it easier to set up artificial situations
     * that would be difficult to replicate by sending individual transactions.
     * Beware that it can be used to create states that would not be reachable
     * by sending transactions!
     *
     * @param address - The address to write to.
     * @param account - The account object to write.
     */
    setAccount(address, account) {
        this.inner.setAccount(address.toBytes(), fromAccountInfo(account));
    }
    /**
     * Overwrite the clock sysvar.
     * @param clock - The new clock object.
     */
    setClock(clock) {
        this.inner.setClock(clock);
    }
    /**
     * Overwrite the rent sysvar.
     * @param rent - The new rent object.
     */
    setRent(rent) {
        this.inner.setRent(rent);
    }
    /**
     * Force the working bank ahead to a new slot.
     * @param slot - The slot to warp to.
     */
    warpToSlot(slot) {
        this.inner.warpToSlot(slot);
    }
    /**
     * Force the working bank ahead to a new epoch.
     * @param epoch - The epoch to warp to.
     */
    warpToEpoch(epoch) {
        this.inner.warpToEpoch(epoch);
    }
}
exports.ProgramTestContext = ProgramTestContext;
class Banksy {
    constructor() {
        this.inner = new internal_1.Banksy();
    }
    async start(computeUnitsMax, transactionAccountLockLimit) {
        let ctx = await this.inner.start(computeUnitsMax, transactionAccountLockLimit);
        return new ProgramTestContext(ctx);
    }
    async getSingleton(computeUnitsMax, transactionAccountLockLimit) {
        if (!Banksy._singleton)
            Banksy._singleton = await this.start(computeUnitsMax, transactionAccountLockLimit);
        return Banksy._singleton;
    }
    addProgram(name, programId) {
        this.inner.addProgram(name, programId.toBytes(), web3_js_1.Keypair.generate().publicKey.toBytes());
    }
    addUpgradableProgram(name, programId, programAuthority, programData) {
        this.inner.addUpgradableProgram(name, programId.toBytes(), programAuthority.toBytes(), programData.toBytes());
    }
    addAccount(address, account) {
        this.inner.addAccount(address.toBytes(), fromAccountInfo(account));
    }
    addAccountWithLamports(address, owner, lamports) {
        this.inner.addAccountWithLamports(address.toBytes(), owner.toBytes(), lamports);
    }
    airdrop(address, lamports) {
        this.inner.airdrop(address.toBytes(), lamports);
    }
    addTokenMint(address, supply, decimals, mintAuthority, freezeAuthority) {
        this.inner.addTokenMint(address.toBytes(), supply, decimals, try_convert_from_pubkey(mintAuthority), try_convert_from_pubkey(freezeAuthority));
    }
    addTokenAccount(address, mint, owner, amount, delegatedAmount, delegate, isNative, closeAuthority) {
        this.inner.addTokenAccount(address.toBytes(), mint.toBytes(), owner.toBytes(), amount, delegatedAmount, try_convert_from_pubkey(delegate), isNative, try_convert_from_pubkey(closeAuthority));
    }
}
exports.Banksy = Banksy;
function try_convert_from_pubkey(key) {
    if (key) {
        return key.toBytes();
    }
    return;
}
/**
 * Start a bankrun!
 *
 * This will spin up a BanksServer and a BanksClient,
 * deploy programs and add accounts as instructed.
 *
 * @param programs - An array of objects indicating which programs to deploy to the test environment. See the main bankrun docs for more explanation on how to add programs.
 * @param accounts - An array of objects indicating what data to write to the given addresses.
 * @param computeMaxUnits - Override the default compute unit limit for a transaction.
 * @param transactionAccountLockLimit - Override the default transaction account lock limit.
 * @returns A container for stuff you'll need to send transactions and interact with the test environment.
 */
// export async function start(
// 	programs: AddedProgram[],
// 	accounts: AddedAccount[],
// 	computeMaxUnits?: bigint,
// 	transactionAccountLockLimit?: bigint,
// ): Promise<ProgramTestContext> {
// 	const ctx = await startInner(
// 		programs.map((p) => [
// 			p.name,
// 			p.programId.toBytes(),
// 		]),
// 		accounts.map((a) => [a.address.toBytes(), fromAccountInfo(a.info)]),
// 		computeMaxUnits,
// 		transactionAccountLockLimit,
// 	);
// 	return new ProgramTestContext(ctx);
// }
// /**
//  * Start a bankrun in an Anchor workspace, with all the workspace programs deployed.
//  *
//  * This will spin up a BanksServer and a BanksClient,
//  * deploy programs and add accounts as instructed.
//  *
//  * @param path - Path to root of the Anchor project.
//  * @param programs - An array of objects indicating extra programs to deploy to the test environment. See the main bankrun docs for more explanation on how to add programs.
//  * @param accounts - An array of objects indicating what data to write to the given addresses.
//  * @param computeMaxUnits - Override the default compute unit limit for a transaction.
//  * @param transactionAccountLockLimit - Override the default transaction account lock limit.
//  * @returns A container for stuff you'll need to send transactions and interact with the test environment.
//  */
// export async function startAnchor(
// 	path: string,
// 	extraPrograms: AddedProgram[],
// 	accounts: AddedAccount[],
// 	computeMaxUnits?: bigint,
// 	transactionAccountLockLimit?: bigint,
// ): Promise<ProgramTestContext> {
// 	const ctx = await startAnchorInner(
// 		path,
// 		extraPrograms.map((p) => [
// 			p.name,
// 			p.programId.toBytes(),
// 			fromOptionalPubkey(p.programAuthority),
// 			fromOptionalPubkey(p.programData),
// 		]),
// 		accounts.map((a) => [a.address.toBytes(), fromAccountInfo(a.info)]),
// 		computeMaxUnits,
// 		transactionAccountLockLimit,
// 	);
// 	return new ProgramTestContext(ctx);
// }
//# sourceMappingURL=index.js.map