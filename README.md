# Starknet NFT Explorer Substreams
1. **Build Substreams**:
   - `substreams build`

2. **Build Subgraph**:
   - `npm install`
   - `graph codegen`
   - `graph build`

3. **Deploy**:
   - `graph deploy --node <node-url> --ipfs <ipfs-url> your-org/nft-explorer-starknet`

4. **Query**:
   - ERC721 Balances:
     ```graphql
     query {
       erc721Balances(first: 5) {
         id
         collection { id, type }
         owner { id }
         tokenIds
         transactionHash
       }
     }
     ```
   - ERC1155 Balances:
     ```graphql
     query {
       erc1155Balances(first: 5) {
         id
         collection { id, type }
         owner { id }
         tokenId
         balance
         transactionHash
       }
     }
     ```
   - Collections:
     ```graphql
     query {
       collections(first: 5) {
         id
         type
         erc721Balances { id }
         erc1155Balances { id }
       }
     }
     ```

5. **Verify**:
   - Ensure `Collection` entities appear for any contract emitting `Transfer`/`TransferSingle`/`TransferBatch`.
   - Check `ERC721Balance.tokenIds` contains correct token IDs.
   - Confirm `ERC1155Balance.balance` updates correctly and removes when `0`.

### Frontend Usage

The Subgraph allows querying all NFT collections and balances on Starknet. Example Apollo query:

```javascript
import { ApolloClient, InMemoryCache, gql } from '@apollo/client';

const client = new ApolloClient({
  uri: 'https://api.thegraph.com/subgraphs/name/your-org/nft-explorer-starknet',
  cache: new InMemoryCache(),
});

const QUERY = gql`
  query Owner($id: ID!) {
    owner(id: $id) {
      id
      erc721Balances {
        collection { id, type }
        tokenIds
      }
      erc1155Balances {
        collection { id, type }
        tokenId
        balance
      }
    }
  }
`;

client.query({
  query: QUERY,
  variables: { id: "0x1234..." },
}).then(result => {
  console.log(result.data.owner);
});
```# starknet-nft-explorer-subgraph
