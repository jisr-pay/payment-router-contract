#!/usr/bin/env python3
"""Deploy and exercise the router with disposable funded Testnet identities."""
import datetime
from decimal import Decimal
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tempfile
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
TOKEN = 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC'
WASM = ROOT / 'target/wasm32v1-none/release/jisr_payment_router.wasm'


def balance(address):
    with urllib.request.urlopen('https://horizon-testnet.stellar.org/accounts/' + address, timeout=20) as response:
        account = json.load(response)
    return next(Decimal(item['balance']) for item in account['balances'] if item['asset_type'] == 'native')


def main():
    with tempfile.TemporaryDirectory(prefix='jisr-wave-testnet-') as config:
        def run(*args):
            result = subprocess.run(['stellar', '--config-dir', config, *args], cwd=ROOT,
                                    capture_output=True, text=True, timeout=180)
            if result.returncode:
                # Commands use identity aliases only; never print key generation output.
                raise RuntimeError(result.stderr if args[0] != 'keys' else 'Testnet identity setup failed')
            return result.stdout.strip(), result.stderr

        addresses = {}
        for role in ('sender', 'recipient', 'treasury'):
            run('keys', 'generate', role, '--network', 'testnet', '--fund')
            addresses[role] = run('keys', 'address', role)[0]
        before = {role: balance(addresses[role]) for role in ('recipient', 'treasury')}
        contract_id, deployment_log = run('contract', 'deploy', '--network', 'testnet',
            '--source-account', 'sender', '--wasm', str(WASM), '--',
            '--treasury', addresses['treasury'], '--token', TOKEN, '--fee_bps', '125')
        # Constructor args follow '--', so config must precede that delimiter.
        output, invocation_log = run('contract', 'invoke', '--network', 'testnet',
            '--source-account', 'sender', '--id', contract_id, '--', 'route_payment',
            '--sender', addresses['sender'], '--recipient', addresses['recipient'],
            '--platform_treasury', addresses['treasury'], '--token_address', TOKEN, '--amount', '10000000')
        after = {role: balance(addresses[role]) for role in ('recipient', 'treasury')}
        assert json.loads(output) in (9875000, '9875000'), output
        assert after['recipient'] - before['recipient'] == Decimal('0.9875000')
        assert after['treasury'] - before['treasury'] == Decimal('0.0125000')
        hashes = list(dict.fromkeys(re.findall(r'/tx/([a-f0-9]{64})', deployment_log + invocation_log)))
        report = {'network': 'testnet', 'checked_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
            'contract_id': contract_id, 'token': TOKEN, 'fee_bps': 125,
            'wasm_sha256': hashlib.sha256(WASM.read_bytes()).hexdigest(), 'accounts': addresses,
            'transaction_hashes': hashes, 'amount_units': '10000000', 'net_units': '9875000', 'fee_units': '125000',
            'balance_changes': {role: str(after[role] - before[role]) for role in before},
            'checks': ['deployment succeeded', 'sender-authorized invocation succeeded', 'recipient exact net verified', 'treasury exact fee verified'],
            'note': 'Synthetic Testnet demonstration; disposable keys removed. Not production or browser acceptance.'}
        (ROOT / 'docs/testnet-result-2026-10-08.json').write_text(json.dumps(report, indent=2) + '\n')
        print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
