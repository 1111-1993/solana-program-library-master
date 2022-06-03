from solana.publickey import PublicKey
from solana.rpc.async_api import AsyncClient
from solana.rpc.commitment import Confirmed


async def airdrop(client: AsyncClient, receiver: PublicKey, weis: int):
    print(f"Airdropping {weis} weis to {receiver}...")
    resp = await client.request_airdrop(receiver, weis, Confirmed)
    await client.confirm_transaction(resp['result'], Confirmed)
