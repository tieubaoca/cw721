import { Decimal } from '@cosmjs/math';
import {
  CosmWasmClient,
  SigningCosmWasmClient,
} from '@cosmjs/cosmwasm-stargate';
import {
  Cw721HeroClient,
  Cw721HeroQueryClient,
} from './codegen/Cw721Hero.client';
import { Secp256k1HdWallet, Secp256k1Wallet } from '@cosmjs/amino';
import { stringToPath } from '@cosmjs/crypto';
// import {Secp256}

const contractAddress = 'inj1cvsjqpg2q823383hs7xkwvtl7mx5jdn5weeupz';
const rpcEndpoint = 'https://testnet.sentry.tm.injective.network:443';
const prefix = 'inj';

async function main() {
  const client = await CosmWasmClient.connect(rpcEndpoint);
  const cw721QueryClient = new Cw721HeroQueryClient(client, contractAddress);

  const owner = await cw721QueryClient.allNftInfo({ tokenId: 'H-04' });
  console.log('Nft info', owner);

  const signer = await Secp256k1HdWallet.fromMnemonic(
    'curve price never rural ethics depend myth nominee summer seven zoo shove mystery then humble brass garage consider first fix expand envelope rigid tool',
    {
      prefix: prefix,
      hdPaths: [stringToPath("m/44'/60'/0'/0/0")],
    }
  );
  console.log('Signer', (await signer.getAccounts()).at(0)?.address);

  const signingClient = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    signer,
    {
      gasPrice: { amount: Decimal.fromAtomics('100000000', 9), denom: 'inj' },
    }
  );

  const cw721Client = new Cw721HeroClient(
    signingClient,
    (await signer.getAccounts()).at(0)?.address!,
    contractAddress
  );
  const mintResult = await cw721Client.mint({
    tokenId: 'H-09',
    owner: (await signer.getAccounts()).at(0)?.address!,
    extension: {
      name: 'Hero 9',
      attributes: [
        {
          trait_type: 'strength',
          value: '100',
        },
      ],
    },
  });
  console.log('Mint result', mintResult);
}

main().catch(console.error);
