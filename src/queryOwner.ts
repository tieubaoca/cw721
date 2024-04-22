import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { Cw721HeroQueryClient } from './codegen/Cw721Hero.client';
import { Network, getNetworkEndpoints } from '@injectivelabs/networks';
import { config } from 'dotenv';

config();

const contractAddress = 'inj1cvsjqpg2q823383hs7xkwvtl7mx5jdn5weeupz';
// const rpcEndpoint = 'https://testnet.sentry.tm.injective.network:443';
const prefix = 'inj';

async function main() {
  const endpoints = getNetworkEndpoints(Network.Testnet);
  const rpcEndpoint = endpoints.rpc as string;

  const client = await CosmWasmClient.connect(rpcEndpoint);
  const cw721QueryClient = new Cw721HeroQueryClient(client, contractAddress);

  const owner = 'inj1wd6xqdgvv6nm94r5a3xu7y7tqssp0652r8su0k';

  const nftId = 'H-01';

  const res = await client.queryContractSmart(contractAddress, {
    owner_of: {
      token_id: nftId,
    },
  });
  console.log(`Query owner by cosmos client: ${JSON.stringify(res)}`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
