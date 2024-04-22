import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { Cw721HeroQueryClient } from './codegen/Cw721Hero.client';
import { Network, getNetworkEndpoints } from '@injectivelabs/networks';
import { config } from 'dotenv';

config();

const contractAddress = 'inj1cvsjqpg2q823383hs7xkwvtl7mx5jdn5weeupz';

async function main() {
  const endpoints = getNetworkEndpoints(Network.Testnet);
  const rpcEndpoint = endpoints.rpc as string;

  const client = await CosmWasmClient.connect(rpcEndpoint);

  const owner = 'inj1wd6xqdgvv6nm94r5a3xu7y7tqssp0652r8su0k';

  const res = await client.queryContractSmart(contractAddress, {
    tokens: {
      owner: owner,
      limit: 10,
    },
  });
  console.log(`Query nft by owner response: ${JSON.stringify(res)}`);
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
