/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */
/* auto-generated by NAPI-RS */
const { existsSync, readFileSync } = require('fs');
const { join } = require('path');
const { platform, arch } = process;
let nativeBinding = null;
let localFileExisted = false;
let loadError = null;
function isMusl() {
    // For Node 10
    if (!process.report || typeof process.report.getReport !== 'function') {
        try {
            const lddPath = require('child_process').execSync('which ldd').toString().trim();
            return readFileSync(lddPath, 'utf8').includes('musl');
        }
        catch (e) {
            return true;
        }
    }
    else {
        const { glibcVersionRuntime } = process.report.getReport().header;
        return !glibcVersionRuntime;
    }
}
switch (platform) {
    case 'android':
        switch (arch) {
            case 'arm64':
                localFileExisted = existsSync(join(__dirname, 'banksy.android-arm64.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.android-arm64.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-android-arm64');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            case 'arm':
                localFileExisted = existsSync(join(__dirname, 'banksy.android-arm-eabi.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.android-arm-eabi.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-android-arm-eabi');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            default:
                throw new Error(`Unsupported architecture on Android ${arch}`);
        }
        break;
    case 'win32':
        switch (arch) {
            case 'x64':
                localFileExisted = existsSync(join(__dirname, 'banksy.win32-x64-msvc.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.win32-x64-msvc.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-win32-x64-msvc');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            case 'ia32':
                localFileExisted = existsSync(join(__dirname, 'banksy.win32-ia32-msvc.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.win32-ia32-msvc.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-win32-ia32-msvc');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            case 'arm64':
                localFileExisted = existsSync(join(__dirname, 'banksy.win32-arm64-msvc.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.win32-arm64-msvc.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-win32-arm64-msvc');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            default:
                throw new Error(`Unsupported architecture on Windows: ${arch}`);
        }
        break;
    case 'darwin':
        localFileExisted = existsSync(join(__dirname, 'banksy.darwin-universal.node'));
        try {
            if (localFileExisted) {
                nativeBinding = require('./banksy.darwin-universal.node');
            }
            else {
                nativeBinding = require('@hylo/banksy-darwin-universal');
            }
            break;
        }
        catch { }
        switch (arch) {
            case 'x64':
                localFileExisted = existsSync(join(__dirname, 'banksy.darwin-x64.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.darwin-x64.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-darwin-x64');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            case 'arm64':
                localFileExisted = existsSync(join(__dirname, 'banksy.darwin-arm64.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.darwin-arm64.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-darwin-arm64');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            default:
                throw new Error(`Unsupported architecture on macOS: ${arch}`);
        }
        break;
    case 'freebsd':
        if (arch !== 'x64') {
            throw new Error(`Unsupported architecture on FreeBSD: ${arch}`);
        }
        localFileExisted = existsSync(join(__dirname, 'banksy.freebsd-x64.node'));
        try {
            if (localFileExisted) {
                nativeBinding = require('./banksy.freebsd-x64.node');
            }
            else {
                nativeBinding = require('@hylo/banksy-freebsd-x64');
            }
        }
        catch (e) {
            loadError = e;
        }
        break;
    case 'linux':
        switch (arch) {
            case 'x64':
                if (isMusl()) {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-x64-musl.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-x64-musl.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-x64-musl');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                else {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-x64-gnu.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-x64-gnu.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-x64-gnu');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                break;
            case 'arm64':
                if (isMusl()) {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-arm64-musl.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-arm64-musl.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-arm64-musl');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                else {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-arm64-gnu.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-arm64-gnu.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-arm64-gnu');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                break;
            case 'arm':
                if (isMusl()) {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-arm-musleabihf.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-arm-musleabihf.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-arm-musleabihf');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                else {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-arm-gnueabihf.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-arm-gnueabihf.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-arm-gnueabihf');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                break;
            case 'riscv64':
                if (isMusl()) {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-riscv64-musl.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-riscv64-musl.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-riscv64-musl');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                else {
                    localFileExisted = existsSync(join(__dirname, 'banksy.linux-riscv64-gnu.node'));
                    try {
                        if (localFileExisted) {
                            nativeBinding = require('./banksy.linux-riscv64-gnu.node');
                        }
                        else {
                            nativeBinding = require('@hylo/banksy-linux-riscv64-gnu');
                        }
                    }
                    catch (e) {
                        loadError = e;
                    }
                }
                break;
            case 's390x':
                localFileExisted = existsSync(join(__dirname, 'banksy.linux-s390x-gnu.node'));
                try {
                    if (localFileExisted) {
                        nativeBinding = require('./banksy.linux-s390x-gnu.node');
                    }
                    else {
                        nativeBinding = require('@hylo/banksy-linux-s390x-gnu');
                    }
                }
                catch (e) {
                    loadError = e;
                }
                break;
            default:
                throw new Error(`Unsupported architecture on Linux: ${arch}`);
        }
        break;
    default:
        throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`);
}
if (!nativeBinding) {
    if (loadError) {
        throw loadError;
    }
    throw new Error(`Failed to load native binding`);
}
const { Account, BanksClient, Clock, CommitmentLevel, EpochSchedule, FeeRateGovernor, GenesisConfig, Inflation, PohConfig, ProgramTestContext, Rent, TransactionReturnData, BanksTransactionMeta, TransactionStatus, BanksTransactionResultWithMeta, BlockhashRes, AddressAndAccount, NativeInstructionProcessor, Banksy } = nativeBinding;
module.exports.Account = Account;
module.exports.BanksClient = BanksClient;
module.exports.Clock = Clock;
module.exports.CommitmentLevel = CommitmentLevel;
module.exports.EpochSchedule = EpochSchedule;
module.exports.FeeRateGovernor = FeeRateGovernor;
module.exports.GenesisConfig = GenesisConfig;
module.exports.Inflation = Inflation;
module.exports.PohConfig = PohConfig;
module.exports.ProgramTestContext = ProgramTestContext;
module.exports.Rent = Rent;
module.exports.TransactionReturnData = TransactionReturnData;
module.exports.BanksTransactionMeta = BanksTransactionMeta;
module.exports.TransactionStatus = TransactionStatus;
module.exports.BanksTransactionResultWithMeta = BanksTransactionResultWithMeta;
module.exports.BlockhashRes = BlockhashRes;
module.exports.AddressAndAccount = AddressAndAccount;
module.exports.NativeInstructionProcessor = NativeInstructionProcessor;
module.exports.Banksy = Banksy;
//# sourceMappingURL=internal.js.map