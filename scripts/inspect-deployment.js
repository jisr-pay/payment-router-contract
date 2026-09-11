import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { rpc, StrKey, Networks, contract } from '@stellar/stellar-sdk';

const root = new URL('../', import.meta.url);
const reference = JSON.parse(await readFile(new URL('deployment-reference.json', root), 'utf8'));
if (reference.networkPassphrase !== Networks.TESTNET || reference.network !== 'TESTNET' ||
    !StrKey.isValidContract(reference.contractId)) throw new Error('Invalid Testnet deployment reference.');
const server = new rpc.Server(reference.rpcUrl, { timeout: 15_000 });
const network = await server.getNetwork();
if (network.passphrase !== Networks.TESTNET) throw new Error('RPC is not Stellar Testnet.');
// Read-only: fetches existing ledger WASM; never signs, deploys or submits.
const wasm = await server.getContractWasmByContractId(reference.contractId);
const module = new WebAssembly.Module(wasm);
const report = {
  checkedAt: new Date().toISOString(), network: reference.network,
  rpcUrl: reference.rpcUrl, contractId: reference.contractId,
  wasmSha256: createHash('sha256').update(wasm).digest('hex'), wasmBytes: wasm.length,
  exports: WebAssembly.Module.exports(module).map(item => item.name),
  contractSpecSectionCount: WebAssembly.Module.customSections(module, 'contractspecv0').length,
  functions: contract.Spec.fromWasm(wasm).funcs().map(fn => ({
    name: fn.name().toString(),
    inputs: fn.inputs().map(input => ({ name: input.name().toString(), type: input.type().switch().name })),
    outputs: fn.outputs().map(output => output.switch().name),
  })),
  sourceVerified: false,
  note: 'Fetched ledger bytecode is not original Rust source and does not establish fee or authorization behavior.',
};
const output = new URL('artifacts/', root);
await mkdir(output, { recursive: true });
await writeFile(new URL('deployed.wasm', output), wasm);
await writeFile(new URL('inspection.json', output), `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify(report, null, 2));
